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



