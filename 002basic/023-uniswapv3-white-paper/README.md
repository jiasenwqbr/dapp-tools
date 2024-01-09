# **Uniswap v3 Core**

## ABSTRACT 概述

Uniswap v3 is a noncustodial automated market maker implemented for the Ethereum Virtual Machine.In comparison to earlier versions of the protocol,Uniswap v3 provides increased capital  efficiency and fine-tuned control to liquidity providers,improves the accuracy and convenience of the price oracle,and has a more flexible fee structure.

Uniswap v3是一个基于以太坊虚拟机（EVM）实现的无监管自动做市商（AMM）。与之前的版本相比，Uniswap v3提高了资金利用率，赋予流动性提供者更多控制能力，改进了价格预言机的准确性和便利性，同时增加了更灵活的手续费结构。

## **1INTRODUCTION** 介绍

Automated market makers (AMMs) are agents that pool liquidity  and make it available to traders according to an algorithm[5].Constant function market makers (CFMMs), a broad class of AMMs of which Uniswap is a member,have seen widespread use in the context of decentralized finance,where they are typically implemented as smart contracts that trade tokens on a permissionless blockchain [2].

自动做市商（AMMs）是集中流动性，并基于算法将其开放给交易者的代理商。常值函数做市商（CFMMs）（Uniswap也是成员之一）作为AMM中的一个常见类别，已被广泛应用于去中心化金融场景，他们一般都在无需许可的区块链上以交易代币的智能合约的形式实现。

CFMMs as they are implemented today are often capital inefficient.In the constant product market maker formula used by Uniswapv1 and v2,only a fraction of the assets in the pool are available at a given price.This is inefficient,particularly when assets are expected to trade close to a particular price at all times.

当前市场上的常值函数做市商大多存在资金利用率不高的问题。在Uniswap v1/v2使用的恒定乘积做市商公式中，对于给定价格，池子中仅部分资金参与做市。这显得十分低效，特别是当代币总是在特定价格附近交易时。

Prior attempts to address this capital efficiency issue,such as Curve[3] and YieldSpace[4],have involved building pools that use different functions to describe the relation between reserves.This requires all liquidity providers in a given pool to adhere to a single  formula,and could result in liquidity fragmentation if liquidity  providers want to provide liquidity within different price ranges.

之前解决这个资本效率问题的尝试，例如 Curve[3] 和 YieldSpace[4]，都涉及建立使用不同函数来描述储备之间关系的池。这要求给定池中的所有流动性提供者都遵守单一的规则。 如果流动性提供者希望在不同的价格范围内提供流动性，则可能会导致流动性碎片化。

In this paper,we present Uniswap v3,a novel AMM that gives liquidity providers more control over the price ranges in which their capital is used,with limited effect on liquidity fragmentation and gas ineffciency.This design does not depend on any shared assumption about the price behavior of the tokens.Uniswap v3 is based on the same constant product reserves curve as earlier versions[1],but offers several significant new features:

在本文中，我们提出了 Uniswap v3，这是一种新颖的 AMM，它使流动性提供者能够更好地控制其资本使用的价格范围，对流动性碎片化和 Gas 效率低下的影响有限。这种设计不依赖于任何关于流动性的共同假设 代币的价格行为。Uniswap v3 基于与早期版本[1]相同的恒定产品储备曲线，但提供了几个重要的新功能：

- *Concentrated Liquidity*：Liquidity providers(LPs) are given the ability to concentrate their liquidity by “bounding" it  within an arbitrary price range.This improves the pool’s capital effciency and allows LPs to approximate their preferred reserves curve,while still being effciently aggregated  with the rest of the pool.We describe this feature in section 2 and its implementation in Section 6.   *集中流动性*：流动性提供者（LP）能够通过将其流动性“限制”在任意价格范围内来集中其流动性。这提高了资金池的资本效率，并允许 LP 接近其首选准备金曲线，同时仍然有效聚合 与池的其余部分。我们在第 2 节中描述此功能，并在第 6 节中描述其实现。

  

- Fliexible Fees:The swap fee is nolonger locked at 0*.*30%.Rather,the fee tier for each pool(of which there can be multiple per asset pair) is set on initialization(Section 3.1).The initially supported fee tiers are 0.05%, 0.30%, and1%.UNI governance is able to add additional values to this set.

  费用灵活：互换费用不再锁定在0*.*30%。而是在初始化时设置每个池（每个资产对可以有多个）的费用等级（第3.1节）。最初支持的费用等级 分别是 0.05%、0.30% 和 1%。UNI 治理能够为该集合添加额外的值。

  

- Improved Price Oracle:Uniswap v3 provides a way for users to query recent price accumulator values, thus avoiding the need to check point the accumulator value at the exact beginning and end of the period for which a  TWAP is being measured.(Section5.1).

  改进的价格预言机：Uniswap v3 为用户提供了一种查询最近价格累加器值的方法，从而避免了在测量 TWAP 期间的确切开始和结束时检查累加器值的需要。（第 5.1 节） 。

  

- Liquidity Oracle: The contracts expose a time-weighted average liquidity oracle(Section 5.3).

  流动性预言机：合约公开了时间加权平均流动性预言机（第 5.3 节）。

  

  The Uniswap v2 core contracts are non-upgradeable by design,so Uniswap v3 is implemented as an entirely new set of contracts, available here.The Uniswap v3 core contracts are also non-upgradeable,with some parameters controlled by governance as described in Section 4.

  Uniswap v2 核心合约在设计上是不可升级的，因此 Uniswap v3 是作为一套全新的合约实现的，可在此处获取。Uniswap v3 核心合约也是不可升级的，一些参数由治理控制，如第 4 节所述 。

  

  ## 2 **CONCENTRATED LIQUIDITY** 集中流动性

  The defning idea of Uniswap v3 is that of *concentrated liquidity*:liquidity bounded within some price range.

  Uniswap v3 的定义理念是“集中流动性”：流动性限制在某个价格范围内。

  In earlier versions,liquidity was distributed uniformly along the
  $$
  x*y=k
  $$
  reserves curve,where x and y are the respective reserves of two assets X and Y,and K is a constant[1].In other words,earlier versions were designed to provide liquidity across the entire price range(0,∞).This is simple to implement and allows liquidity to be efficiently aggregated,but means that much of the assets held in a pool are never touched.

  在早期版本中，流动性沿着准备金曲线均匀分布，其中x和y分别是两种资产X和Y的准备金，K是一个常数[1]。换句话说，早期版本旨在提供跨资产流动性 整个价格范围（0，∞）。这很容易实现，并且可以有效地聚合流动性，但这意味着池中持有的大部分资产从未被触及。

  

<img src="images/image-20240108223102765.png" alt="image-20240108223102765" style="zoom:50%;" />

​                           **Figure 1: Simulation of Virtual Liquidity**

​     Having considered this, it seems reasonable to allow LPs to concentrate their liquidity to smaller price ranges than (0*,* ∞).We call liquidity concentrated to a finite range a *position*. A position only needs to maintain enough reserves to support trading within    its range, and therefore can act like a constant product pool with larger reserves (we call these the *virtual reserves*) within that range.

​      考虑到这一点，允许 LP 将其流动性集中到小于 (0*,* ∞) 的价格范围似乎是合理的。我们将流动性集中到有限范围称为“头寸”。 头寸只需要维持足够的准备金来支持其范围内的交易，因此可以像一个恒定的产品池一样，在该范围内拥有更大的准备金（我们称之为“虚拟准备金”）。

Specifically, a position only needs to hold enough of asset $X$ to cover price movement to its upper bound, because upwards price movement1 corresponds to depletion of the $X$ reserves. Similarly, it only needs to hold enough of asset $Y$ to cover price movement to its lower bound. Fig. 1 depicts this relationship for a position on a range  $[p_a,p_b]$ and a current price  $p_c ∈ [p_a,p_b]$.  $𝑥_{real}$and $𝑦_{real} $ denote the position’s real reserves. When the price exits a position’s range, the position’s liquidity is no longer active, and no longer earns fees. At that point, its liquidity is composed entirely of a single asset, because the reserves of the other asset must have been entirely depleted. If the price ever reenters the range, the liquidity becomes active again.

具体来说，头寸只需要持有足够的资产 X 来覆盖价格变动至其上限，因为向上的价格变动 1 对应于 $X$ 储备的耗尽。 同样，它只需要持有足够的资产 $Y$来覆盖价格变动至其下限。 图 1 描述了 $[p_a,p_b]$ 范围内的仓位与当前价格 $p_c ∈ [p_a,p_b]$。 $𝑥_{real}$ 和  $𝑦_{real} $  表示头寸的实际储备。 当价格退出仓位范围时，该仓位的流动性不再活跃，也不再赚取费用。 到那时，它的流动性完全由单一资产组成，因为其他资产的储备肯定已经完全耗尽。 如果价格重新进入该范围，流动性就会再次活跃。

The amount of liquidity provided can be measured by the value $L$, which is equal to$\sqrt{k}$. The real reserves of a position are described by the curve:

提供的流动性数量可以用*𝐿*的值来衡量，它等于$\sqrt{k}$。 头寸的实际储备由曲线描述：
$$
（x{\frac{L}{\sqrt{P_b}}}）(y+L\sqrt{P_a}) = L^2
$$
This curve is a translation of formula 2.1 such that the position is solvent exactly within its range (Fig. 2).

该曲线是公式 2.1 的翻译，使得该位置恰好在其范围内（图 2）。

<img src="images/image-20240109113417922.png" alt="image-20240109113417922" style="zoom: 50%;" />





