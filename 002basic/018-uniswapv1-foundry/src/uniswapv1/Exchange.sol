// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/token/ERC20/ERC20.sol";
interface IExchange {

}
interface IFactory {

}
contract Exchange is ERC20 {
    address public tokenAddress;
    address public factoryAddress;
    constructor(address _token) ERC20("ZhiZunBaoSwapV1","ZZB-V1"){
        require(_token != address(0),"Invalid token address");
        tokenAddress = _token;
        factoryAddress = msg.sender;
    }

    function addLiquidity(uint256 tokenAmount) public payable {
        // Reserve of the token held by the exchange 交易所持有的token数量
        uint256 tokenReserve = getReserve();
        //  Reference to the token contract token合约的引用
        IERC20 token = IERC20(tokenAddress);
        if (tokenReserve == 0){
            // Amount of Ether held by the exchange 交易所持有以太的数量
            uint256 liquidity = address(this).balance;
            // 调用合约的 _mint 函数，向调用者（msg.sender）发行新的流动性代币，代表他们提供了以太币。
            _mint(msg.sender, liquidity);
            // 从调用者账户中转移 tokenAmount 数量的代币到合约地址，作为提供的代币。
            token.transferFrom(msg.sender, address(this), tokenAmount);
        } else {
            // 计算合约当前持有的以太币数量，将其存储在 ethReserve 变量中。
            uint256 ethReserve = address(this).balance;
            // 根据用户提供的以太币数量，计算需要转移的代币数量。
            uint256 needTokenAmount = (tokenReserve * msg.value) / ethReserve;
            // 断言确保用户提供的代币数量足够，防止用户欺骗合约。
            require(needTokenAmount <= tokenAmount,"insufficient token amount");
            // 从调用者账户中转移 needTokenAmount 数量的代币到合约地址。
            token.transferFrom(msg.sender, address(this), needTokenAmount);
            // 计算发行给用户的新流动性代币数量，与用户提供的以太币数量和当前总流动性代币数量相关。
            uint256 liquidity = (totalSupply() * msg.value) /ethReserve ;
            // 向调用者发行新的流动性代币，代表他们提供了以太币和代币。
            _mint(msg.sender, liquidity);
        }
    }

    function removeLiquidity(uint256 _amount) public returns(uint256,uint256) {
        require(_amount>0,"invalid amount");
        uint256 ethAmount = (address(this).balance * _amount) / totalSupply();
        uint256 tokenAmount = (getReserve() * _amount) / totalSupply();
        _burn(msg.sender,_amount);
        payable(msg.sender).transfer(ethAmount);
        IERC20(tokenAddress).transfer(msg.sender, tokenAmount);
        return (ethAmount,tokenAmount);
    }

    function getReserve() public view returns (uint256) {
        return IERC20(tokenAddress).balanceOf(address(this)); 
    }


}