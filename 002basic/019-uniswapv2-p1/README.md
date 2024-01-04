



## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```

## create project

```shell
forge init 019-uniswapv2-p1  --no-commit --force
```

```shell
cd 019-uniswapv2-p1
```

## forge build

```shell
forge build
```

```shell
[⠊] Compiling...
[⠊] Installing Solc version 0.8.23
[⠆] Successfully installed Solc 0.8.23
[⠰] Compiling 24 files with 0.8.23
[⠑] Solc 0.8.23 finished in 5.11s
Compiler run successful!

```

## forge test

```shell
forge test

```

```shell
[⠆] Compiling...
No files changed, compilation skipped

Running 2 tests for test/Counter.t.sol:CounterTest
[PASS] testFuzz_SetNumber(uint256) (runs: 256, μ: 27009, ~: 28409)
[PASS] test_Increment() (gas: 28379)
Test result: ok. 2 passed; 0 failed; 0 skipped; finished in 22.97ms
 
Ran 1 test suites: 2 tests passed, 0 failed, 0 skipped (2 total tests)
```

```shell
forge install rari-capital/solmate --no-commit
```



# Programming DeFi: Uniswap V2. Part 1

## Introduction介绍

Uniswap is a decentralized exchange running on the Ethereum blockchain. It’s fully automated, not managed, and decentralized. It has come through multiple iterations of development: first version was launched in November 2018; second version–in May 2020; and final, third, version was launched in March 2021.

Uniswap 是一个运行在以太坊区块链上的去中心化交易所。它是完全自动化的，而不是托管的，并且是去中心化的。 它经历了多次开发迭代：第一个版本于 2018 年 11 月推出;第二版——2020 年 5 月; 最终的第三个版本于 2021 年 3 月推出。

## Tooling工具

In this series, I’ll be using [Foundry](https://github.com/gakonst/foundry/) for contracts developing and testing. Foundry is a modern Ethereum toolkit written in Rust by [Georgios Konstantopoulos](https://twitter.com/gakonst). It’s much faster than Hardhat and, what’s specifically useful to us, it allows to write tests in Solidity. Yes, we’ll use Solidity for both writing contracts and testing them and you’ll see that this is much cleaner and handier than writing tests in JS.

在本系列中，我将使用 Foundry 进行合约开发和测试。Foundry 是由 Georgios Konstantopoulos 用 Rust 编写的现代以太坊工具包。它要快得多 比 Hardhat 更有用，它允许在 Solidity 中编写测试。是的，我们将同时使用 Solidity 编写合约并测试它们，你会发现这比用 JS 编写测试更干净、更方便。

It’s also worth noting that many things have changes since 2020, when Uniswap V2 was launched. For example, `SafeMath` library has become obsolete since the release of Solidity 0.8, which introduced native overflow checks. So, we’re building a modern version of Uniswap, so to say.

还值得注意的是，自 2020 年 Uniswap V2 推出以来，许多事情都发生了变化。例如 "SafeMath" 自 Solidity 0.8 发布以来，库已经过时，它引入了原生溢出检查。所以，我们是 可以这么说，构建现代版本的 Uniswap。

## Architecture of Uniswap V2   Uniswap V2 架构

The core architectural idea of Uniswap V2 is pooling: liquidity providers are available to stake their liquidity in a contract; that staked liquidity allows anyone else to trade in a decentralized way. Similarly to Uniswap V1, traders pay a small fee, which gets accumulated in a contract and then gets shared by all liquidity providers.

Uniswap V2 的核心架构思想是池化：流动性提供者可以将其流动性质押在 合同;质押的流动性允许其他任何人以去中心化的方式进行交易。与 Uniswap V1 类似，交易者 支付少量费用，该费用在合约中累积，然后由所有流动性提供者共享。

The core contract of Uniswap V2 is [UniswapV2Pair](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol). “Pair” and “Pool” are interchangeable terms, they mean the same thing–`UniswapV2Pair` contract. That contract’s main purpose is to accept tokens from users and use accumulated reserves of tokens to perform swaps. This is why it’s a pooling contract. Every `UniswapV2Pair` contract can pool only one pair of tokens and allows to perform swaps only between these two tokens–this is why it’s called “pair”.

Uniswap V2 的核心合约是 UniswapV2Pair。 “Pair”和“Pool”是可以互换的术语，它们的意思是一样的——"UniswapV2Pair"合约。该合约的主要 目的是接受来自用户的代币，并使用累积的代币储备进行交换。这就是为什么它是一个池化 合约。每 "UniswapV2Pair" 份合约只能汇集一对代币，并且只允许在这些代币之间执行交换 两个代币——这就是为什么它被称为“对”。

The codebase of Uniswap V2 contracts is split into two repositories:

1. [core](https://github.com/Uniswap/v2-core), and
2. [periphery](https://github.com/Uniswap/v2-periphery).

UniswapV2合约的代码库分为两个存储库：

1.core核心

2.外围

The core repository stores these contracts:

核心存储库存储以下合约：

1. `UniswapV2ERC20` – an extended ERC20 implementation that’s used for LP-tokens. It additionally implements [EIP-2612](https://eips.ethereum.org/EIPS/eip-2612) to support off-chain approval of transfers.

   用于 LP 代币的扩展 ERC20 实现。它还实现了 [EIP-2612](https://eips.ethereum.org/EIPS/eip-2612) 支持链下转账审批。

2. `UniswapV2Factory` – similarly to V1, this is a factory contract that creates pair contracts and serves as a registry for them. The registry uses `create2` to generate pair addresses–we’ll see how it works in details.

   与V1类似，这是一个工厂合约，用于创建成对合约并用做注册表。注册表用于生成对地址。

3. `UniswapV2Pair` – the main contract that’s responsible for the core logic. It’s worth noting that the factory allows to create only unique pairs to not dilute liquidity.

   负责核心逻辑的主合约。值得注意的是，工厂只允许创建唯一的货币对，以免稀释流动性。

The periphery repository contains multiple contracts that make it easier to use Uniswap. Among them is `UniswapV2Router`, which is the main entrypoint for the Uniswap UI and other web and decentralized applications working on top of Uniswap. This contracts has an interface that’s very close to that of the exchange contract in Uniswap V1.

外围存储库包含多个合约，可以更轻松地使用 Uniswap。其中有， "UniswapV2Router" 这是 Uniswap UI 和其他在 Uniswap 之上工作的 Web 和去中心化应用程序的主要入口点。 该合约的接口与 Uniswap V1 中的交易所合约非常接近。

Another important contract in the periphery repository is `UniswapV2Library`, which is a collection of helper functions that implement important calculations. We’ll implement both of these contracts.

外围存储库中另一个重要的约定是 "UniswapV2Library"，它是辅助函数的集合 实现重要计算。我们将实现这两个协定。

We’ll start our journey with the core contracts to focus on the most important mechanics first. We’ll see that these contracts are very general and their functions require preparatory steps–this low-level structure reduces the attack surfaces and makes the whole architecture more granular.

我们将从核心合同开始我们的旅程，首先关注最重要的机制。我们将看到这些合同 非常通用，其功能需要准备步骤——这种低级结构减少了攻击面并使 整个架构更加精细化。

Alright, let’s begin!

## Pooling liquidity汇集流动性

No trades are possible without liquidity. Thus, the first feature we need to implement is liquidity pooling. How does it work?

没有流动性，任何交易都是不可能的。因此，我们需要实现的第一个功能是流动性池。它是如何实现的 工作？

Liquidity pools are simply contracts that store token liquidity and allow to perform swaps that use this liquidity. So, “pooling liquidity” means sending tokens to a smart-contract and storing them there for some time.

流动性池只是存储代币流动性并允许使用这种流动性进行掉期的合约。所以 “汇集流动性”意味着将代币发送到智能合约并在那里存储一段时间。

As you probably already know, every contract has its own storage, and the same is true for ERC20 tokens–each of them has a `mapping` that connects addresses and their balances. And our pools will have their own balances in ERC20 contracts. Is this enough to pool liquidity? As it turns out, no.

您可能已经知道，每个合约都有自己的存储空间，ERC20 代币也是如此——每个代币 具有 "mapping" 连接地址及其余额的 a。我们的资金池将在 ERC20 合约中拥有自己的余额。 这足以汇集流动性吗？事实证明，没有。

The main reason is that relying only on ERC20 balances would make price manipulations possible: imaging someone sending a big amount of tokens to a pool, makes profitable swaps, and cashes out in the end. To avoid such situations, **we need to track pool reserves on our side**, and we also need to control when they’re updated.

主要原因是仅依靠 ERC20 余额会使价格变高 可能的操纵：想象有人向池子发送大量代币，进行有利可图的掉期，然后兑现 最后。为了避免这种情况，我们需要跟踪我们这边的池储备，我们还需要控制它们何时 更新。

We’ll use `reserve0` and `reserve1` variable to track reserves in pools:

我们将使用 "reserve0" 和 "reserve1" 变量来跟踪池中的储备：

```solidity
contract ZuniswapV2Pair is ERC20, Math {
  ...

  uint256 private reserve0;
  uint256 private reserve1;

  ...
}
```



If you followed my [UniswapV1 series](https://jeiwan.net/posts/programming-defi-uniswap-1/), you probably remember that we implemented `addLiquidity` function that counted new liquidity and issued LP-tokens. Uniswap V2 implements an identical function in periphery contract `UniswapV2Router`, and, in the pair contract, this functionality is implemented at a lower level: liquidity management is simply viewed as LP-tokens management. When you add liquidity to a pair, the contract mints LP-tokens; when you remove liquidity, LP-tokens get burned. As I explained earlier, core contracts are lower-level contracts that perform only core operations.

如果你关注了我的 UniswapV1 系列，你可能还记得 我们实现了 "addLiquidity" 计算新流动性和发行LP代币的功能。Uniswap V2 实现了相同的 函数，"UniswapV2Router"而在 pair 合约中，此功能在 较低层次：流动性管理被简单地看作是LP代币管理。当您为一对货币对添加流动性时，合约 铸造LP代币;当您移除流动性时，LP 代币会被烧毁。正如我之前所解释的，核心合约是较低级别的 仅执行核心操作的合同。

So, here’s the low-level function for depositing new liquidity:

因此，这是用于存入新流动性的低级函数：

```solidity
function mint() public {
   uint256 balance0 = IERC20(token0).balanceOf(address(this));
   uint256 balance1 = IERC20(token1).balanceOf(address(this));
   uint256 amount0 = balance0 - reserve0;
   uint256 amount1 = balance1 - reserve1;

   uint256 liquidity;

   if (totalSupply == 0) {
      liquidity = ???
      _mint(address(0), MINIMUM_LIQUIDITY);
   } else {
      liquidity = ???
   }

   if (liquidity <= 0) revert InsufficientLiquidityMinted();

   _mint(msg.sender, liquidity);

   _update(balance0, balance1);

   emit Mint(msg.sender, amount0, amount1);
}
```

First, we need to calculate newly deposited amounts that haven’t yet been counted (saved in reserves). Then, we calculate the amount of LP-tokens that must be issued as a reward for provided liquidity. Then, we issue the tokens and update reserves (function `_update` simply saves balances to the reserve variables). The function is quite minimal, isn’t it?

首先，我们需要计算尚未计算（保存在储备金中）的新存入金额。然后，我们计算 必须发行的LP代币数量，作为对所提供流动性的奖励。然后，我们发行令牌并更新 储备金（函数 "_update" 只是将余额保存到储备金变量中）。功能非常小，不是吗？

As you can see from the code, liquidity is calculated differently when initially deposited into pool (the `totalSupply == 0` branch). Think about this: how many LP-token do we need to issue when there’s no liquidity in the pool? Uniswap V1 used the amount of deposited ethers, which made the initial amount of LP-tokens dependent on the ratio at which liquidity was deposited. But nothing forces users to deposit at the correct ratio that reflects actual prices at that moment. Moreover, Uniswap V2 now supports arbitrary ERC20 token pairs, which means there might be no prices valued in ETH at all.

从代码中可以看出，流动性在最初存入矿池（分行）时计算方式不同 "totalSupply == 0" 。 想一想：当池中没有流动性时，我们需要发行多少 LP 代币？Uniswap V1 使用了该金额 存入的以太币，这使得LP代币的初始数量取决于存入流动性的比率。但 没有什么能强迫用户以反映当时实际价格的正确比率存款。此外，Uniswap V2 现在 支持任意 ERC20 代币对，这意味着可能根本没有 ETH 价值。

For initial LP-amount, Uniswap V2 ended up using geometric mean of deposited amounts:

对于初始 LP 金额，Uniswap V2 最终使用存款金额的几何平均值：
$$
Liquidity_{minted} = \sqrt{Amount_0 * Amount_1}
$$
The main benefit of this decision is that such formula ensures that the initial liquidity ratio doesn’t affect the value of a pool share.

此决策的主要好处是，这种公式可确保初始流动性比率不会影响价值 池共享。

Now, let’s calculate LP-tokens issued when there’s already some liquidity. The main requirement here is that the amount is:

现在，让我们计算一下已经有一些流动性时发行的 LP 代币。这里的主要要求是金额 水务署

1. proportional to the deposited amount,与存款金额成正比，
2. proportional to the total issued amount of LP-tokens.与LP代币的总发行量成正比。

Recall this formula from the V1 series:

回想一下 V1 系列中的这个公式：
$$
Liquidity_{minted} = TotalSupply_{LP} * \dfrac{Amount_{deposited}}{Reserve}
$$
New amount of LP-tokens, that’s proportional to the deposited amount of tokens, gets minted. But, in V2, there are two underlying tokens–which of them should we use in the formula?

新数量的 LP 代币将被铸造，其数量与存入的代币数量成正比。 但是，在 V2 中，有两个底层代币——我们应该在公式中使用哪一个？

We can choose either of them, but there’s interesting pattern: the closer the ratio of deposited amounts to the ratio of reserves, the smaller the difference. Consequently, if the ratio of deposited amounts is different, LP amounts will also be different, and one of them will be bigger than the other. If we choose the bigger one, then we’ll incentivize price changing via liquidity provision and this leads to price manipulation. If we choose the smaller one, we’ll punish for depositing of unbalanced liquidity (liquidity providers would get fewer LP-tokens). It’s clear that choosing smaller number is more benefitial, and this is what Uniswap is doing. Let’s fill the gaps in the above code:

我们可以选择其中任何一个，但有一个有趣的模式：存款金额的比率与准备金的比率越接近，差异就越小。 因此，如果存入金额的比例不同，LP的金额也会不同，并且其中一个会比另一个大。 如果我们选择更大的一个，那么我们将通过提供流动性来激励价格变化，这会导致价格操纵。 如果我们选择较小的一个，我们将因存放不平衡的流动性而受到惩罚（流动性提供者将获得更少的 LP 代币）。 显然，选择较小的数字更有利，这就是 Uniswap 正在做的事情。 让我们填补上面代码中的空白：

```solidity
if (totalSupply == 0) {
   liquidity = Math.sqrt(amount0 * amount1) - MINIMUM_LIQUIDITY;
   _mint(address(0), MINIMUM_LIQUIDITY);
} else {
   liquidity = Math.min(
      (amount0 * totalSupply) / _reserve0,
      (amount1 * totalSupply) / _reserve1
   );
}
```

In the first branch, we’re subtracting `MINIMUM_LIQUIDITY` (which is a constant 1000, or 1e-15) when initial liquidity is provided. This protects from someone making one pool token share (1e-18, 1 wei) too expensive, which would turn away small liquidity providers. 1000 wei of LP-tokens is a negligible amount for most of pools, but if someone tries to make the cost of one pool token share too expensive (say, $100), they’d have to burn 1000 times of such cost (that is, $100,000).

To solidify our understanding of minting, let’s write tests.

## Writing tests in Solidity 在 Solidity 中编写测试

As I said above, I’ll be using Foundry to test our smart contracts–this will allow us to quickly set up our tests and not have any business with JavaScript. Our smart contracts tests will simply be other smart contracts. That’s it: **smart contracts that test smart contracts**.

正如我上面所说，我将使用 Foundry 来测试我们的智能合约——这将使我们能够快速设置我们的测试和 与 JavaScript 没有任何业务往来。我们的智能合约测试将只是其他智能合约。就是这样：智能 测试智能合约的合约。

This is all we need to set up testing of the pair contract:

这就是我们设置货币对合约测试所需的全部内容：

```solidity
contract ZuniswapV2PairTest is Test {
  ERC20Mintable token0;
  ERC20Mintable token1;
  ZuniswapV2Pair pair;

  function setUp() public {
    token0 = new ERC20Mintable("Token A", "TKNA");
    token1 = new ERC20Mintable("Token B", "TKNB");
    pair = new ZuniswapV2Pair(address(token0), address(token1));

    token0.mint(10 ether);
    token1.mint(10 ether);
  }

  // Any function starting with "test" is a test case.
}
```



Let’s add a test for pair bootstrapping (providing initial liquidity):

让我们添加一个对引导测试（提供初始流动性）：

```solidity
function testMintBootstrap() public {
  token0.transfer(address(pair), 1 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint();

  assertEq(pair.balanceOf(address(this)), 1 ether - 1000);
  assertReserves(1 ether, 1 ether);
  assertEq(pair.totalSupply(), 1 ether);
}
```



1 ether of `token0` and 1 ether of `token1` are added to the test pool. As a result, 1 ether of LP-tokens is issued and we get 1 ether - 1000 (minus the minimal liquidity). Pool reserves and total supply get changed accordingly.

将 1 个以太币和 1 个以太"token0"币 "token1" 添加到测试池中。结果，发行了 1 个 LP 代币以太币，我们 获得 1 个以太币 - 1000（减去最低流动性）。池储备和总供应量会相应变化。

What happens when balanced liquidity is provided to a pool that already has some liquidity? Let’s see:

当向已经拥有一定流动性的池提供平衡流动性时会发生什么？我看看：

```solidity
function testMintWhenTheresLiquidity() public {
  token0.transfer(address(pair), 1 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint(); // + 1 LP

  token0.transfer(address(pair), 2 ether);
  token1.transfer(address(pair), 2 ether);

  pair.mint(); // + 2 LP

  assertEq(pair.balanceOf(address(this)), 3 ether - 1000);
  assertEq(pair.totalSupply(), 3 ether);
  assertReserves(3 ether, 3 ether);
}
```

Everything looks correct here. Let’s see what happens when unbalanced liquidity is provided:

这里的一切看起来都是正确的。让我们看看当提供不平衡的流动性时会发生什么：

```solidity
function testMintUnbalanced() public {
  token0.transfer(address(pair), 1 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint(); // + 1 LP
  assertEq(pair.balanceOf(address(this)), 1 ether - 1000);
  assertReserves(1 ether, 1 ether);

  token0.transfer(address(pair), 2 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint(); // + 1 LP
  assertEq(pair.balanceOf(address(this)), 2 ether - 1000);
  assertReserves(3 ether, 2 ether);
}
```



This is what we talked about: even though user provided more `token0` liquidity than `token1` liquidity, they still got only 1 LP-token.

这就是我们所讨论的：即使用户提供的"token0"流动性多于流动性 "token1" ， 他们仍然只得到了 1 个 LP 代币。

Alright, liquidity provision looks good. Let’s now move to liquidity removal.

好吧，流动性供应看起来不错。现在让我们转向流动性移除。

## Removing liquidity消除流动性

Liquidity removal is opposite to provision. Likewise, burning is opposite to minting. Removing liquidity from pool means burning of LP-tokens in exchange for proportional amount of underlying tokens. The amount of tokens returned to liquidity provided is calculated like that:

流动性移除与拨备相反。同样，燃烧与铸币相反。从资金池中移除流动性意味着 销毁LP代币以换取一定比例数量的基础代币。返回流动性的代币数量 提供的计算方式如下：
$$
Amount_{token} = Reserve_{token} * {\frac {Balance_{LP}}{TotalSupply_{LP}}}
$$
In plain English: the amount of tokens returned is proportional to the amount of LP-tokens held over total supply of LP tokens. The bigger your share of LP-tokens, the bigger share of reserve you get after burning.

简单来说：返回的代币数量与持有的 LP 代币数量占 LP 总供应量成正比 令 牌。您的LP代币份额越大，销毁后获得的储备份额就越大。

And this is all we need to know to implement `burn` function:

这就是我们实现函数需要知道的全部内容 "burn" ：

```solidity
function burn() public {
  uint256 balance0 = IERC20(token0).balanceOf(address(this));
  uint256 balance1 = IERC20(token1).balanceOf(address(this));
  uint256 liquidity = balanceOf[msg.sender];

  uint256 amount0 = (liquidity * balance0) / totalSupply;
  uint256 amount1 = (liquidity * balance1) / totalSupply;

  if (amount0 <= 0 || amount1 <= 0) revert InsufficientLiquidityBurned();

  _burn(msg.sender, liquidity);

  _safeTransfer(token0, msg.sender, amount0);
  _safeTransfer(token1, msg.sender, amount1);

  balance0 = IERC20(token0).balanceOf(address(this));
  balance1 = IERC20(token1).balanceOf(address(this));

  _update(balance0, balance1);

  emit Burn(msg.sender, amount0, amount1);
}
```

As you can see, UniswapV2 doesn’t support partial removal of liquidity.

如您所见，UniswapV2 不支持部分移除流动性。

> **Update**: the above statement is wrong! I made a logical bug in this function, can you spot it? If not, I explained and fixed it in [Part 4](https://jeiwan.net/posts/programming-defi-uniswapv2-4/) ([commit](https://github.com/Jeiwan/zuniswapv2/commit/babf8509b8be96796e2d944710bfcb22cc1fe77d#diff-835d3f34100b5508951336ba5a961932492eaa6923e3c5299f77007019bf2b6fR84))
>
> 更新：上述说法是错误的！我在这个函数中犯了一个逻辑错误，你能发现它吗？如果没有，我解释说 并在第 4 部分（提交）中修复了它

Let’s test it:

让我们来测试一下：

```solidity
function testBurn() public {
  token0.transfer(address(pair), 1 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint();
  pair.burn();

  assertEq(pair.balanceOf(address(this)), 0);
  assertReserves(1000, 1000);
  assertEq(pair.totalSupply(), 1000);
  assertEq(token0.balanceOf(address(this)), 10 ether - 1000);
  assertEq(token1.balanceOf(address(this)), 10 ether - 1000);
}
```

We see that the pool returns to its uninitialized state except the minimum liquidity that was sent to the zero address– it cannot be claimed.

我们看到池子恢复到未初始化的状态，除了发送到零地址的最小流动性—— 它不能被认领。

Now, let’s see what happens when we burn after providing unbalanced liquidity:

现在，让我们看看在提供不平衡的流动性后销毁时会发生什么：

```solidity
function testBurnUnbalanced() public {
  token0.transfer(address(pair), 1 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint();

  token0.transfer(address(pair), 2 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint(); // + 1 LP

  pair.burn();

  assertEq(pair.balanceOf(address(this)), 0);
  assertReserves(1500, 1000);
  assertEq(pair.totalSupply(), 1000);
  assertEq(token0.balanceOf(address(this)), 10 ether - 1500);
  assertEq(token1.balanceOf(address(this)), 10 ether - 1000);
}
```

What we see here is that we have lost 500 wei of `token0`! This is the punishment for price manipulation we talked above. But the amount is ridiculously small, it doesn’t seem significant at all. This so because our current user (the test contract) is the only liquidity provider. What if we provide unbalanced liquidity to a pool that was initialized by another user? Let’s see:

我们在这里看到的是，我们已经损失了 500 wei 的"token0"！这是我们谈到的对价格操纵的惩罚 以上。但数量少得离谱，似乎一点也不重要。之所以如此，是因为我们当前的用户（ 测试合约）是唯一的流动性提供者。如果我们向初始化的池提供不平衡的流动性怎么办 由其他用户？我看看：

```solidity
function testBurnUnbalancedDifferentUsers() public {
  testUser.provideLiquidity(
    address(pair),
    address(token0),
    address(token1),
    1 ether,
    1 ether
  );

  assertEq(pair.balanceOf(address(this)), 0);
  assertEq(pair.balanceOf(address(testUser)), 1 ether - 1000);
  assertEq(pair.totalSupply(), 1 ether);

  token0.transfer(address(pair), 2 ether);
  token1.transfer(address(pair), 1 ether);

  pair.mint(); // + 1 LP

  assertEq(pair.balanceOf(address(this)), 1);

  pair.burn();

  assertEq(pair.balanceOf(address(this)), 0);
  assertReserves(1.5 ether, 1 ether);
  assertEq(pair.totalSupply(), 1 ether);
  assertEq(token0.balanceOf(address(this)), 10 ether - 0.5 ether);
  assertEq(token1.balanceOf(address(this)), 10 ether);
}
```

This looks completely different! We’ve now lost 0.5 ether of `token0`, which is 1/4 of what we deposited. Now that’s a significant amount!

这看起来完全不同！我们现在已经损失了 0.5 个以太"token0"币，这是我们存入的 1/4。现在是 一大笔钱！

Try to figure out who eventually gets that 0.5 ether: the pair or the test user? 😉

试着弄清楚谁最终得到了那 0.5 个以太币：这对还是测试用户？😉

## Conclusion结论

Well, enough for today. Feel free experimenting with the code and, for example, choosing the bigger amount of LP-tokens when adding liquidity to a pool.

好了，今天就够了。随意尝试代码，例如，选择更多数量的 LP 代币 向资金池添加流动性时。
