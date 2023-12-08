// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/token/ERC20/ERC20.sol";
interface IExchange {
    function ethToTokenSwap(uint256 _minTokens) external payable;

    function ethToTokenTransfer(uint256 _minTokens, address _recipient)
        external
        payable;
}
interface IFactory {
    function getExchange(address _tokenAddress) external returns (address);
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

    function getAmount(uint256 inputAmount,uint256 inputReserve,uint256 outputReserve) 
        public 
        pure
        returns (uint256){
            require(inputReserve>0 && outputReserve>0 ,"invalid reserve");
            uint256 numerator = (inputAmount * 99) * outputReserve;
            uint256 denominator = (inputReserve * 100) + (inputAmount * 99);
            return numerator/denominator;

    }
    function getTokenAmount(uint256 _ethSold) public view returns (uint256) {
        require(_ethSold > 0, "ethSold is too small");

        uint256 tokenReserve = getReserve();

        return getAmount(_ethSold, address(this).balance, tokenReserve);
    }

    function getEthAmount(uint256 _tokenSold) public view returns (uint256) {
        require(_tokenSold > 0, "tokenSold is too small");

        uint256 tokenReserve = getReserve();

        return getAmount(_tokenSold, tokenReserve, address(this).balance);
    }

    function ethToToken(uint256 _minTokens,address recipent) private {
        uint256 tokenReserve = getReserve();
        uint256 inputAmount = msg.value;
        uint256 ethReserve = address(this).balance - msg.value;
        uint256 outputTokenAmount = getAmount(inputAmount, ethReserve, tokenReserve);
        require(outputTokenAmount >= _minTokens,"insufficient output amount");
        IERC20(tokenAddress).transfer(recipent,_minTokens);
    }
    function ethToTokenSwap(uint256 _minTokens) public payable {
        ethToToken(_minTokens, msg.sender);
    }

    function tokenToETHSwap(uint256 _tokenSold,uint256 _minEth) public payable {
        uint256 tokenReserve = getReserve();
        uint256 ethReserve = address(this).balance;
        uint256 outputETHAmount = getAmount(_tokenSold, tokenReserve, ethReserve);
        require(outputETHAmount >= _minEth, "insufficient output amount");
        IERC20(tokenAddress).transferFrom(msg.sender,address(this), _tokenSold);
        payable(msg.sender).transfer(outputETHAmount);
    }

    function tokenToTokenSwap(uint256 _tokenSold, uint256 _minTokenBought, address _targetTokenAddress) public {
        // exchange if exist
        address exchange = IFactory(factoryAddress).getExchange(_targetTokenAddress);
        require(exchange!=address(0) && exchange!=address(this), "exchange not exist");
        // token to eth
        uint256 tokenReserve = getReserve();
        uint256 ethReserve = address(this).balance;
        uint256 outputETHAmount = getAmount(
            _tokenSold,
            tokenReserve,
            ethReserve
        );
        
        IERC20(tokenAddress).transferFrom(msg.sender, address(this), _tokenSold);
        // another exchange eth to token
        IExchange(exchange).ethToTokenTransfer{value: outputETHAmount}(_minTokenBought, msg.sender);
    }




}