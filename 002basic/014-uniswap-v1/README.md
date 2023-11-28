# Programming DeFi: Uniswap. Part 1

Extracted from: 摘自：

https://jeiwan.net/posts/programming-defi-uniswap-1/

## Introduction 介绍

The best way to learn something is to teach others. Second best way to learn something is to do it yourself. I decided to combine the two ways and teach myself and you how to program DeFi services on Ethereum (and any other blockchains based on EVM – Ethereum Virtual Machine).

学习某样东西的最好方法就是教别人。 学习某事的第二个最佳方法是自己动手。 我决定将这两种方法结合起来，教我自己和你如何在以太坊（以及基于 EVM - 以太坊虚拟机的任何其他区块链）上编写 DeFi 服务。

Our main focus will be on how those services work, we’ll try to understand the economical mechanics that make them what they are (and they all based on economical mechanics). We’ll find out, decompose, learn, and build their core mechanisms.

我们的主要关注点将是这些服务如何运作，我们将尝试了解使它们成为现实的经济机制（并且它们都基于经济机制）。 我们将找出、分解、学习并构建它们的核心机制。

However, we’ll only work on smart contracts: building front-end for smart-contracts is also a big and interesting task, but it’s out of the scope of this series.

然而，我们只会研究智能合约：为智能合约构建前端也是一项艰巨而有趣的任务，但这超出了本系列的范围。

Let’s begin our journey with Uniswap.让我们开始 Uniswap 之旅。

A lot of us might have been in a situation wherein we had a brilliant idea, may it be of a product or an event, but just didn’t have the funds to bring it to life. This is where crowdfunding comes in.

我们中的很多人可能都遇到过这样的情况：我们有一个绝妙的想法，可能是一个产品或一个活动，但只是没有资金将其变为现实。 这就是众筹的用武之地。

## Different versions of Uniswap 不同版本的Uniswap

As of June 2021, three versions of Uniswap have been launched.

截至 2021 年 6 月，Uniswap 已推出三个版本。

First version was launched in November 2018 and it allowed only swaps between ether and a token. Chained swaps were also possible to allow token-token swaps.

第一个版本于 2018 年 11 月推出，仅允许以太币和代币之间的交换。 链式交换也可以允许代币之间的交换。

V2 was launched in March 2020 and it was a huge improvement of V1 that allowed direct swaps between any ERC20 tokens, as well as chained swaps between any pairs.

V2 于 2020 年 3 月推出，它是 V1 的巨大改进，允许任何 ERC20 代币之间直接交换，以及任何货币对之间的链式交换。

V3 was launched in May 2021 and it significantly improved capital efficiency, which allowed liquidity providers to remove a bigger portion of their liquidity from pools and still keep getting the same rewards (or squeeze the capital in smaller price ranges and get up to 4000x of profits).

V3 于 2021 年 5 月推出，显着提高了资本效率，这使得流动性提供者可以从池中移走更大一部分流动性，并仍然获得相同的奖励（或者将资本压缩在较小的价格范围内，并获得高达 4000 倍的利润） ）。

In this series, we’ll dig into each of the versions and will try to build simplified copies of each of them from scratch.

在本系列中，我们将深入研究每个版本，并尝试从头开始构建每个版本的简化副本。

**This blog post specifically focuses on Uniswap V1** to respect the chronological order and better understand how previous solutions were improved.

**这篇博文特别关注 Uniswap V1**，以尊重时间顺序并更好地了解以前的解决方案是如何改进的。

## What is Uniswap?

In simple terms, [Uniswap](https://uniswap.org/) is a decentralized exchange (DEX) that aims to be an alternative to centralized exchanges. It’s running on the Ethereum blockchain and it’s fully automated: there are no admins, managers, or users with privileged access.

简单来说，Uniswap 是一个去中心化交易所（DEX），旨在成为中心化交易所的替代品。 它在以太坊区块链上运行并且完全自动化：没有管理员、经理或具有特权访问权限的用户。

On the lower lever, it’s an algorithm that allows to make pools, or token pairs, and fill them with liquidity to let users exchange tokens using this liquidity. Such algorithm is called *automated market maker* or *automated liquidity provider*.

在较低的杠杆上，它是一种算法，允许创建池或代币对，并为其填充流动性，以便用户使用这种流动性交换代币。 这种算法被称为自动做市商或自动流动性提供者。

Let’s learn more about market makers.让我们更多地了解做市商。

Market makers are entities that provide liquidity (trading assets) to markets. Liquidity is what makes trades possible: if you want to sell something but no one is buying it, there won’t be a trade. Some trading pairs have high liquidity (e.g. BTC-USDT), but some have low or no liquidity at all (like some scammy or shady altcoins).

做市商是向市场提供流动性（交易资产）的实体。流动性使交易成为可能：如果你想出售某种东西但没人购买，那么就不会进行交易。有些交易具有较高的流动性（例如BTC-USDT），但有些交易的流动性较低或根本没有流动性（例如一些诈骗或可疑的山寨币）。

A DEX must have enough (or a lot of) liquidity to function and serve as an alternative to centralized exchanges. One way to get that liquidity is for the developers of the DEX to put their own money (or money of their investors) in it and become market makers. However, this is not a realistic solution because they would need a lot of money to provide enough liquidity for all pairs, considering that DEXes allow exchanges between any tokens. Moreover, this would make the DEX centralized: as the only market makers, the developers would have a lot of power in their hands.

DEX 必须拥有足够（或大量）的流动性才能发挥作用并作为中心化交易所的替代品。 获得流动性的一种方法是 DEX 开发商将自己的资金（或投资者的资金）投入其中并成为做市商。 然而，这不是一个现实的解决方案，因为考虑到 DEX 允许任何代币之间的交换，他们需要大量资金来为所有货币对提供足够的流动性。 此外，这将使 DEX 中心化：作为唯一的做市商，开发商手中将拥有很大的权力。

A better solution is to allow **anyone to be a market maker**, and this is what makes Uniswap an automated market maker: any user can deposit their funds into a trading pair (and benefit from that).

更好的解决方案是允许任何人成为做市商，这就是 Uniswap 成为自动化做市商的原因：任何用户都可以将资金存入交易对（并从中受益）。

Another important role that Uniswap plays is **price oracle**. Price oracles are services that fetch token prices from centralized exchanges and provide them to smart contracts – such prices are usually hard to manipulate because volumes on centralized exchanges are often very big. However, while not having that big volumes, Uniswap can still serve as a price oracle.

Uniswap 扮演的另一个重要角色是价格预言机。 价格预言机是从中心化交易所获取代币价格并将其提供给智能合约的服务——此类价格通常很难操纵，因为中心化交易所的交易量通常非常大。 然而，尽管 Uniswap 的交易量没有那么大，但它仍然可以充当价格预言机。

Uniswap acts as a secondary market that attracts arbitrageurs who make profit on differences in prices between Uniswap and centralized exchanges. This makes prices on Uniswap pools as close as possible to those on bigger exchanges. And that wouldn’t have been possible without proper pricing and reserves balancing functions.

Uniswap 作为二级市场，吸引了套利者，他们通过 Uniswap 和中心化交易所之间的价格差异获利。 这使得 Uniswap 池上的价格尽可能接近大型交易所的价格。 如果没有适当的定价和储备平衡功能，这是不可能的。

## Constant product market maker

You probably have already heard this definition, let’s see what it means.

您可能已经听说过这个定义，让我们看看它的意思。

Automated market maker is a general term that embraces different decentralized market maker algorithms. The most popular ones (and those that gave birth to the term) are related to prediction markets - markets that allow to make profit on predictions. Uniswap and other on-chain exchanges are a continuation of those algorithms.

自动做市商是一个通用术语，包含不同的去中心化做市商算法。 最受欢迎的市场（以及诞生该术语的市场）与预测市场有关 - 允许通过预测获利的市场。 Uniswap 和其他链上交易所是这些算法的延续。

At the core of Uniswap is the constant product function:

Uniswap 的核心是恒定乘积函数：

x * y = k

Where *x* is ether reserve, y is token reserve (or vice versa), and *k* is a constant. Uniswap requires that *k* remains the same no matter how much of reserves of x or *y* there are. When you trade ether for tokens you deposit your ethers into the contract and get some amount of tokens in return. Uniswap ensures that after each trade *k* remains the same (this is not really true, we’ll see later why).

其中 *x* 是以太储备，y 是代币储备（反之亦然），*k* 是常数。 Uniswap 要求无论 x 或 *y* 有多少储备，*k* 保持不变。 当您用以太币交易代币时，您将以太币存入合约并获得一定数量的代币作为回报。 Uniswap 确保每次交易后 *k* 保持不变（这不是真的，我们稍后会看到原因）。

This formula is also responsible for pricing calculations, and we’ll soon see how.

这个公式还负责定价计算，我们很快就会看到如何进行。

## Smart contracts development

To really understand how Uniswap works we’ll build a copy of it. We’ll write smart contracts in [Solidity](https://soliditylang.org/) and will use [HardHat](https://hardhat.org/) as our development environment. HardHat is a really nice tool that greatly simplifies development, testing, and deployment of smart contracts. Highly recommended!

为了真正了解 Uniswap 的工作原理，我们将构建一个它的副本。 我们将在 [Solidity](https://soliditylang.org/) 中编写智能合约，并将使用 [HardHat](https://hardhat.org/) 作为我们的开发环境。 HardHat 是一个非常好的工具，它极大地简化了智能合约的开发、测试和部署。 强烈推荐！

If you’re new to smart contracts development, I highly recommend you to finish [this course](https://cryptozombies.io/en/course/) (at least the basic path) – that’ll be a huge help to you!

如果您是智能合约开发的新手，我强烈建议您完成[本课程](https://cryptozombies.io/en/course/)（至少是基本路径）——这将对您有很大帮助 你！

### Setting up the project

First, create an empty directory (I called mine `zuniswap`), `cd` into it and install HardHat:

```bash
$ mkdir zuniswap && cd $_
$ yarn add -D hardhat
```



We’ll also need a token contract, let’s use ERC20 contracts provided by [OpenZeppelin](https://openzeppelin.com/).

```bash
$ yarn add -D @openzeppelin/contracts
```

Initialize a HardHat project and remove everything from `contract`, `script`, and `test` folders.

```bash
$ yarn hardhat
...follow the instructions...
$ rm ...
$ tree -a
.
├── .gitignore
├── contracts
├── hardhat.config.js
├── scripts
└── test
```

Final touch: we’ll use the latest version of Solidity, which is 0.8.4 at the time of writing . Open your `hardhat.config.js` and update Solidity version at the bottom of it.

### Token contract

Uniswap V1 supports only ether-token swaps. To make them possible we need an ERC20 token contract. Let’s write it!

