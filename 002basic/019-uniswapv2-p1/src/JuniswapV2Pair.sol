// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
import "solmate/tokens/ERC20.sol";
import "./libraries/Math.sol";

interface IERC20 {
    function balanceOf(address) external returns (uint256);
    function transfer(address to, uint256 amount) external;
}
error InsufficientLiquidityMinted();
error InsufficientLiquidityBurned();
error TransferFailed();
contract JuniswapV2Pair is ERC20, Math {
    uint256 constant MINIMUM_LIQUIDITY = 1000;

    address public token0;
    address public token1;

    uint112 private reserve0;
    uint112 private reserve1;

    event Burn(address indexed sender, uint256 amount0, uint256 amount1);
    event Mint(address indexed sender, uint256 amount0, uint256 amount1);
    event Sync(uint256 reserve0, uint256 reserve1);

    constructor(address token0_,address token1_) ERC20("Juinswapv2","JUNIV2",18){
        token0 = token0_;
        token1 = token1_;
    }

    function mint() public {
        // The purpose of this function is to fetch the current reserves of the liquidity pool.
        //这个函数的目的是获取交易对当前的资金池储备（reserves）。
        (uint112 _reserve0,uint112 _reserve1,) = getReserves();
        // These lines respectively fetch the balances of the token0 and token1 tokens held by the current smart contract address.
        // 这两行代码分别获取当前智能合约地址持有的 token0 和 token1 代币的余额
        uint256 balance0 = IERC20(token0).balanceOf(address(this));
        uint256 balance1 = IERC20(token1).balanceOf(address(this));
        // these lines calculate the changes in balance for each token relative to the reserves.
        // 这两行代码计算了每个代币相对于储备的余额变化量。
        uint256 amount0 = balance0 - _reserve0;
        uint256 amount1 = balance1 - _reserve1;
        uint256 liquidity;
        if (totalSupply == 0){
            liquidity = Math.sqrt(amount0 * amount1) - MINIMUM_LIQUIDITY;
            _mint(address(0), MINIMUM_LIQUIDITY);
        } else {
            liquidity = Math.min(
                (amount0 * totalSupply) / _reserve0,
                (amount1 * totalSupply) / _reserve1
            );
        }

        if (liquidity <= 0) revert InsufficientLiquidityMinted();
        _mint(msg.sender, liquidity);

        _update(balance0, balance1);

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
        _update(balance0, balance1);
        emit Burn(msg.sender, amount0, amount1);
    }

    function sync() public {
        _update(
            IERC20(token0).balanceOf(address(this)),
            IERC20(token1).balanceOf(address(this))
        );
    }
    function getReserves() public view returns (uint112,uint112,uint32){
        return (reserve0,reserve1,0);
    }

    function _update(uint256 balance0,uint256 balance1) private {
        reserve0 = uint112(balance0);
        reserve1 = uint112(balance1);
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



