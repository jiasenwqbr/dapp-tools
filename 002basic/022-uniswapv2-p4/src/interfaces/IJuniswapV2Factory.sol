// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
interface IJuniswapV2Factory {
    function pairs(address,address) external pure returns (address);
    function createPair(address,address) external returns (address);
}