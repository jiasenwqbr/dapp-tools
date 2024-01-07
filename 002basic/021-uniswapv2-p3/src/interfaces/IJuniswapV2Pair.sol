// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
interface IJuniswapV2Pair {
    function initialize(address,address) external;
    function getReverses() external returns (uint112,uint112,uint32);
    function mint(address) external returns (uint256);
}