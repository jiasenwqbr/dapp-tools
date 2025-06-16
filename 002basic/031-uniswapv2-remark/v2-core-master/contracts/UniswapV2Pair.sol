pragma solidity =0.5.16;

import './interfaces/IUniswapV2Pair.sol';
import './UniswapV2ERC20.sol';
import './libraries/Math.sol';
import './libraries/UQ112x112.sol';
import './interfaces/IERC20.sol';
import './interfaces/IUniswapV2Factory.sol';
import './interfaces/IUniswapV2Callee.sol';
/**
 * 继承与库
 *  接口和 ERC20
 *      IUniswapV2Pair：定义了外部可调用的函数签名。
 *      UniswapV2ERC20：实现了 LP Token（流动性凭证）的 ERC20 逻辑，包括 permit、_mint、_burn 等。
 */
contract UniswapV2Pair is IUniswapV2Pair, UniswapV2ERC20 {
    // SafeMath 防止整数溢出。
    using SafeMath  for uint;
    // Q112x112 支持 112.112 定点数，用于价格累积。
    using UQ112x112 for uint224;
    // MINIMUM_LIQUIDITY 部署第一笔流动性时，锁定 1000 LP Token，防止除以零。
    uint public constant MINIMUM_LIQUIDITY = 10**3;
    // SELECTOR 用于 _safeTransfer 中的低级调用。
    bytes4 private constant SELECTOR = bytes4(keccak256(bytes('transfer(address,uint256)')));

    // factory：部署本对合约的工厂地址。
    address public factory;
    // token0/token1：两种交易资产（地址已按大小排序）。
    address public token0;
    address public token1;

    // reserve0/reserve1：Pair 当前在合约里的币量快照。
    uint112 private reserve0;           // uses single storage slot, accessible via getReserves
    uint112 private reserve1;           // uses single storage slot, accessible via getReserves
    // blockTimestampLast：上次更新储备量时的区块时间戳（取低 32 位）。
    uint32  private blockTimestampLast; // uses single storage slot, accessible via getReserves
    // price0CumulativeLast/price1CumulativeLast：Uniswap v2 的 on‑chain 时间加权平均价格（TWAP）累积值。
    uint public price0CumulativeLast;
    uint public price1CumulativeLast;
    // kLast：上一次 mint 或 burn 后的 reserve0 * reserve1，用于手续费开关逻辑。
    uint public kLast; // reserve0 * reserve1, as of immediately after the most recent liquidity event

    uint private unlocked = 1;
    // 重入锁
    // 给所有修改状态的核心函数（mint/burn/swap/skim/sync）加 lock，防止重入攻击。
    modifier lock() {
        require(unlocked == 1, 'UniswapV2: LOCKED');
        unlocked = 0;
        _;
        unlocked = 1;
    }

    // 读取储备 任何人都能读取当前储备量及上次更新时间。
    function getReserves() public view returns (uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast) {
        _reserve0 = reserve0;
        _reserve1 = reserve1;
        _blockTimestampLast = blockTimestampLast;
    }

    // 安全转账 用低级 call 而不 transfer/approve，兼容各种 ERC20 实现。
    function _safeTransfer(address token, address to, uint value) private {
        (bool success, bytes memory data) = token.call(abi.encodeWithSelector(SELECTOR, to, value));
        require(success && (data.length == 0 || abi.decode(data, (bool))), 'UniswapV2: TRANSFER_FAILED');
    }

    event Mint(address indexed sender, uint amount0, uint amount1);
    event Burn(address indexed sender, uint amount0, uint amount1, address indexed to);
    event Swap(
        address indexed sender,
        uint amount0In,
        uint amount1In,
        uint amount0Out,
        uint amount1Out,
        address indexed to
    );
    event Sync(uint112 reserve0, uint112 reserve1);
    // 构造与初始化
    // 部署：只有 Factory 能部署后调用 initialize，并将 token0/token1 赋值。
    constructor() public {
        factory = msg.sender;
    }

    // called once by the factory at time of deployment
    function initialize(address _token0, address _token1) external {
        require(msg.sender == factory, 'UniswapV2: FORBIDDEN'); // sufficient check
        token0 = _token0;
        token1 = _token1;
    }

    // 储备更新与价格累积
    //  同步储备：更新内存储备量并发出 Sync 事件。
    //  价格累计：每次跨区块调用才累积 reserve1/reserve0 与 reserve0/reserve1，供 TWAP 使用。
    // update reserves and, on the first call per block, price accumulators
    function _update(uint balance0, uint balance1, uint112 _reserve0, uint112 _reserve1) private {
        //确认余额0和余额1小于等于最大的uint112
        require(
            balance0 <= uint112(-1) && balance1 <= uint112(-1),
            "UniswapV2: OVERFLOW"
        );
        //区块时间戳,将时间戳转换为uint32
        //solium-disable-next-line
        uint32 blockTimestamp = uint32(block.timestamp % 2**32);
        //计算时间流逝
        uint32 timeElapsed = blockTimestamp - blockTimestampLast; // overflow is desired
        //如果时间流逝>0 并且 储备量0,1不等于0
        if (timeElapsed > 0 && _reserve0 != 0 && _reserve1 != 0) {
            // * never overflows, and + overflow is desired
            //价格0最后累计 += 储备量1 * 2**112 / 储备量0 * 时间流逝
            //solium-disable-next-line
            price0CumulativeLast +=
                uint256(UQ112x112.encode(_reserve1).uqdiv(_reserve0)) *
                timeElapsed;
            //价格1最后累计 += 储备量0 * 2**112 / 储备量1 * 时间流逝
            //solium-disable-next-line
            price1CumulativeLast +=
                uint256(UQ112x112.encode(_reserve0).uqdiv(_reserve1)) *
                timeElapsed;
        }
        //余额0,1放入储备量0,1
        reserve0 = uint112(balance0);
        reserve1 = uint112(balance1);
        //更新最后时间戳
        blockTimestampLast = blockTimestamp;
        //触发同步事件
        emit Sync(reserve0, reserve1);
    }

    // 手续费开关逻辑
    // 逻辑：如果 Factory 设置了 feeTo，则在 mint/burn 时给 feeTo 铸造额外 LP Token，等同于 0.05% 交易费。
    // if fee is on, mint liquidity equivalent to 1/6th of the growth in sqrt(k)
    function _mintFee(uint112 _reserve0, uint112 _reserve1) private returns (bool feeOn) {
        address feeTo = IUniswapV2Factory(factory).feeTo();
        feeOn = feeTo != address(0);
        uint _kLast = kLast; // gas savings
        if (feeOn) {
            if (_kLast != 0) {
                uint rootK = Math.sqrt(uint(_reserve0).mul(_reserve1));
                uint rootKLast = Math.sqrt(_kLast);
                if (rootK > rootKLast) {
                    uint numerator = totalSupply.mul(rootK.sub(rootKLast));
                    uint denominator = rootK.mul(5).add(rootKLast);
                    uint liquidity = numerator / denominator;
                    if (liquidity > 0) _mint(feeTo, liquidity);
                }
            }
        } else if (_kLast != 0) {
            kLast = 0;
        }
    }
    // 增加流动性（mint）
    // 首次提供：按 √(amount0*amount1) 计算 LP；锁定 MINIMUM_LIQUIDITY。
    // 后续提供：按最小比例 (amount0/reserve0, amount1/reserve1) 计算 LP。
    // 更新储备与手续费状态。
    // this low-level function should be called from a contract which performs important safety checks
    function mint(address to) external lock returns (uint liquidity) {
        // //获取`储备量0`,`储备量1`
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        // 获取当前合约在token0合约内的余额
        uint balance0 = IERC20(token0).balanceOf(address(this));
        // 获取当前合约在token1合约内的余额
        uint balance1 = IERC20(token1).balanceOf(address(this));
        // amount0 = 余额0 - 储备0
        uint amount0 = balance0.sub(_reserve0);
        // amount1 = 余额1 - 储备1
        uint amount1 = balance1.sub(_reserve1);

        // 返回铸造费开关
        bool feeOn = _mintFee(_reserve0, _reserve1);
        // 获取totalSupply,必须在此处定义，因为totalSupply可以在mintFee中更新
        uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        if (_totalSupply == 0) {
            // 流动性 = (数量0 * 数量1)的平方根 - 最小流动性1000
            liquidity = Math.sqrt(amount0.mul(amount1)).sub(MINIMUM_LIQUIDITY);
            // 在总量为0的初始状态,永久锁定最低流动性
           _mint(address(0), MINIMUM_LIQUIDITY); // permanently lock the first MINIMUM_LIQUIDITY tokens
        } else {
            // 流动性 = 最小值 (amount0 * _totalSupply / _reserve0) 和 (amount1 * _totalSupply / _reserve1)
            liquidity = Math.min(amount0.mul(_totalSupply) / _reserve0, amount1.mul(_totalSupply) / _reserve1);
        }
        // //确认流动性 > 0
        require(liquidity > 0, 'UniswapV2: INSUFFICIENT_LIQUIDITY_MINTED');
        //铸造流动性给to地址
        _mint(to, liquidity);
        // 更新储备量
        _update(balance0, balance1, _reserve0, _reserve1);
       //如果铸造费开关为true, k值 = 储备0 * 储备1
        if (feeOn) kLast = uint256(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        //触发铸造事件
        emit Mint(msg.sender, amount0, amount1);
    }


    // 移除流动性（burn）
    // LP Token 换回两侧资产，按持仓比例“拉出”资产。
    // this low-level function should be called from a contract which performs important safety checks
    function burn(address to) external lock returns (uint amount0, uint amount1) {
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        address _token0 = token0;                                // gas savings
        address _token1 = token1;                                // gas savings
        uint balance0 = IERC20(_token0).balanceOf(address(this));
        uint balance1 = IERC20(_token1).balanceOf(address(this));
        uint liquidity = balanceOf[address(this)];

        bool feeOn = _mintFee(_reserve0, _reserve1);
        uint _totalSupply = totalSupply; // gas savings, must be defined here since totalSupply can update in _mintFee
        amount0 = liquidity.mul(balance0) / _totalSupply; // using balances ensures pro-rata distribution
        amount1 = liquidity.mul(balance1) / _totalSupply; // using balances ensures pro-rata distribution
        require(amount0 > 0 && amount1 > 0, 'UniswapV2: INSUFFICIENT_LIQUIDITY_BURNED');
        _burn(address(this), liquidity);
        _safeTransfer(_token0, to, amount0);
        _safeTransfer(_token1, to, amount1);
        balance0 = IERC20(_token0).balanceOf(address(this));
        balance1 = IERC20(_token1).balanceOf(address(this));

        _update(balance0, balance1, _reserve0, _reserve1);
        if (feeOn) kLast = uint(reserve0).mul(reserve1); // reserve0 and reserve1 are up-to-date
        emit Burn(msg.sender, amount0, amount1, to);
    }

    // 交易（swap）
    // 乐观转账：先把输出资产给到 to，再验证输入量。
    // 手续费：0.3%，通过 balance.mul(1000).sub(amountIn.mul(3)) 体现。
    // 乘积不减：保证 x*y ≥ k，维护恒定乘积。
    // this low-level function should be called from a contract which performs important safety checks
    
    /**
     * 
     *
     * 整体流程总结
        1.输入检查：输出量、流动性足够且接收地址合法。
        2.乐观转账：先给用户输出资产，并可回调执行闪兑逻辑。
        3.输入量计算：比较转账前后余额，确定用户到底输入了多少资产。
        4.手续费与恒定乘积校验：扣除手续费后，验证 x * y ≥ k。
        5.状态同步与事件：更新储备快照与价格累积，发出 Swap 事件。
        如此设计，使 Uniswap V2 能在一次原子交易（atomic transaction）中完成资产交换，并保证 LP 们的价值不被套利者无费侵蚀。
    */
    /**
     * 
     * @param amount0Out 期望从池子中取出的两种代币数量，至少一项要大于 0。
     * @param amount1Out 期望从池子中取出的两种代币数量，至少一项要大于 0。
     * @param to 接收代币的地址。
     * @param data 可选参数，如果非空，则触发闪兑回调 IUniswapV2Callee。
     * lock：重入锁，防止在同一次调用中再次进入 swap。
     */
    function swap(uint amount0Out, uint amount1Out, address to, bytes calldata data) external lock {
        // 至少要换出一种代币，否则没有意义。
        require(amount0Out > 0 || amount1Out > 0, 'UniswapV2: INSUFFICIENT_OUTPUT_AMOUNT');
        (uint112 _reserve0, uint112 _reserve1,) = getReserves(); // gas savings
        require(amount0Out < _reserve0 && amount1Out < _reserve1, 'UniswapV2: INSUFFICIENT_LIQUIDITY');

        uint balance0;
        uint balance1;
        // 局部变量作用域 为了避免 EVM “Stack too deep” 错误，把部分变量封装在子作用域。
        { // scope for _token{0,1}, avoids stack too deep errors
            address _token0 = token0;
            address _token1 = token1;
            // 防止转回自己   接收地址不能是池子中任一代币合约地址，否则会破坏平衡。
            require(to != _token0 && to != _token1, 'UniswapV2: INVALID_TO');
            // 乐观转账 先把要给用户的代币输出给 to。如果后续输入不足或校验失败，会 revert 整个交易。
            if (amount0Out > 0) _safeTransfer(_token0, to, amount0Out); // optimistically transfer tokens
            if (amount1Out > 0) _safeTransfer(_token1, to, amount1Out); // optimistically transfer tokens
            // 闪兑回调（可选） 如果传入了回调数据，合约会调用接收者的 uniswapV2Call，支持闪电贷或复杂逻辑。回调中可以从池子再转入资产。
            if (data.length > 0) IUniswapV2Callee(to).uniswapV2Call(msg.sender, amount0Out, amount1Out, data);
            // 读取新余额 转账与回调后，再次读取池子中两种代币的最新余额，用于后续输入量计算。
            balance0 = IERC20(_token0).balanceOf(address(this));
            balance1 = IERC20(_token1).balanceOf(address(this));
        }
        uint amount0In = balance0 > _reserve0 - amount0Out ? balance0 - (_reserve0 - amount0Out) : 0;
        uint amount1In = balance1 > _reserve1 - amount1Out ? balance1 - (_reserve1 - amount1Out) : 0;
        require(amount0In > 0 || amount1In > 0, 'UniswapV2: INSUFFICIENT_INPUT_AMOUNT');
        { // scope for reserve{0,1}Adjusted, avoids stack too deep errors
            uint balance0Adjusted = balance0.mul(1000).sub(amount0In.mul(3));
            uint balance1Adjusted = balance1.mul(1000).sub(amount1In.mul(3));
            require(balance0Adjusted.mul(balance1Adjusted) >= uint(_reserve0).mul(_reserve1).mul(1000**2), 'UniswapV2: K');
        }
        // 状态更新与事件
        //      调用 _update 同步新储备量，并更新价格累计、时间戳。
        //      触发 Swap 事件，便于链上索引与前端展示。
        _update(balance0, balance1, _reserve0, _reserve1);
        emit Swap(msg.sender, amount0In, amount1In, amount0Out, amount1Out, to);
    }
    // skim：转走因直接转账进合约多余的资产。
    // force balances to match reserves
    function skim(address to) external lock {
        address _token0 = token0; // gas savings
        address _token1 = token1; // gas savings
        _safeTransfer(_token0, to, IERC20(_token0).balanceOf(address(this)).sub(reserve0));
        _safeTransfer(_token1, to, IERC20(_token1).balanceOf(address(this)).sub(reserve1));
    }

    // sync：手动把储备同步到当前余额。
    // force reserves to match balances
    function sync() external lock {
        _update(IERC20(token0).balanceOf(address(this)), IERC20(token1).balanceOf(address(this)), reserve0, reserve1);
    }
}
