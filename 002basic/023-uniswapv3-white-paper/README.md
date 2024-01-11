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

  

## 2 CONCENTRATED LIQUIDITY 集中流动性

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



Liquidity providers are free to create as many positions as they see fit, each on its own price range. In this way, LPs can approximate any desired distribution of liquidity on the price space (see Fig. 3 for a few examples). Moreover, this serves as a mechanism to let the market decide where liquidity should be allocated. Rational LPs can reduce their capital costs by concentrating their liquidity in a narrow band around the current price, and adding or removing tokens as the price moves to keep their liquidity active.

流动性提供者可以自由地创建他们认为合适的任意数量的头寸，每个头寸都有自己的价格范围。 通过这种方式，有限合伙人可以在价格空间上近似任何期望的流动性分布（参见图 3 中的几个示例）。 此外，这是一种让市场决定流动性配置的机制。 理性有限合伙人可以通过将流动性集中在当前价格附近的狭窄区间内，并随着价格变动添加或删除代币以保持流动性活跃来降低资本成本。

### **2.1 Range Orders** 范围订单

Positions on very small ranges act similarly to limit orders—if the range is crossed, the position flips from being composed entirely of one asset, to being composed entirely of the other asset (plus accrued fees). There are two differences between this *range order* and a traditional limit order:

范围非常小的头寸的作用类似于限价单——如果超出范围，头寸就会从完全由一种资产组成，转变为完全由另一种资产组成（加上应计费用）。 此*范围订单*与传统限价订单之间有两个区别：

- There is a limit to how narrow a position’s range can be.While the price is within that range, the limit order might be partially executed.

​		头寸范围的狭窄程度是有限制的。当价格在该范围内时，限价单可能会被部分执行。

- When the position has been crossed, it needs to be with drawn. If it is not, and the price crosses back across that range, the position will be traded back, effectively reversing the trade.

​		当仓位被交叉时，需要平仓。 如果不是，并且价格重新穿越该范围，则头寸将被交易回来，从而有效地逆转交易。

![image-20240109134026900](images/image-20240109134026900.png)

## **3 ARCHITECTURAL CHANGES** 架构更改

Uniswap v3 makes a number of architectural changes, some of which are necessitated by the inclusion of concentrated liquidity,and some of which are independent improvements.

Uniswap v3 进行了许多架构更改，其中一些是由于包含集中流动性而必需的，还有一些是独立的改进。

### **3.1 Multiple Pools Per Pair**

In Uniswap v1 and v2, every pair of tokens corresponds to a single liquidity pool, which applies a uniform fee of 0*.*30% to all swaps. While this default fee tier historically worked well enough for many tokens, it is likely too high for some pools (such as pools between two stablecoins), and too low for others (such as pools that include highly volatile or rarely traded tokens).

在 Uniswap v1 和 v2 中，每对代币对应一个流动性池，该池对所有互换统一收取 0*.*30% 的费用。 虽然这种默认费用等级历来对于许多代币来说效果很好，但对于某些矿池（例如两个稳定币之间的矿池）来说可能太高，而对于其他矿池（例如包含高波动性或很少交易代币的矿池）来说可能太低。

Uniswap v3 introduces multiple pools for each pair of tokens, each with a different swap fee. All pools are created by the same factory contract. The factory contract initially allows pools to be created at three fee tiers: 0*.*05%, 0*.*30%, and 1%. Additional fee tiers can be enabled by UNI governance.

Uniswap v3 为每对代币引入了多个池，每个池都有不同的交换费用。 所有池都是由同一个工厂合约创建的。 工厂合约最初允许以三个费用等级创建矿池：0*.*05%、0*.*30% 和 1%。 UNI 治理可以启用额外的费用等级。

### **3.2 Non-Fungible Liquidity**

*3.2.1 Non-Compounding Fees.* 非复利费用

 Fees earned in earlier versions were continuously deposited in the pool as liquidity. This meant that liquidity in the pool would grow over time, even without explicit deposits, and that fee earnings compounded.

早期版本中赚取的费用会作为流动性持续存入池中。 这意味着，即使没有明确的存款，资金池中的流动性也会随着时间的推移而增长，并且费用收入也会增加。

In Uniswap v3, due to the non-fungible nature of positions, this is no longer possible. Instead, fee earnings are stored separately and held as the tokens in which the fees are paid (see Section 6.2.2).

在 Uniswap v3 中，由于头寸的不可替代性，这不再可能。 相反，费用收入单独存储并作为支付费用的代币持有（参见第 6.2.2 节）。

*3.2.2 Removal of Native Liquidity Tokens.* 移除原生流动性代币。

In Uniswap v1 and v2, the pool contract is also an ERC-20 token contract, whose tokens represent liquidity held in the pool. While this is convenient, it actually sits uneasily with the Uniswap v2 philosophy that anything that does not need to be in the core contracts should be in the periphery, and blessing one “canonical" ERC-20 implementation discourages the creation of improved ERC-20 token wrappers. Arguably, the ERC-20 token implementation should have been in the periphery, as a wrapper on a single liquidity position in the core contract.

在 Uniswap v1 和 v2 中，池合约也是 ERC-20 代币合约，其代币代表池中持有的流动性。 虽然这很方便，但它实际上与 Uniswap v2 的理念不一致，即任何不需要在核心合约中的东西都应该在外围，并且祝福一个“规范”的 ERC-20 实现会阻碍创建改进的 ERC-20 代币包装器：可以说，ERC-20 代币实施应该在外围，作为核心合约中单个流动性头寸的包装器。

The changes made in Uniswap v3 force this issue by making completely fungible liquidity tokens impossible. Due to the custom liquidity provision feature, fees are now collected and held by the pool as individual tokens, rather than automatically reinvested as liquidity in the pool.

Uniswap v3 中所做的更改使完全可替代的流动性代币成为不可能，从而迫使这个问题出现。 由于自定义流动性提供功能，费用现在由矿池作为单独的代币收取和持有，而不是自动作为流动性再投资于矿池中。

As a result, in v3, the pool contract does not implement the ERC-20 standard. Anyone can create an ERC-20 token contract in the periphery that makes a liquidity position more fungible, but it will have to have additional logic to handle distribution of, or reinvestment of, collected fees. Alternatively, anyone could create a periphery contract that wraps an individual liquidity position (including collected fees) in an ERC-721 non-fungible token.

因此，在 v3 中，矿池合约并未实现 ERC-20 标准。 任何人都可以在外围创建 ERC-20 代币合约，使流动性头寸更具可替代性，但它必须有额外的逻辑来处理所收取费用的分配或再投资。 或者，任何人都可以创建一个外围合约，将个人流动性头寸（包括收取的费用）包装在 ERC-721 不可替代代币中。

## **4 GOVERNANCE**

The factory has an owner, which is initially controlled by UNI tokenholders.2 The owner does not have the ability to halt the operation of any of the core contracts.

工厂有一个所有者，最初由 UNI 代币持有者控制。2 所有者没有能力停止任何核心合约的运行。

As in Uniswap v2, Uniswap v3 has a protocol fee that can be turned on by UNI governance. In Uniswap v3, UNI governance has more flexibility in choosing the fraction of swap fees that go to the protocol, and is able to choose any fraction $\frac{1}{N}$ where 4 ≤ *𝑁* ≤ 10,or 0. This parameter can be set on a per-pool basis.

与 Uniswap v2 一样，Uniswap v3 也有协议费用，可以通过 UNI 治理来开启。 在 Uniswap v3 中，UNI 治理在选择进入协议的掉期费用比例方面具有更大的灵活性，并且能够选择任何比例 $\frac{1}{N}$，其中 4 ≤ *𝑁* ≤ 10，或 0 . 该参数可以针对每个池进行设置。

UNI governance also has the ability to add additional fee tiers.When it adds a new fee tier, it can also define the tickSpacing (see Section 6.1) corresponding to that fee tier. Once a fee tier is added to the factory, it cannot be removed (and the tickSpacing cannot be changed). The initial fee tiers and tick spacings supported are 0*.*05% (with a tick spacing of 10, approximately 0*.*10% between initializable ticks), 0*.*30% (with a tick spacing of 60, approximately 0*.*60% between initializable ticks), and 1% (with a tick spacing of 200, approximately 2*.*02% between ticks.

UNI治理还具有添加额外费用等级的能力。当它添加新的费用等级时，它还可以定义与该费用等级相对应的tickSpacing（参见第6.1节）。 一旦费用等级被添加到工厂中，就无法将其删除（并且tickSpacing也无法更改）。 支持的初始费用等级和刻度间距为 0*.*05%（刻度间距为 10，可初始化刻度之间大约为 0*.*10%）、0*.*30%（刻度间距为 60，大约为 0*.*10%）。可初始化刻度之间的 0*.*60%）和 1%（刻度间距为 200，刻度之间大约为 2*.*02%。

Finally, UNI governance has the power to transfer ownership to another address.

最后，UNI 治理有权将所有权转移到另一个地址。

## **5 ORACLE UPGRADES**

Uniswap v3 includes three significant changes to the time-weighted average price (TWAP) oracle that was introduced by Uniswap v2.

Uniswap v3 对 Uniswap v2 引入的时间加权平均价格 (TWAP) 预言机进行了三项重大更改。

Most significantly, Uniswap v3 removes the need for users of the oracle to track previous values of the accumulator externally.

最重要的是，Uniswap v3 消除了预言机用户在外部跟踪累加器先前值的需要。

Uniswap v2 requires users to checkpoint the accumulator value at both the beginning and end of the time period for which they  wanted to compute a TWAP. Uniswap v3 brings the accumulator checkpoints into core, allowing external contracts to compute on chain TWAPs over recent periods without storing checkpoints of the accumulator value.

Uniswap v2 要求用户在想要计算 TWAP 的时间段的开始和结束时对累加器值进行检查点。 Uniswap v3 将累加器检查点引入核心，允许外部合约在最近一段时间内在链上 TWAP 上进行计算，而无需存储累加器值的检查点。

Another change is that instead of accumulating the sum of prices,allowing users to compute the arithmetic mean TWAP, Uniswap v3 tracks the sum of *log* prices, allowing users to compute the *geometric mean* TWAP.

另一个变化是，Uniswap v3 不再累加价格总和，允许用户计算算术平均 TWAP，而是跟踪 *log* 价格总和，允许用户计算 *几何平均 * TWAP。

Finally, Uniswap v3 adds a liquidity accumulator that is tracked alongside the price accumulator, which accumulates $\frac{1}{L}$ for each second. This liquidity accumulator is useful for external contracts that want to implement liquidity mining on top of Uniswap v3. It can also be used by other contracts to inform a decision on which of the pools corresponding to a pair (see section 3.1) will have the most reliable TWAP.

最后，Uniswap v3 添加了一个与价格累加器一起跟踪的流动性累加器，每秒累加 $\frac{1}{L}$。 该流动性累加器对于想要在 Uniswap v3 之上实施流动性挖矿的外部合约非常有用。 其他合约也可以使用它来决定对应于一对（参见第 3.1 节）的哪个池将具有最可靠的 TWAP。

### **5.1 Oracle Observations**

As in Uniswap v2, Uniswap v3 tracks a running accumulator of the price at the beginning of each block, multiplied by the number of seconds since the last block.

与 Uniswap v2 一样，Uniswap v3 跟踪每个块开始时价格的运行累加器，乘以自上一个块以来的秒数。

A pool in Uniswap v2 stores only the most recent value of this price accumulator—that is, the value as of the last block in which a swap occurred. When computing average prices in Uniswap v2, it is the responsibility of the external caller to provide the previous value of the price accumulator. With many users, each will have to provide their own methodology for checkpointing previous values of the accumulator, or coordinate on a shared method to reduce costs. And there is no way to guarantee that every block in which the pool is touched will be reflected in the accumulator.

Uniswap v2 中的池仅存储该价格累加器的最新值，即发生交换的最后一个块的值。 在 Uniswap v2 中计算平均价格时，外部调用者有责任提供价格累加器的先前值。 对于许多用户，每个用户都必须提供自己的方法来检查累加器的先前值，或者协调共享方法以降低成本。 并且无法保证每个触及池的块都会反映在累加器中。

In Uniswap v3, the pool stores a list of previous values for the price accumulator (as well as the liquidity accumulator described in section 5.3). It does this by automatically checkpointing the accumulator value every time the pool is touched for the first time in a block, cycling through an array where the oldest checkpoint is eventually overwritten by a new one, similar to a circular buffer.

在 Uniswap v3 中，池存储价格累加器（以及第 5.3 节中描述的流动性累加器）的先前值列表。 它通过每次在块中第一次接触池时自动对累加器值设置检查点来实现这一点，循环遍历一个数组，其中最旧的检查点最终被新的检查点覆盖，类似于循环缓冲区。

While this array initially only has room for a single checkpoint, anyone can initialize additional storage slots to lengthen the array, extending to as many as 65,536 checkpoints.3 This imposes the one-time gas cost of initializing additional storage slots for this array on whoever wants this pair to checkpoint more slots.

虽然这个阵列最初只有一个检查点的空间，但任何人都可以初始化额外的存储槽来延长阵列，最多可扩展至 65,536 个检查点。3 这会给任何人带来为此阵列初始化额外存储槽的一次性 Gas 成本。 希望这一对检查更多的位置。

The pool exposes the array of past observations to users, as well as a convenience function for finding the (interpolated) accumulator value at any historical timestamp within the checkpointed period.

该池向用户公开过去观察的数组，以及用于查找检查点期间内任何历史时间戳的（插值）累加器值的便利函数。

### **5.2 Geometric Mean Price Oracle**

Uniswap v2 maintains two price accumulators—one for the price of token0 in terms of token1, and one for the price of token1 in terms of token0. Users can compute the time-weighted arithmetic mean of the prices over any period, by subtracting the accumulator value at the beginning of the period from the accumulator at the end of the period, then dividing the difference by the number of seconds in the period. Note that accumulators for token0 and token1 are tracked separately, since the time-weighted arithmetic mean price  of token0 is not equivalent to the reciprocal of the time-weighted arithmetic mean price of token1.

Using the time-weighted *geometric* mean price, as Uniswap v3 does, avoids the need to track separate accumulators for these ratios. The geometric mean of a set of ratios is the reciprocal of the geometric mean of their reciprocals. It is also easy to implement in Uniswap v3 because of its implementation of custom liquidity provision, as described in section 6. In addition, the accumulator can be stored in a smaller number of bits, since it trackslog *𝑃* rather than *𝑃*, and log *𝑃* can represent a wide range of prices with consistent precision.4 Finally, there is a theoretical argument that the time weighted geometric mean price should be a truer representation of the average price.5

Instead of tracking the cumulative sum of the price *𝑃*, Uniswap v3 accumulates the cumulative sum of the current tick index (*𝑙𝑜𝑔*1*.*0001*𝑃*, the logarithm of price for base 1*.*0001, which is precise up to 1 basis point). The accumulator at any given time is equal to the sum of *𝑙𝑜𝑔*1*.*0001 (*𝑃*) for every second in the history of the contract:

$$

$$











