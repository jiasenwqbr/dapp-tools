// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {AggregatorV3Interface} from "@chainlink/contracts/src/v0.8/shared/interfaces/AggregatorV3Interface.sol";


contract MyToken is ERC20 {
    AggregatorV3Interface internal dataFeeds = AggregatorV3Interface(0x694AA1769357215DE4FAC081bf1f309aDC325306);
    uint256 minimum_value = 10 * 10 ** 18;
    constructor() ERC20("MyToken", "MTK") {}
    // 让所有mint的人支付100 usd
    function mint()  public payable {
        require(convertEthAmountToUSDValue(msg.value) > 100 * (10 **18),"Send more ETH");
        
        _mint(msg.sender,minimum_value);
    }

    function convertEthAmountToUSDValue(uint256 ethAmount) internal view  returns (uint256){
        (
            /* uint80 roundID */,
            int answer,
            /*uint startedAt*/,
            /*uint timeStamp*/,
            /*uint80 answeredInRound*/
        ) = dataFeeds.latestRoundData();
        return uint256(answer) * ethAmount / (10**8);
    }
}
