pragma solidity =0.5.16;

/**
    Solidity 版本锁定：=0.5.16，确保和 Uniswap V2 主网兼容。
    接口导入：IUniswapV2Factory 定义工厂合约应有的方法签名，便于类型检查和 IDE 补全。
    Pair 合约：后续 createPair 会动态部署 UniswapV2Pair。
 */

import './interfaces/IUniswapV2Factory.sol';
import './UniswapV2Pair.sol';

contract UniswapV2Factory is IUniswapV2Factory {
    // 收取协议交易手续费（如 swap fee）的接收地址。
    address public feeTo;
    // 有权限更改 feeTo 地址的管理者。
    address public feeToSetter;
    // 双层映射，用于快速查找已创建的交易对。
    mapping(address => mapping(address => address)) public getPair;
    // 保存所有创建过的 pair 合约地址，便于枚举。
    address[] public allPairs;
    // 在每次新交易对创建时发出，由前端或索引器（The Graph）监听，方便展现新市场。
    event PairCreated(address indexed token0, address indexed token1, address pair, uint);
    // 部署工厂时指定一开始的管理员 feeToSetter，之后它可以调用 setFeeTo / setFeeToSetter。
    constructor(address _feeToSetter) public {
        feeToSetter = _feeToSetter;
    }
    // 返回当前已创建交易对的数量，接口友好，比直接读取 allPairs.length 更安全。
    function allPairsLength() external view returns (uint) {
        return allPairs.length;
    }

    // 核心函数——createPair
    function createPair(address tokenA, address tokenB) external returns (address pair) {
        // 1.去重与排序
        // tokenA != tokenB 防止同地址做自交易对。
        require(tokenA != tokenB, 'UniswapV2: IDENTICAL_ADDRESSES');
        // 通过地址大小 < 确定 token0/token1，保证映射一致性。
        (address token0, address token1) = tokenA < tokenB ? (tokenA, tokenB) : (tokenB, tokenA);
        // 2.地址合法性校验 不允许零地址。
        require(token0 != address(0), 'UniswapV2: ZERO_ADDRESS');
        // 3.防重复创建
        // getPair[token0][token1] 必须为空，单向检查即可（存储了正反向映射）。
        require(getPair[token0][token1] == address(0), 'UniswapV2: PAIR_EXISTS'); // single check is sufficient
        // 4.使用 CREATE2 部署 Pair 合约
        // creationCode 获取 UniswapV2Pair 的字节码。
        bytes memory bytecode = type(UniswapV2Pair).creationCode;
        // 盐值 (salt) 由两个 token 地址哈希得到，保证同一对只能部署一次，且地址可预计算。
        bytes32 salt = keccak256(abi.encodePacked(token0, token1));
        // 内联 Assembly 调用 create2，可指定合约地址并节省 gas。
        /**
         * value：发送给新合约的以太（wei），此处为 0。
         * offset：内存中合约字节码的起始位置。
         * size：合约字节码长度。
         * salt：32 字节的值，用于影响新合约的地址计算，确保同一字节码在相同部署者地址上、不同 salt 会得到不同合约地址。
         */
        /**
         * bytecode 变量
         *  在 Solidity 中，type(UniswapV2Pair).creationCode 返回一个 bytes 类型的数据，
         *  它的结构是：[0…31]   = 部署字节码长度（uint256） [32…]    = 实际的合约字节码内容
         *  前 32 字节是 Solidity 把长度放在 bytes 数组开头的“ABI 编码”前缀。
         *  真正需要传给 EVM 的，是从第 32 字节开始的那部分。
         * add(bytecode, 32)
         *  add(ptr, 32) 是将内存地址 ptr 向后偏移 32 字节，跳过那个长度前缀。
         *  因此 add(bytecode, 32) 正好指向合约字节码的第一条指令。
         * mload(bytecode)
         *  mload(p) 会读取内存地址 p 处的 32 字节数据，也就是 bytecode 数组开头的“长度前缀”。
         *  这给出了合约字节码的实际大小 size。
         * salt = keccak256(abi.encodePacked(token0, token1))
         *  CREATE2 的地址计算公式是：address = keccak256(0xff ++ deployerAddress ++ salt ++ keccak256(bytecode))[12:]
         *  这样，你就能在链下根据 token0 和 token1 预计算这个交易对合约地址，并且保证同一对不会重复部署（因为相同 token0,token1 会用相同 salt）。
         * 部署结果
         *  create2 部署成功后返回新合约地址，否则返回 0x0 并 revert。
         *  这个地址同时也是 getPair[token0][token1] 的值，并用于后续对该 Pair 合约的初始化调用。
         */

        assembly {
            pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
        }
        /**
         * 为什么用 CREATE2？
            地址可预计算：前端或其他合约可以提前算出某个 Pair 的合约地址，而不用等它真正部署。
            防止重复部署：同样的 salt 在同一工厂地址上只会生成一次；第二次部署会失败（返回零地址）。
            链下一致性：同样的部署逻辑在所有网络上复现时，生成的地址完全一致，方便跨链工具和索引器使用。
            这样，UniswapV2Factory 就实现了「去中心化且无须许可」的交易对创建，同时保证了唯一性和地址可预测性。
         */

        // 5.初始化
        // 部署后需调用 initialize(token0, token1)，设置 Pair 合约的初始状态（如流动性 Token 名称、交易对地址）。
        IUniswapV2Pair(pair).initialize(token0, token1);
        // 6.映射与数组更新
        // 双向存储，便于任何顺序的查询。
        getPair[token0][token1] = pair;
        getPair[token1][token0] = pair; // populate mapping in the reverse direction
        // 将新 Pair 地址 push 到 allPairs。
        allPairs.push(pair);
        // 7.发出事件
        // 同步链上索引
        emit PairCreated(token0, token1, pair, allPairs.length);
    }
    // 管理费用收取地址
    function setFeeTo(address _feeTo) external {
       
        require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        feeTo = _feeTo;
    }
    // 双重保护：只有当前 feeToSetter 能修改。
    // feeToSetter 可委派给多签、DAO 等，从而治理协议费的分配。
    function setFeeToSetter(address _feeToSetter) external {
        require(msg.sender == feeToSetter, 'UniswapV2: FORBIDDEN');
        feeToSetter = _feeToSetter;
    }
}
