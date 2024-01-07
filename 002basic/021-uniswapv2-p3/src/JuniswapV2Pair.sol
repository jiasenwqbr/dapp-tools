// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
import "solmate/tokens/ERC20.sol";
import "./libraries/Math.sol";
import "./libraries/UQ112x112.sol";
interface IERC20 {
    function balanceOf(address) external returns (uint256);
    function transfer(address to, uint256 amount) external;
}
error AlreadyInitialized();
error BalanceOverflow();
error InsufficientLiquidityMinted();
error InsufficientLiquidityBurned();
error InsufficientOutputAmount();
error InsufficientLiquidity();
error InvalidK();
error TransferFailed();

contract JuniswapV2Pair is ERC20, Math {
    using UQ112x112 for uint224;
    uint256 constant MINIMUM_LIQUIDITY = 1000;

    address public token0;
    address public token1;

    uint112 private reserve0;
    uint112 private reserve1;
    uint32 private blockTimestampLast;

    uint256 public price0CumulativeLast;
    uint256 public price1CumulativeLast;

    event Burn(address indexed sender, uint256 amount0, uint256 amount1);
    event Mint(address indexed sender, uint256 amount0, uint256 amount1);
    event Sync(uint256 reserve0, uint256 reserve1);
    event Swap(
        address indexed sender,
        uint256 amount0Out,
        uint256 amount1Out,
        address indexed to
    );

    constructor(address token0_,address token1_) ERC20("Juinswapv2","JUNIV2",18){
        token0 = token0_;
        token1 = token1_;
    }
    function initialize(address token0_, address token1_) public {
        if (token0 != address(0) || token1 != address(0))
            revert AlreadyInitialized();

        token0 = token0_;
        token1 = token1_;
    }

    function mint() public {
        // The purpose of this function is to fetch the current reserves of the liquidity pool.
        //这个函数的目的是获取交易对当前的资金池储备（reserves）。
        (uint112 reserve0_,uint112 reserve1_,) = getReserves();
        // These lines respectively fetch the balances of the token0 and token1 tokens held by the current smart contract address.
        // 这两行代码分别获取当前智能合约地址持有的 token0 和 token1 代币的余额
        uint256 balance0 = IERC20(token0).balanceOf(address(this));
        uint256 balance1 = IERC20(token1).balanceOf(address(this));
        // these lines calculate the changes in balance for each token relative to the reserves.
        // 这两行代码计算了每个代币相对于储备的余额变化量。
        uint256 amount0 = balance0 - reserve0_;
        uint256 amount1 = balance1 - reserve1_;
        uint256 liquidity;
        if (totalSupply == 0){
            liquidity = Math.sqrt(amount0 * amount1) - MINIMUM_LIQUIDITY;
            _mint(address(0), MINIMUM_LIQUIDITY);
        } else {
            liquidity = Math.min(
                (amount0 * totalSupply) / reserve0_,
                (amount1 * totalSupply) / reserve1_
            );
        }

        if (liquidity <= 0) revert InsufficientLiquidityMinted();
        _mint(msg.sender, liquidity);

        _update(balance0, balance1, reserve0_, reserve1_);
        emit Mint(msg.sender, amount0, amount1);
    }

    function burn() public {
        uint256 balance0 = IERC20(token0).balanceOf(address(this));
        uint256 balance1 = IERC20(token1).balanceOf((address(this)));
        uint256 liquidity = balanceOf[msg.sender];

        uint256 amount0 = (liquidity * balance0) / totalSupply;
        uint256 amount1 = (liquidity * balance1) / totalSupply;
        if (amount0 <= 0 || amount1 <= 0) revert InsufficientLiquidityBurned();
        _burn(msg.sender, liquidity);
        _safeTransfer(token0, msg.sender, amount0);
        _safeTransfer(token1, msg.sender, amount1);
        balance0 = IERC20(token0).balanceOf(address(this));
        balance1 = IERC20(token1).balanceOf(address(this));
        (uint112 reserve0_, uint112 reserve1_, ) = getReserves();
        _update(balance0, balance1, reserve0_, reserve1_);
        emit Burn(msg.sender, amount0, amount1);
    }

    function swap(uint256 amount0Out,uint256 amount1Out,address to) public {
        if(amount0Out == 0 && amount1Out == 0){
            revert InsufficientLiquidity();
        }
        (uint112 reserve0_, uint112 reserve1_, ) = getReserves();
        if (amount0Out > reserve0_ || amount1Out > reserve1_){
            revert InsufficientLiquidity();
        }
        uint256 balance0 = IERC20(token0).balanceOf(address(this)) - amount0Out;
        uint256 balance1 = IERC20(token1).balanceOf(address(this)) - amount1Out;
        _update(balance0, balance1, reserve0_, reserve1_);
        if (balance0 * balance1 < uint256(reserve0_) * uint256(reserve1_)){
            revert InvalidK();
        }
        if (amount0Out > 0) _safeTransfer(token0, to, amount0Out);
        if (amount1Out > 0) _safeTransfer(token1, to, amount1Out);

        emit Swap(msg.sender, amount0Out, amount1Out, to);
    }

    function sync() public {
        (uint112 reserve0_, uint112 reserve1_, ) = getReserves();
        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this)),
            reserve0_,
            reserve1_
        );
    }
    function getReserves() public view returns (uint112,uint112,uint32){
        return (reserve0,reserve1,0);
    }

    function _update(
        uint256 balance0,
        uint256 balance1,
        uint112 reserve0_,
        uint112 reserve1_
        ) private {
        if (balance0 > type(uint112).max || balance1 > type(uint112).max)
            revert BalanceOverflow();

        unchecked {
            uint32 timeElapsed = uint32(block.timestamp) - blockTimestampLast;

            if (timeElapsed > 0 && reserve0_ > 0 && reserve1_ > 0) {
                price0CumulativeLast +=
                    uint256(UQ112x112.encode(reserve1_).uqdiv(reserve0_)) *
                    timeElapsed;
                price1CumulativeLast +=
                    uint256(UQ112x112.encode(reserve0_).uqdiv(reserve1_)) *
                    timeElapsed;
            }
        }

        reserve0 = uint112(balance0);
        reserve1 = uint112(balance1);
        blockTimestampLast = uint32(block.timestamp);

        emit Sync(reserve0, reserve1);
    }

    function _safeTransfer (
        address token,
        address to,
        uint256 value
    ) private {
        (bool success,bytes memory data) = token.call(
            abi.encodeWithSignature("transfer(address,uint256)", to, value)
        );
        if (!success || (data.length != 0 && !abi.decode(data, (bool))))
            revert TransferFailed();
    }

}



