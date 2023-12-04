// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.20;
import "./Exchange.sol";
contract Factory {
    mapping(address => address) public tokenToExchange;

    function getExchange(address _tokenExchange) public view returns(address) {
        return tokenToExchange[_tokenExchange];
    } 

    function createExchange(address _tokenAddress) public returns(address) {
        require(_tokenAddress!=address(0),"invalid token address");
        require(tokenToExchange[_tokenAddress] == address(0),"exchange already exist");

        Exchange exchange = new Exchange(_tokenAddress);
        tokenToExchange[_tokenAddress] = address(exchange);

        return address(exchange);
    }
}