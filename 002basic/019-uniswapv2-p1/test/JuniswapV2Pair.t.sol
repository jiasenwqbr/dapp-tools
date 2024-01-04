// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
import "../src/JuniswapV2Pair.sol";
import "./mocks/ERC20Mintable.sol";
contract JuniswapV2PairTest is Test{
    ERC20Mintable token0;
    ERC20Mintable token1;
    JuniswapV2Pair pair;
    TestUser testUser;
    
    function setUp() public {
        testUser = new TestUser();

        token0 = new ERC20Mintable("Token A","TkA");
        token1 = new ERC20Mintable("Token B","TKB");
        pair = new JuniswapV2Pair(address(token0),address(token1));
        
        token0.mint(10 ether,address(this));
        token1.mint(10 ether,address(this));

        token0.mint(10 ether,address(testUser));
        token1.mint(10 ether, address(testUser));
    }
    
    function testMintBootstrap() public {
        token0.transfer(address(pair), 1 ether);
        token1.transfer(address(pair), 1 ether);

        pair.mint();
        assertEq(pair.balanceOf(address(this)), 1 ether - 1000);
        assertReserves(1 ether, 1 ether);
        assertEq(pair.totalSupply(), 1 ether);
    }
    function assertReserves(uint112 expectedReserve0,uint112 expectedReserve1) internal {
        (uint112 reserve0,uint112 reserve1,) = pair.getReserves();
        assertEq(reserve0, expectedReserve0, "unexpected reserve0");
        assertEq(reserve1, expectedReserve1, "unexpected reserve1");
    }

}

contract TestUser {
    function provideLiquidity(
        address pairAddress_,
        address token0Address_,
        address token1Address_,
        uint256 amount0_,
        uint256 amount1_
    ) public {
        ERC20(token0Address_).transfer(pairAddress_, amount0_);
        ERC20(token1Address_).transfer(pairAddress_, amount1_);

        JuniswapV2Pair(pairAddress_).mint();
    }

    function withdrawLiquidity(address pairAddress_) public {
        JuniswapV2Pair(pairAddress_).burn();
    }
}