# Uniswap. Part 2

## Adding more liquidity 增加更多流动性

In the previous part, we discussed that our implementation of `addLiquidity` is not complete. There was a reason for that and today we’re going to finish the function.

在上一部分中，我们讨论了“addLiquidity”的实现并不完整。 这是有原因的，今天我们将完成这个功能。

So far, the function looks like that:

到目前为止，该函数如下所示：

```solidity
function addLiquidity(uint256 _tokenAmount) public payable {
  IERC20 token = IERC20(tokenAddress);
  token.transferFrom(msg.sender, address(this), _tokenAmount);
}
```

Can you identify the problem?

你能找出问题所在吗？

The function allows to add arbitrary amounts of liquidity at any moment.

该功能允许随时添加任意数量的流动性。

As you remember, exchange rate is calculated as a ratio of reserves:

如您所知，汇率是按准备金比率计算的：
$$
Px = \frac{y}{x},Py = \frac{x}{y}
$$

Where  *Px* and *Py* are prices of ether and token; *x* and *y* are reserves of ether and token.

其中*Px*和*Py*是以太币和代币的价格； *x* 和 *y* 是以太币和代币的储备。

We also learned that swapping tokens changes reserves in a non-linear way, which affects prices, and that arbitrageurs make profit by balancing prices so they match those of big central exchanges.

我们还了解到，交换代币会以非线性方式改变储备，从而影响价格，并且套利者通过平衡价格以使其与大型中央交易所的价格相匹配来获利。

The problem with our implementation is that it allows to significantly change prices at any point of time. Or, in other words, **it doesn’t enforce current reserves ratio on new liquidity**. This is a problem because this allows price manipulations, and we want prices on decentralized exchanges to be as close to those on centralized exchanges. We want our exchange contracts to act as price oracles.

我们实施的问题是它允许在任何时间点显着改变价格。 或者，换句话说，**它不会对新的流动性强制执行当前准备金率**。 这是一个问题，因为这允许价格操纵，并且我们希望去中心化交易所的价格与中心化交易所的价格尽可能接近。 我们希望我们的交易合约能够充当价格预言机。

So, we must ensure that additional liquidity is added in the same proportion that has already established in the pool. At the same time, we want to allow liquidity in an arbitrary proportion when reserves are empty, i.e. when a pool hasn’t yet been initialized. And this is an important moment because **this is when the price is set initially**.

因此，我们必须确保额外流动性的添加比例与池中已建立的比例相同。 同时，当储备为空时，即当资金池尚未初始化时，我们希望允许任意比例的流动性。 这是一个重要的时刻，因为**这是最初确定价格的时候**。

Now, `addLiquidity` will have two branches:

现在，`addLiquidity` 将有两个分支：

1. If this is a new exchange (no liquidity), allow an arbitrary liquidity proportion when pool is empty.

   如果这是一个新的交易所（没有流动性），则在池为空时允许任意流动性比例。

2. Else, enforce established reserves proportion when there’s some liquidity.

   否则，当有一定的流动性时，执行既定的准备金比例。

The first branch remains unchanged:

第一个分支保持不变：

```solidity
if (getReserve() == 0) {
    IERC20 token = IERC20(tokenAddress);
    token.transferFrom(msg.sender, address(this), _tokenAmount);
```

The second branch is where the new code goes to:

第二个分支是新代码所在的位置：

```solidity
} else {
    uint256 ethReserve = address(this).balance - msg.value;
    uint256 tokenReserve = getReserve();
    uint256 tokenAmount = (msg.value * tokenReserve) / ethReserve;
    require(_tokenAmount >= tokenAmount, "insufficient token amount");

    IERC20 token = IERC20(tokenAddress);
    token.transferFrom(msg.sender, address(this), tokenAmount);
}
```

The only difference is that we’re not depositing all tokens provided by user but only an amount calculated based on current reserves ratio. To get the amount, we multiply the ratio (`tokenReserve / ethReserve`) by the amount of deposited ethers. Then, an error is thrown if user deposited less than this amount.

唯一的区别是，我们不会存入用户提供的所有代币，而只会存入根据当前准备金率计算的金额。 为了获得该金额，我们将比率（“tokenReserve / ethReserve”）乘以存入的以太币数量。 然后，如果用户存入的金额少于此金额，则会抛出错误。

This will preserve a price when liquidity is added to a pool.

当流动性添加到池中时，这将保留价格。

## LP-tokens

We haven’t discussed this concept but it’s a crucial part of the Uniswap design.

我们还没有讨论过这个概念，但它是 Uniswap 设计的关键部分。

We need to have a way to reward liquidity providers for their tokens. If they’re not incentivized, they won’t provide liquidity because no one would put their tokens in a third-party contract for nothing. Moreover, that reward shouldn’t be paid by us because we’d have to get investments or issue an inflationary token to fund it.

我们需要有一种方法来奖励流动性提供者的代币。如果他们没有激励，他们就不会 提供流动性，因为没有人会无缘无故地将他们的代币放入第三方合约中。而且，那个奖励 不应该由我们支付，因为我们必须获得投资或发行通货膨胀代币来为其提供资金。

The only good solution is to collect a small fee on each token swap and distribute accumulated fees among liquidity providers. This also seems pretty much fair: users (traders) pay for services (liquidity) provided by other people.

唯一好的解决方案是在每次代币交换时收取少量费用，并将累积的费用分配给流动性 供应商。这似乎也很公平：用户（交易者）为其他人提供的服务（流动性）付费。

For rewards to be fair, we need to reward liquidity providers proportionally to their contribution, i.e. the amount of liquidity they provide. If someone has provided 50% of pool liquidity, they should get 50% of accumulated fees. This makes sense, right?

为了使奖励公平，我们需要根据流动性提供者的贡献按比例奖励他们，即 他们提供的流动性。如果有人提供了 50% 的池流动性，他们应该获得累积费用的 50%。这 有道理，对吧？

Now, the task seems pretty complicated. However, there’s an elegant solution: LP-tokens.

现在，这项任务似乎相当复杂。然而，有一个优雅的解决方案：LP代币。

LP-tokens are basically ERC20 tokens issued to liquidity providers in exchange for their liquidity. In fact, **LP-tokens are shares**:

LP代币基本上是发行给流动性提供者的ERC20代币，以换取其流动性。事实上，LP代币 是股份：

1. You get LP-tokens in exchange for your liquidity.

   您可以获得 LP 代币以换取您的流动性。

2. The amount of tokens you get is proportional to the share of your liquidity in pool’s reserves.

   您获得的代币数量与您的流动性在资金池储备中的份额成正比。

3. Fees are distributed proportionally to the amount of tokens you hold.

   费用与您持有的代币数量成比例分配。

4. LP-tokens can be exchanged back for liquidity + accumulated fees.

   LP代币可以兑换回流动性+累积费用。

Ok, how will we calculate the amount of issued LP-tokens depending on the amount of provided liquidity? This is not so obvious because there a some requirements we need to meet:

好的，我们将如何根据提供的流动性量计算已发行的 LP 代币的数量？事实并非如此 很明显，因为我们需要满足一些要求：

1. Every issued share must be always correct. When someone deposits or removes liquidity after me my share must remain correct.每股已发行股票必须始终正确无误。当有人在我之后存入或移除流动性时，我的份额必须保持正确。
2. Write operations (e.g. storing new data or updating existing data in a contract) on Ethereum are very expensive. So we’d want to reduce maintenance costs of LP-tokens (i.e. we don’t want to run a scheduled job that regularly recalculates and updates shares).以太坊上的写入操作（例如，在合约中存储新数据或更新现有数据）非常昂贵。 因此，我们希望降低 LP 代币的维护成本（即我们不想定期运行计划作业 重新计算和更新份额）。

Imagine if we issue a lot of tokens (say, 1 billion) and distribute them among all liquidity providers. If we always distribute all the tokens (first liquidity provider gets 1 billion, second one gets a share of it, etc.) we are forced to recalculate issued shares, which is expensive. If we distribute only a portion of the tokens initially, then we’re risking hitting the supply limit, which will eventually force use into redistributing existing shares.

想象一下，如果我们发行大量代币（比如 10 亿个）并将它们分配给所有流动性提供者。如果我们总是 分配我们被迫的所有代币（第一个流动性提供者获得 10 亿，第二个获得份额等） 重新计算已发行股票，这很昂贵。如果我们最初只分发一部分代币，那么我们 冒着达到供应限制的风险，这最终将迫使USE重新分配现有股份。

The only good solution seems to not have supply limit at all and mint new tokens when new liquidity is added. This allows infinite growth and, if we use a proper formula, all issued shares will remain correct (will scale proportionally) when liquidity is added or removed. Luckily, inflation doesn’t reduce value of LP-tokens because they’re always backed by some amount of liquidity that doesn’t depend on the number of issued tokens.

唯一好的解决方案似乎根本没有供应限制，并在添加新的流动性时铸造新的代币。这 允许无限增长，如果我们使用适当的公式，所有已发行的股票将保持正确（将缩放 按比例）添加或删除流动性时。幸运的是，通货膨胀不会降低LP代币的价值，因为 它们总是得到一定数量的流动性的支持，这些流动性不依赖于发行的代币数量。

Now, the final piece in this puzzle: how to calculated the amount of minted LP-tokens when liquidity is deposited?

现在，这个谜题的最后一块：当流动性存入时，如何计算铸造的LP代币的数量？

The exchange contract stores reserves of ether and token, so we’d want to calculate based on reserves of both… or only one of them? Or both? I don’t know 😁 Uniswap V1 calculates the amount proportionally to the ether reserve, but Uniswap V2 allows only swaps between tokens (not between ether and token), so it’s not clear how to choose between them. Let’s stick to what Uniswap V1 does and later we’ll see how to solve this problem when there are two ERC20 tokens.

交易所合约存储了以太币和代币的储备，因此我们希望根据两者的储备进行计算......或 只有一个？还是两者兼而有之？我不知道 😁 Uniswap V1 会按以太币储备的比例计算金额，但是 Uniswap V2 只允许代币之间的交换（不允许以太币和代币之间的交换），因此尚不清楚如何在它们之间进行选择。 让我们坚持 Uniswap V1 所做的，稍后我们将看到当有两个 ERC20 代币时如何解决这个问题。

This equation shows how the amount of new LP-tokens is calculated depending on the amount of ethers deposited:

这个等式显示了如何根据存入的以太币数量计算新LP代币的数量：
$$
amountMinted = totalAmount * \frac {ethDeposited}{ethReserve}
$$




Every liquidity depositing issues LP-tokens proportionally to the share of deposited ethers in ether reserve. This is tricky, try putting different numbers in this equation and see how total amount changes. For example, what `amountMinted` and `totalAmount` would be when someone deposits `etherReserve` amount of ethers? Are issued shares still valid after that?

每次流动性存款都会根据存入的以太币在以太币储备中的份额按比例发行LP代币。 这很棘手，试着在这个等式中放入不同的数字，看看总量是如何变化的。例如 "amountMinted" "totalAmount" 当有人存入 "etherReserve" 一定数量的以太币时，会是什么？已发行股份 在那之后仍然有效？

Let’s get to the code.让我们来看看代码。

Before modifying `addLiquidity`, we need to make our Exchange contract an ERC20 contract and change its constructor:

在修改之前"addLiquidity"，我们需要将我们的交易所合约设为 ERC20 合约并更改其构造函数：

```solidity
contract Exchange is ERC20 {
    address public tokenAddress;

    constructor(address _token) ERC20("Zuniswap-V1", "ZUNI-V1") {
        require(_token != address(0), "invalid token address");

        tokenAddress = _token;
    }
```

Our LP-tokens will have a constant name and a symbol, this is how Uniswap does it. Feel free to improve this by taking the underlaying token’s name and symbol.

我们的 LP 代币将有一个恒定的名称和一个符号，这就是 Uniswap 的做法。随意通过服用来改善这一点 底层令牌的名称和符号。

Now, let’s update `addLiquidity`: when adding initial liquidity, the amount of LP-tokens issued equals to the amount of ethers deposited.

现在，让我们更新"addLiquidity"一下：当添加初始流动性时，发行的 LP 代币数量等于 沉积的醚。

```solidity
function addLiquidity(uint256 _tokenAmount)
    public
    payable
    returns (uint256)
{
    if (getReserve() == 0) {
        ...

        uint256 liquidity = address(this).balance;
        _mint(msg.sender, liquidity);

        return liquidity;
```

Additional liquidity mints LP-tokens proportionally to the amount of ethers deposited:

额外的流动性与存入的以太币数量成比例地铸造LP代币：

```solidity
 } else {
        ...

        uint256 liquidity = (totalSupply() * msg.value) / ethReserve;
        _mint(msg.sender, liquidity);

        return liquidity;
    }
}
```

Just a few lines and we now have LP-tokens!

只需几行，我们现在就有了LP代币！

## Fees 费用

We’re now ready to collect fees on swaps. Before that, we need to answer a couple of questions:

我们现在已准备好收取掉期费用。在此之前，我们需要回答几个问题：

1. Do we want to take fees in ether or tokens? Do we want to pay rewards to liquidity providers in ether or tokens?我们想以太币或代币收取费用吗？我们想以太币或代币向流动性提供者支付奖励吗？
2. How to collect a small fixed fee from each swap?如何从每次掉期中收取少量固定费用？
3. How to distribute accumulated fees to liquidity providers proportionnaly to their contribution?如何将累积费用按其贡献比例分配给流动性提供者？

Again, this might be seemed as a difficult task but we already have everything to solve it.

同样，这似乎是一项艰巨的任务，但我们已经拥有解决它的一切。

Let’s think about the last two questions. We might introduce an extra payment that’s sent along with a swap transaction. Such payments then get accumulated in a fund from which any liquidity provider can withdraw an amount proportional to their share. This sounds like a reasonable idea and, surprisingly, it’s almost done:

让我们考虑最后两个问题。我们可能会引入与掉期一起发送的额外付款 交易。然后，这些付款会累积到一个基金中，任何流动性提供者都可以从中提取一笔款项 与他们的份额成正比。这听起来像是一个合理的想法，令人惊讶的是，它几乎完成了：

1. Traders already send ethers/tokens to the exchange contract. Instead of asking for a fee we can simply subtract it from ethers/tokens that are sent to the contract.交易者已经向交易所合约发送了以太币/代币。与其要求费用，不如简单地减去费用 从发送到合约的以太币/代币。
2. We already have the fund – it’s the exchange reserves! The reserves can be used to accumulated fees. This also means that 我们已经有了资金——这是外汇储备！储备金可用于累积费用。这也 意味着**reserves will grow over time储备会随着时间的推移而增长**, so the constant product formula is not that constant! However, this doesn’t invalidate it: the fee is small compared to reserves and there’s no way to manipulate it to try to significantly change reserves.，所以恒定乘积公式并不是那么恒定！但是，事实并非如此 使其无效：与储备金相比，费用很小，并且无法操纵它以尝试显着改变储备金。
3. And now we have an answer to the first question: fees are paid in the currency of the traded in asset. Liquidity providers get a balanced amount of ethers and tokens plus a share of accumulated fees proportional to the share of their LP-tokens.现在我们对第一个问题有了答案：费用以交易资产的货币支付。流动性提供者 获得均衡数量的以太币和代币，以及与其 LP 代币份额成比例的累积费用份额。

That’s it! Let’s get to the code.

就是这样！让我们来看看代码。

Uniswap takes 0.3% in fees from each swap. We’ll take 1% just so that it’s easier to see the difference in tests. Adding fees to the contract is as easy as adding a couple of multipliers to `getAmount` function:

Uniswap 从每次掉期中收取 0.3% 的费用。我们将取 1%，以便更容易看到测试中的差异。 向合约添加费用就像添加几个乘数一样简单 "getAmount" ：

```solidity
function getAmount(
  uint256 inputAmount,
  uint256 inputReserve,
  uint256 outputReserve
) private pure returns (uint256) {
  require(inputReserve > 0 && outputReserve > 0, "invalid reserves");

  uint256 inputAmountWithFee = inputAmount * 99;
  uint256 numerator = inputAmountWithFee * outputReserve;
  uint256 denominator = (inputReserve * 100) + inputAmountWithFee;

  return numerator / denominator;
}

```

Since Solidity doesn’t support floating point division, we have to use a trick: both numerator and denominator are multiplied by a power of 10, and fee is subtracted from the multiplier in the numerator. Normally, we would calculate it like that:

由于 Solidity 不支持浮点除法，我们必须使用一个技巧：分子和分母都是 乘以 10 的幂，然后从分子中的乘数中减去费用。通常，我们会计算它 诸如此类： 



In Solidity, we have to do it like that:

在 Solidity 中，我们必须这样做：
$$
amountWithFee = amount * \frac {100-fee}{100}
$$


## Removing liquidity 消除流动性

Finally, last function on our list: `removeLiquidity`.

最后，我们列表中的最后一个函数："removeLiquidity".

To remove liquidity we can again use LP-tokens: we don’t need to remember amounts deposited by each liquidity provider and can calculate the amount of removed liquidity based on an LP-tokens share.

为了消除流动性，我们可以再次使用LP代币：我们不需要记住每个流动性提供者存入的金额 并且可以根据 LP 代币份额计算移除的流动性量。

```solidity
unction removeLiquidity(uint256 _amount) public returns (uint256, uint256) {
  require(_amount > 0, "invalid amount");

  uint256 ethAmount = (address(this).balance * _amount) / totalSupply();
  uint256 tokenAmount = (getReserve() * _amount) / totalSupply();

  _burn(msg.sender, _amount);
  payable(msg.sender).transfer(ethAmount);
  IERC20(tokenAddress).transfer(msg.sender, tokenAmount);

  return (ethAmount, tokenAmount);
}
```

When liquidity is removed, it’s returned in both ethers and tokens and their amounts are, of course, balanced. This is the moment that causes [impermanent loss](https://pintail.medium.com/uniswap-a-good-deal-for-liquidity-providers-104c0b6816f2): the ratio of reserves changes over time following changes in their prices in USD. When liquidity is removed the balance can be different from what it was when liquidity was deposited. This means that you would get different amounts of ethers and tokens and their total price might be lower than if you have just held them in a wallet.

当流动性被移除时，它会以以太币和代币的形式返回，当然，它们的数量是平衡的。这是 造成无常损失的时刻： 储备金的比率随着美元价格的变化而随时间变化而变化。当流动性被移除时，余额 可能与存入流动性时的情况不同。这意味着您将获得不同数量的 以太币和代币及其总价可能低于您刚刚将它们放在钱包中的价格。



To calculate the amounts we multiply reserves by the share of LP-tokens:

要计算储备金乘以 LP 代币份额的金额：
$$
removedAmount = reserve * \frac{amountLP}{totalAmountLP}
$$


Notice that LP-tokens are burnt each time liquidity is removed. LP-tokens are only backed by deposited liquidity.

请注意，每次移除流动性时，LP代币都会被烧毁。LP代币仅由存入的流动性支持。

## LP reward and impermanent loss demonstration 奖励和无常损失演示

Let’s write a test that reproduces the full cycle of adding liquidity, swapping tokens, accumulating fees, and removing liquidity:

让我们写一个测试，再现添加流动性、交换代币、累积费用和删除的完整周期 流动性：

1. First, liquidity provider deposits 100 ethers and 200 tokens. This makes 1 token being equal to 0.5 ethers and 1 ether being equal to 2 tokens.

   首先，流动性提供者存入 100 个以太币和 200 个代币。这使得 1 个代币等于 0.5 个以太币和 1 个 以太币等于 2 个代币。

   ```solidity
   exchange.addLiquidity(toWei(200), { value: toWei(100) });
   ```

   

2. A user swaps 10 ethers and expects to get at least 18 tokens. In fact, they got 18.0164 tokens. It includes slippage (traded amounts are relatively big) and the 1% fee.

   用户交换 10 个以太币，并希望获得至少 18 个代币。事实上，他们得到了 18.0164 个代币。它包括滑点 （交易金额比较大）和1%的费用。

   ```solidity
   exchange.connect(user).ethToTokenSwap(toWei(18), { value: toWei(10) });
   ```

3. Liquidity provider then removes their liquidity:

   然后，流动性提供者会移除他们的流动性：

   ```solidity
   exchange.removeLiquidity(toWei(100));
   ```

   

4. Liquidity provider got 109.9 ethers (transaction fees included) and 181.9836 tokens. As you can see, these numbers are different from those that were deposited: we got the 10 ethers traded in by the user but had to give 18.0164 tokens in exchange. However, that amount includes the 1% fee the user has paid to us. Since the liquidity provider has provided all the liquidity, they got all the fees.

   流动性提供者获得了 109.9 个以太币（包括交易费用）和 181.9836 个代币。 正如你所看到的，这些数字与存入的数字不同：我们得到了 10 个以太币的交易 用户，但必须提供 18.0164 个代币作为交换。但是，该金额包括用户向我们支付的 1% 费用。 由于流动性提供者提供了所有流动性，因此他们获得了所有费用。



## Conclusion 结论

That was a big post! Hopefully LP-tokens are not a mystery for you anymore and Uniswap is easy as pie (not cake 😉).

那是一个很大的帖子！ 希望 LP 代币对您来说不再是谜，Uniswap 就像馅饼一样容易（不是蛋糕😉）。

However, we’re not done yet: Exchange contract is now finished, but we also need to implement Factory contract, which serves as a registry of exchanges and a bridge that connects multiple exchanges and makes token-to-token swaps possible. We’ll implement it in the next part!

但是，我们还没有完成：Exchange 合约现在已经完成，但我们还需要实现 Factory 合约，它服务 作为交易所的注册处和连接多个交易所并使代币到代币交换成为可能的桥梁。 我们将在下一部分中实现它！











