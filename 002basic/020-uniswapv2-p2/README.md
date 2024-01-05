# Programming DeFi: Uniswap V2. Part 2
## Tokens swapping

At this point, we have everything we need to perform actual tokens exchanging. Let’s think how we will implement it.

在这一点上，我们拥有执行实际代币交换所需的一切。让我们考虑一下我们将如何实现它。

Exchanging means giving away some amount of Token A in exchange for Token B. But we need some kind of a mediator that:

交换意味着放弃一定数量的代币 A 以换取代币 B。但我们需要某种调解人：

1. Provides actual exchange rates.提供实际汇率。
2. Guarantees that all exchanges are paid in full, i.e. all exchanges are made under correct rate.保证所有交换都全额支付，即所有交换都以正确的价格进行。

We learned something about pricing of DEXes when we were working on liquidity provision: it’s the amount of liquidity in a pool that defines exchange rates. In [the Uniswap V1 series](https://jeiwan.net/posts/programming-defi-uniswap-1/), I explained in details how the constant product formula works and what is the main condition for a successful swap. Namely: **the product of reserves after a swap must be equal or greater than that before the swap**. That’s it: the constant product must remain the same, no matter what’s the amount of reserves in pool. This is basically the only condition we must guarantee and, surprisingly, this condition frees us from calculating swap price.

当我们研究流动性提供时，我们学到了一些关于 DEX 定价的知识：它是流动性的数量 在定义汇率的池中。在 Uniswap V1 系列中， 我详细解释了恒定乘积公式是如何工作的，以及成功交换的主要条件是什么。 即：掉期后的准备金乘积必须等于或大于掉期前的准备金。就是这样： 恒定乘积必须保持不变，无论池中的储备量是多少。这基本上是唯一的 我们必须保证的条件，令人惊讶的是，这个条件使我们免于计算掉期价格。

As I mentioned in the introduction, the pair contract is a core contract, which means it must be as low-level and minimal as possible. This also affects how we send tokens to the contract. There a two ways of transferring tokens to someone:

正如我在引言中提到的，货币对合约是核心合约，这意味着它必须是低级和最小的 尽可能。这也影响了我们向合约发送代币的方式。有两种方法可以将代币转移给某人：

1. By calling `transfer` method of the token contract and passing recipient’s address and the amount to be sent.

   通过调用 `transfer` 代币合约的方法并传递接收者的地址和要发送的金额。

2. By calling `approve` method to allow the other user or contract to transfer some amount of your tokens to their address. The other party would have to call `transferFrom` to get your tokens. You pay only for approving a certain amount; the other party pays for the actual transfer.

   通过调用 `approve`方法允许其他用户或合约将一定数量的代币转移到他们的 地址。对方必须调用transferFrom才能获得您的代币。您只需为批准某个 量;另一方支付实际转账费用。

The approval pattern is very common in Ethereum applications: dapps ask users to approve spending of the maximum amount so users don’t need to call `approve` again and again (which costs gas). This improves user experience. And this is not what we’re looking for at this moment. So we’ll go with the manual transferring to the pair contract.

批准模式在以太坊应用程序中非常常见：dapp 要求用户批准最大金额的支出 因此，用户不需要一次又一次地调用 "approve" （这需要花费 gas）。这改善了用户体验。这是 不是我们此刻想要的。因此，我们将手动转移到货币对合约。

Let’s get to code!

The function takes two output amounts, one for each token. These are the amounts that caller wants to get in exchange for their tokens. Why doing it like that? Because we don’t even want to enforce the direction of swap: caller can specify either of the amounts or both of them, and we’ll just perform necessary checks.

该函数接受两个输出量，每个标记一个。这些是呼叫者想要获得的金额 为他们的代币。为什么要这样做？因为我们甚至不想强制执行交换的方向：调用者可以 指定其中一项金额或两项金额，我们将只执行必要的检查。

```solidity
function swap(
    uint256 amount0Out,
    uint256 amount1Out,
    address to
) public {
    if (amount0Out == 0 && amount1Out == 0)
        revert InsufficientOutputAmount();

    ...
```

Next, we need to ensure that there are enough of reserves to send to user.

接下来，我们需要确保有足够的储备金发送给用户。

```solidity
...

    (uint112 reserve0_, uint112 reserve1_, ) = getReserves();

    if (amount0Out > reserve0_ || amount1Out > reserve1_)
        revert InsufficientLiquidity();

    ...
```

Next, we’re calculating token balances of this contract minus the amounts we’re expected to send to the caller. At this point, it’s expected that the caller has sent tokens they want to trade in to this contract. So, either or both of the balances is expected to be greater than corresponding reserve.

接下来，我们将计算该合约的代币余额减去我们预期发送给调用方的金额。在此 点，预计调用方已将他们想要交易的代币发送到该合约。因此，其中一项或两项 预计余额将大于相应的准备金。

```solidity
...
    uint256 balance0 = IERC20(token0).balanceOf(address(this)) - amount0Out;
    uint256 balance1 = IERC20(token1).balanceOf(address(this)) - amount1Out;
    ...
```

And here’s the constant product check we talked about above. We expect that this contract token balances are different than its reserves (the balances will be saved to reserves soon) and we need to ensure that their product is equal or greater than the product of current reserves. If this requirement is met then:

这是我们上面谈到的持续产品检查。我们预计该合约代币余额与其不同 储备金（余额将很快存入储备金），我们需要确保其乘积等于或大于 当前储备的乘积。如果满足此要求，则：

1. The caller has calculated the exchange rate correctly (including slippage).调用者已正确计算汇率（包括滑点）。
2. The output amount is correct.输出量正确。
3. The amount transferred to the contract is also correct.转移到合同的金额也是正确的。

```solidity
 ...
    if (balance0 * balance1 < uint256(reserve0_) * uint256(reserve1_))
        revert InvalidK();
    ...
```

It’s now safe to transfer tokens to the caller and to update the reserves. The swap is complete.

现在可以安全地将令牌转移给调用方并更新储备金。交换完成。

```solidity
_update(balance0, balance1, reserve0_, reserve1_);

    if (amount0Out > 0) _safeTransfer(token0, to, amount0Out);
    if (amount1Out > 0) _safeTransfer(token1, to, amount1Out);

    emit Swap(msg.sender, amount0Out, amount1Out, to);
}
```

Feel free to write tests for this function. And don’t forget about the case when both output amounts are specified. 😉

随意为这个函数编写测试。并且不要忘记指定两个输出量的情况。😉

> Keep in mind that this implementation is not complete: the contract doesn’t collect exchange fees and, as a result, liquidity providers don’t get profit on their assets. We’ll fill this gap after implementing price calculation.
>
> 请记住，这种实现并不完整：合约不收取交易所费用，因此， 流动性提供者不会从他们的资产中获得利润。我们将在实施价格计算后填补这一空白。

## Re-entrancy attacks and protection重入攻击和保护

One of the most common attacks on Ethereum smart contracts is re-entrancy attack. This attack is possible when contract makes external calls without doing necessary checks or updating state. Attacker can trick the contract into calling attacker’s contract, which, in its turn, calls the attacked contract again (but usually it calls it many times). As a result, that second call (which re-enters the contract) exploits incorrectly updated state of the contract, which causes lost of funds (that’s the main goal of the attack).

对以太坊智能合约最常见的攻击之一是重入攻击。当收缩时，这种攻击是可能的 在不进行必要的检查或更新状态的情况下进行外部调用。攻击者可以诱使合约调用 攻击者的合约，反过来又调用被攻击的合约（但通常它会多次调用）。作为 结果，第二次调用（重新进入合约）利用了合约的错误更新状态，这会导致 资金损失（这是攻击的主要目标）。

In the pair contract, there’s `safeTransfer` calls in `swap` function–the contract sends tokens to caller. Re-entrancy attacks are targeted at exactly such calls. It’s very naive to assume that the called `transfer` method does exactly what we expect it to do. In fact, nothing forces a token contract to implement any of the ERC20 functions according to the standard–they can do whatever their developers programmed them to do.

在对合约中，"safeTransfer"函数中有"swap"调用——合约向调用者发送代币。重入 攻击正是针对此类调用的。假设被调用的方法完全执行是非常幼稚的 "transfer" 我们期望它做什么。事实上，没有什么能强迫代币合约根据 标准——他们可以做开发人员编程他们要做的任何事情。

There are two common ways of preventing re-entrancy attacks:

有两种常见的方法可以防止重入攻击：

1. Using a re-entrancy guard. 使用重入防护
   For example, [the one from OpenZeppelin contracts](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/security/ReentrancyGuard.sol). UniswapV2 uses [its own implementation](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol#L30-L36), since it’s not hard to implement. The main idea is to set a flag when a function is called and not allow to call the function when the flag is set; the flag is unset when the call is done. This mechanism doesn’t allow to call a function when it’s being called (since transactions are applied atomically, there’s only caller at a time and locking a function won’t make it inaccessible for other callers).

   主要思想是在调用函数时设置一个标志，在设置标志时不允许调用该函数;这 调用完成时，标志未设置。此机制不允许在调用函数时调用函数（因为 事务是原子式应用的，一次只有调用者，锁定函数不会使其无法访问 对于其他呼叫者）。

2. Following the [Checks, Effects, Interactions Pattern](https://fravoll.github.io/solidity-patterns/checks_effects_interactions.html).
   The pattern enforces a strict order of operations in a contract function: first, all necessary checks are made to ensure the function is working with correct state. Second, the function updates its own state according to its logic. Finally, the function makes external calls. Such order guarantees that every function call is made when function’s state is finalized and correct, i.e. there are no pending state updates.

   该模式在合约函数中强制执行严格的操作顺序：首先，对 确保函数在正确的状态下工作。其次，函数根据其逻辑更新自己的状态。 最后，该函数进行外部调用。 这样的顺序保证了每个函数调用都是在函数的状态最终确定并正确时进行的，即有 没有挂起的状态更新。

Is our implementation of `swap` vulnerable? Can one trick it into sending all its reserves to caller? Theoretically, yes, since it depends on third-party contracts (tokens), and either of the token contracts can provide it wrong balances to trick it into sending all its reserves to caller. However, if a token contract is malicious, a re-entrancy attack is a lesser evil, and an exploit would still be possible without it.

我们的实施是否 "swap" 易受攻击？有人可以欺骗它将其所有储备发送给呼叫者吗？从理论上讲，是的， 因为它依赖于第三方合约（代币），而任何一个代币合约都可能提供错误 余额以诱骗它将其所有储备发送给呼叫者。但是，如果代币合约是恶意的，则重新进入 攻击是一种较小的邪恶，如果没有它，仍然可以利用攻击。

## Price oracle价格预言机

The idea of oracles, bridges that connect blockchain with off-chain services so that real-world data can be queried from smart contracts, has been around for quite a while. Chainlink, one of the biggest (or the biggest one?) oracle networks, was created in 2017 and, today, it’s a crucial part of many DeFi applications.

预言机的想法，将区块链与链下服务连接起来的桥梁，以便可以从中查询真实世界的数据 智能合约已经存在了很长一段时间。Chainlink，最大（或最大的）预言机网络之一， 创建于 2017 年，如今，它已成为许多 DeFi 应用程序的重要组成部分。

Uniswap, while being an on-chain application, can also serve as an oracle. Each Uniswap pair contract that is regularly used by traders also attracts arbitrageurs, who make money on minimizing price differences between exchanges. Arbitrageurs make Uniswap prices as close to those on centralized exchanges as possible, which can also be seemed as feeding prices from centralized exchanges to blockchain. Why not use this fact to turn the pair contract into a price oracle? And this is what was done in Uniswap V2.

Uniswap 虽然是一个链上应用程序，但也可以充当预言机。每个 Uniswap 对合约 交易者经常使用也吸引了套利者，他们通过最小化交易所之间的价格差异来赚钱。 套利者使 Uniswap 的价格尽可能接近中心化交易所的价格，这也可以看作是 将价格从中心化交易所提供给区块链。为什么不利用这个事实将货币对合约变成价格 神谕？这就是 Uniswap V2 中所做的。

The kind of prices provided by the price oracle in Uniswap V2 is called **time-weighted average price**, or TWAP. It basically allows to get an average price between two moments in time. To make this possible, the contract stores accumulated prices: before every swap, it calculates current marginal prices (excluding fees), multiplies them by the amount of seconds that has passed since last swap, and adds that number to the previous one.

Uniswap V2 中价格预言机提供的价格类型称为时间加权平均价格，或 TWAP。它 基本上允许在两个时刻之间获得平均价格。为了实现这一点，合约存储 累计价格：在每次掉期之前，它计算当前边际价格（不包括费用），乘以 自上次交换以来经过的秒数，并将该数字与前一个交换相加。

I mentioned marginal price in the previous paragraph–this is simply a relation of two reserves:

我在上一段中提到了边际价格——这只是两个储备金的关系：
$$
price_0 =  \dfrac {reserve_1} {reserve_0}
$$


or 
$$
price_1 =  \dfrac {reserve_0} {reserve_1}
$$


For the price oracle functionality, Uniswap V2 uses marginal prices, which don’t include slippage and swap fee and also don’t depend on swapped amount.

对于价格预言机功能，Uniswap V2 使用边际价格，其中不包括滑点和掉期费用，也 不要依赖掉期金额。

Since Solidity doesn’t support float point division, calculating such prices can be tricky: if, for example, the ratio of two reserves is $ \frac 2 3$, then the price is 0. We need to increase precision when calculating marginal prices, and Unsiwap V2 uses [UQ112.112 numbers](https://en.wikipedia.org/wiki/Q_(number_format)) for that.

由于 Solidity 不支持浮点除法，因此计算此类价格可能很棘手：例如，如果 两个储备是 $\frac{2}{3}$，那么价格是 0。在计算边际价格时，我们需要提高精度， Unsiwap V2 为此使用 UQ112.112 数字。

UQ112.112 is basically a number that uses 112 bits for the fractional part and 112 for the integer part. 112 bits were chosen to make storage of the reserve state variables more optimal (more on this in the next section)-that’s why the variables use type `uint112`. Reserves, on the other hand, are stored as the integer part of a UQ112.112 number–this is why they’re multiplied by `2**112` before price calculation. Check out `UQ112x112.sol` for more details, it’s very simple.

UQ112.112 基本上是一个使用 112 位作为小数部分和 112 位作为整数部分的数字。 112 位是 选择使储备状态变量的存储更加优化（下一节将对此进行更多介绍）-这就是为什么 变量使用类型 "uint112"。另一方面，储备金存储为 UQ112.112 数字的整数部分——这是 为什么在 "2**112" 价格计算之前将它们乘以。查看 "UQ112x112.sol" 更多详细信息，非常 简单。

I hope this all will be clearer for you from code, so let’s implement prices accumulation. We only need to add one state variable:

我希望这一切能从代码中让你更清楚，所以让我们实现价格累积。我们只需要添加一个状态 变量：

```solidity
uint32 private blockTimestampLast;
```

Which will store last swap (or, actually, reserves update) timestamp. And then we need to modify the reserves updating function:

它将存储上次交换（或者实际上，保留更新）时间戳。然后我们需要修改储备更新 功能：

```soli
function _update(
    uint256 balance0,
    uint256 balance1,
    uint112 reserve0_,
    uint112 reserve1_
) private {
    ...
    unchecked {
        uint32 timeElapsed = uint32(block.timestamp) - blockTimestampLast;

        if (timeElapsed > 0 && reserve0_ > 0 && reserve1_ > 0) {
            price0CumulativeLast +=
                uint256(UQ112x112.encode(reserve1_).uqdiv(reserve0_)) *
                timeElapsed;
            price1CumulativeLast +=
                uint256(UQ112x112.encode(reserve0_).uqdiv(reserve1_)) *
                timeElapsed;
        }
    }

    reserve0 = uint112(balance0);
    reserve1 = uint112(balance1);
    blockTimestampLast = uint32(block.timestamp);

    ...
}
```

`UQ112x112.encode` multiplies a `uint112` value by `2**112`, which makes it a `uint224` value. Then, it’s divided by the other reserve and multiplied by `timeElapsed`. The result is **added** to the currently stored one–this makes it cumulative. Notice the `unchecked` block–we’ll discuss it shortly.

"UQ112x112.encode" 将" uint112"值乘以 "2**112"，使它成为 "uint224" 值。然后，除以 另一个储备并乘以 "timeElapsed"。结果被添加到当前存储的结果中 - 这使得它 累积。请注意 "unchecked" 这个块——我们稍后会讨论它。



## Storage optimization存储优化

What’s that weird `uint112` type? Why not using `uint256`? The answer is: gas optimization.

那是什么奇怪的 "uint112" 类型？为什么不使用"uint256"？答案是：gas优化。

Every EVM operation consumes some amount of gas. Simple operations, like arithmetics ones, consume very little gas, but there are operations that consume a lot of gas. The most expensive one is `SSTORE`–saving value to contract storage. Its counterpart, `SLOAD`, is also expensive. So, it’s beneficial to users if smart contract developers try to optimize gas consumption of their contracts. Using `uuint112` for the reserve variables serves exactly this purpose.

每个 EVM 操作都会消耗一定量的 gas。简单的运算，如算术运算，消耗很少gas，但有些操作会消耗大量gas。最昂贵的是 "SSTORE"– 为合同节省价值 存储。它的对应物"SLOAD"，也很昂贵。因此，如果智能合约开发人员尝试这样做，这对用户是有益的 优化其合约gas消耗。使用 "uuint112" for reserve 变量正是为此目的而使用的。

Take a look at how we laid out the variables:

看看我们是如何布置变量的：

```solidity
address public token0;
address public token1;

uint112 private reserve0;
uint112 private reserve1;
uint32 private blockTimestampLast;

uint256 public price0CumulativeLast;
uint256 public price1CumulativeLast;
```

This is critical–they must go in exactly this order. The reason is that each state variable corresponds to a certain storage slot, and EVM uses 32-byte storage slots (every storage slot is exactly 32 bytes). When you read a state variable value, it’s get read from the storage slot this variable is linked to. Every `SLOAD` call reads 32 bytes at a time, and every `SSTORE` call writes 32 bytes at a time. Since these are expensive operations, we’d really want to reduce the number of storage reads and writes. And this is where proper laying out of state variables might help.

这很关键——它们必须完全按照这个顺序进行。原因是每个状态变量对应一个 存储插槽，EVM 使用 32 字节存储插槽（每个存储插槽正好是 32 字节）。读取状态变量时 值，则从此变量链接到的存储槽中读取。每次 "SLOAD" 调用一次读取 32 个字节， 每次 "SSTORE" 调用一次写入 32 个字节。由于这些都是昂贵的操作，我们真的希望减少 存储读取和写入次数。这就是正确布局状态变量可能会有所帮助的地方。

What if there are several consecutive state variables that take less than 32 bytes? Do we need to read each of them separately? It turns out, no. **EMV packs neighbor variables that are less than 32 bytes**.

如果有几个连续的状态变量占用的时间少于 32 个字节，该怎么办？我们需要阅读它们中的每一个吗 分别？事实证明，没有。EMV 打包小于 32 字节的邻居变量。

Take another look at our state variables:

再看看我们的状态变量：

1. First two are `address` variables. `address` takes 20 bytes, and two addresses take 40 bytes, which means they have to take separate storage slots. They cannot be stored in one slot since they simply won’t fit.
2. Two `uint112` variables and one `uint32`–this looks interesting: 112+112+32=256! This means they can fit in one storage slot! This is why `uint112` was chosen for reserves: the reserves variables are always read together, and it’s better to load them from storage at once, not separately. This saves one `SLOAD` operation, and since reserves are used very often, this is huge gas saving.
3. Two `uint256` variables. These cannot be packed because each of them takes a full slot.

It’s also important that the two `uint112` variables go after a variable that takes a full slot–this ensures that the first of them won’t be packed in the previous slot.

同样重要的是，这两个"uint112"变量要跟随一个占用完整插槽的变量——这确保了 第一个不会打包在上一个插槽中。

## Integer overflow and underflow整数上溢和下溢

We wrapped accumulated prices calculation in `unchecked`–why?

我们将累计价格计算包含在 "unchecked"- 为什么？

Another popular vulnerability of smart contracts is integer overflow or underflow. The maximum value of a `uint256` integer is 2256−12256−1 and the minimum value is 0. Integer overflow means increasing the value of an integer variable so it’s greater than the maximum one, this will result in an overflow: the value wraps and starts at 0. E.g.:
$$
uint256(2^{256}-1)+1=0
$$
Similarly, subtracting a number from 0 will result in a very big number, e.g.:
$$
uint256(0)-1 = 2^{256}-1
$$


Until version 0.8.0, Solidity hadn’t checked for overflows and underflows, and developers came up with a library: [SafeMath](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/math/SafeMath.sol). Nowadays, this library is not needed anymore as Solidity now throws exceptions when an overflow or underflow is detected.

在 0.8.0 版本之前，Solidity 还没有检查溢出和下溢，开发人员想出了一个库： 安全数学。 如今，不再需要这个库，因为 Solidity 现在在检测到溢出或下溢时会抛出异常。

Solidity 0.8.0 also introduced `unchecked` block which, as the name suggests, disables the overflow/underflow detection within its boundaries.

Solidity 0.8.0 还引入了 "unchecked" 块，顾名思义，它禁用了溢出/下溢 在其边界内进行检测。

Let’s return to our code.

We’re using `unchecked` block when calculating `timeElapsed` and accumulated prices. This seems to be bad for the security of the contract, but it’s expected that timestamp and accumulated prices overflow: nothing bad will happen when either of them overflows. We want them to overflow without throwing an error so they could function properly.

我们在 "unchecked" 计算 "timeElapsed" 和累积价格时使用块。这似乎对 合约的安全性，但预计时间戳和累积价格会溢出：当 它们中的任何一个都会溢出。我们希望它们在不引发错误的情况下溢出，以便它们可以正常运行。

Such cases are rare, and the overflow/underflow detection should almost never be disabled.

这种情况很少见，几乎绝不应禁用溢出/下溢检测。

## Safe transfer安全转移

You probably have noticed the strange way of sending tokens we’re using:

您可能已经注意到我们使用的发送令牌的奇怪方式：

```solidity
function _safeTransfer(
  address token,
  address to,
  uint256 value
) private {
  (bool success, bytes memory data) = token.call(
    abi.encodeWithSignature("transfer(address,uint256)", to, value)
  );
  if (!success || (data.length != 0 && !abi.decode(data, (bool))))
    revert TransferFailed();
}
```

Why not call `transfer` method directly on ERC20 interface?

为什么不 "transfer" 直接在 ERC20 接口上调用方法？

In the pair contract, when doing token transfers, we always want to be sure that they’re successful. According to ERC20, `transfer` method must return a boolean value: `true`, when it’s successful; `fails`, when it’s not. Most of tokens implement this correctly, but some tokens don’t–they simply return nothing. Of course, we cannot check token contract implementation and cannot be sure that token transfer was in fact made, but we at least can check transfer result. And we don’t want to continue if a transfer has failed.

在配对合约中，在进行代币转移时，我们总是希望确保它们成功。根据 ERC20， "transfer" 方法必须返回一个布尔值："true"，当它成功时; "fails"，但事实并非如此。大多数代币 正确地实现了这一点，但有些令牌没有——它们只是不返回任何内容。当然，我们不能检查代币合约 实现，并且无法确定是否确实进行了令牌转移，但我们至少可以检查转移结果。而我们 如果传输失败，则不想继续。

`call` here is an `address` [method](https://docs.soliditylang.org/en/latest/types.html#members-of-addresses)–this is a low-level function that gives us a more fine-grained control over a contract call. In this specific case, it allows us to get a result of a transfer no matter whether the `transfer` method return one or not.

"call" 这里有一个"address"方法——这是一个 低级函数，使我们能够更精细地控制合约调用。在这种特定情况下，它允许我们 获取传输结果，无论 "transfer" 方法是否返回一个。

## Conclusion结论

That’s it for today! I hope this part clarifies a lot in our implementation. Next time we’ll continue with adding new features and contracts.

今天就到这里！我希望这部分在我们的实现中能澄清很多。下次我们将继续添加新的 功能和合同。

## Links链接

1. [Source code of part 2第 2 部分的源代码](https://github.com/Jeiwan/zuniswapv2/tree/part_2)
2. [UniswapV2 WhitepaperUniswapV2 白皮书](https://uniswap.org/whitepaper.pdf) – worth reading and re-reading. – 值得一读和重读。
3. [Layout of State Variables in Storage存储中状态变量的布局](https://docs.soliditylang.org/en/latest/internals/layout_in_storage.html)
4. [Q (number format)Q（数字格式）](https://en.wikipedia.org/wiki/Q_(number_format))
5. [Check Effects Interactions Pattern检查效果交互模式](https://fravoll.github.io/solidity-patterns/checks_effects_interactions.html)
6. [Checked or Unchecked Arithmetic已检查或未检查的算术](https://docs.soliditylang.org/en/latest/control-structures.html#checked-or-unchecked-arithmetic)











