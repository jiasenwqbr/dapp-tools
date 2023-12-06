// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "./Exchange.sol";
contract Factory {
    mapping (address => address) public tokenExchange;

    function getExchange(address _tokenAddress) public view returns(address){
        return tokenExchange[_tokenAddress];
    }

    function createExchange(address _tokenAddress) public returns(address){
        require(_tokenAddress != address(0),"Invalid token address");
        require(tokenExchange[_tokenAddress] != address(0),"exchange exist");

        Exchange exchange = new Exchange(_tokenAddress);
        tokenExchange[_tokenAddress] = address(exchange);
        return address(exchange);
    }

}
