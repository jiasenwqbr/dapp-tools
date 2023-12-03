# Programming DeFi: Uniswap. Part 3 

This project demonstrates a basic Hardhat use case. It comes with a sample contract, a test for that contract, and a script that deploys that contract.

Try running some of the following tasks:

```shell
npx hardhat help
npx hardhat test
REPORT_GAS=true npx hardhat test
npx hardhat node
npx hardhat run scripts/deploy.js
```

## Introduction介绍

Here we are again building a clone of Uniswap V1! Our implementation is almost ready: we’ve implemented all core mechanics of Exchange contract, including pricing functions, swapping, LP-tokens, and fees. It looks like our clone is complete, however there’s a missing piece: Factory contract. Today, we’ll implement it and our Uniswap V1 clone will be done. However, this is not the end: in the next part we’ll be building Uniswap V2 and it’ll be not less interesting than V1!

在这里，我们再次构建了 Uniswap V1 的克隆！我们的实施几乎已经准备就绪：我们已经实现了所有核心 交易所合约的机制，包括定价功能、掉期、LP代币和费用。它看起来像我们的克隆 是完整的，但是缺少一个部分：工厂合同。今天，我们将实现它，我们的 Uniswap V1 克隆将 完成。然而，这还不是结束：在下一部分中，我们将构建 Uniswap V2，它同样有趣 比V1！

## What Factory is for 工厂是干什么用的

Factory contract serves as a registry of exchanges: every new deployed Exchange contract is registered with a Factory. And this an important mechanic: any exchange can be found by querying the registry. By having such registry exchanges can find other exchanges when user tries to swap a token for another token (and not ether).

工厂合约充当交易所的注册表：每个新部署的交易所合约都向工厂注册。 这是一个重要的机制：任何交易所都可以通过查询注册表找到。通过拥有这样的注册表交换可以 当用户尝试将一个代币换成另一个代币（而不是以太币）时，查找其他交易所。

Another handy utility provided by Factory contract is ability to deploy an exchange without dealing with code, nodes, deploying scripts, and any other development tools. Factory implements a function that allows users to create and deploy an exchange by simply calling this function. So, **today we’ll also learn how a contract can deploy another contract**.

工厂合约提供的另一个方便的实用程序是能够在不处理代码、节点、 部署脚本和任何其他开发工具。Factory 实现了一个允许用户创建和部署的功能 通过简单地调用此函数进行交换。因此，今天我们还将学习一个合约如何部署另一个合约。

Uniswap has only one Factory contract, thus there’s only one registry of Uniswap pairs. However, nothing prevents other users from deploying their own Factories or even Exchange contract that are not registered with the official Factory. While this is possible, such exchanges won’t be recognized by Uniswap and there would be no way to use them to swap tokens via the official web-site.

Uniswap 只有一个 Factory 合约，因此只有一个 Uniswap 对注册表。然而，没有什么能阻止 其他用户部署自己的工厂，甚至是未在官方注册的 Exchange 合约 厂。虽然这是可能的，但 Uniswap 不会承认此类交易所，也无法使用它们 通过官方网站交换代币。

That’s basically it. Let’s get to the code!

基本上就是这样。让我们来看看代码吧！

## Factory implementation工厂实施

Factory is a registry so we need a data structure to store exchanges, and that will be a mapping of addresses to addresses – it will allow to find exchanges by their tokens (1 exchange can swap only 1 token, remember?).

Factory 是一个注册表，因此我们需要一个数据结构来存储交换，这将是地址映射到 地址 – 它将允许通过他们的代币找到交易所（1 个交易所只能交换 1 个代币，还记得吗？

```solidity
pragma solidity ^0.8.0;

import "./Exchange.sol";

contract Factory {
    mapping(address => address) public tokenToExchange;

    ...
```

Next, is the `createExchange` functions that allows to create and deploy an exchange by simply taking a token address:

接下来，是 "createExchange" 允许通过简单地获取令牌地址来创建和部署交易所的函数：

```solidity
function createExchange(address _tokenAddress) public returns (address) {
  require(_tokenAddress != address(0), "invalid token address");
  require(
    tokenToExchange[_tokenAddress] == address(0),
    "exchange already exists"
  );

  Exchange exchange = new Exchange(_tokenAddress);
  tokenToExchange[_tokenAddress] = address(exchange);

  return address(exchange);
}
```

There are two checks:

有两个检查：

1. The first ensures the token address is not the zero address (第一个确保令牌地址不是零地址 （`0x0000000000000000000000000000000000000000`).）。
2. Next one ensures that the token hasn’t already been added to the registry (default address value is the zero address). The idea is that we don’t want to have different exchanges for the same token because we don’t want liquidity to be scattered across multiple exchanges. It should better be concentrated on one exchange to reduce slippage and provide better exchange rates.下一个确保令牌尚未添加到注册表中（默认地址值为零） 地址）。我们的想法是，我们不想为同一个代币进行不同的交易所，因为我们不想 流动性将分散在多个交易所。最好集中在一个交易所，以减少 滑点并提供更好的汇率。

Next, we instantiate Exchange with the provided token address, this is why we needed to import “Exchange.sol” earlier. This instantiation is similar to instantiation of classes in OOP languages, however, in Solidity, the `new` operator will in fact deploy a contract. The returned values has the type of the contract (Exchange) and every contract can be converted to an address – this is what we’re doing on the next line to get the address of the new exchange and save it to the registry.

接下来，我们使用提供的令牌地址实例化 Exchange，这就是我们需要更早导入“Exchange.sol”的原因。 这种实例化类似于 OOP 语言中类的实例化，但是，在 Solidity 中， "new" 运算符将 实际上部署了一个合约。返回的值具有合约的类型（Exchange），每个合约都可以是 转换为地址 – 这就是我们在下一行中所做的，以获取新交易所的地址并保存它 到注册表。

To finish the contract, we need to implement only one more function – `getExchange`, which will allow us to query the registry via an interface from another contract:

为了完成合约，我们只需要再实现一个函数 – "getExchange"，这将允许我们查询 注册表通过来自另一个合约的接口：

```solidity
function getExchange(address _tokenAddress) public view returns (address) {
  return tokenToExchange[_tokenAddress];
}
```

That’s it for the factory! It’s really simple.

工厂就是这样！这真的很简单。

Next, we need to improve the exchange contract so it could use the factory to perform token-to-token swaps.

接下来，我们需要改进交换合约，以便它可以使用工厂来执行代币到代币的交换。

## Linking Exchange to Factory将 Exchange 链接到 Factory

First, we need to link Exchange to Factory because every exchange needs to know the address of the Factory and we don’t want to hard-code so the contract is more flexible. To link Exchange to Factory, we need to add a new state variable that will store factory address and we’ll need update the constructor:

首先，我们需要将 Exchange 链接到 Factory，因为每个交易所都需要知道 Factory 的地址，而我们不需要 想要硬编码，使合约更灵活。若要将 Exchange 链接到 Factory，我们需要添加一个新的状态变量 这将存储工厂地址，我们需要更新构造函数：

```solidity
contract Exchange is ERC20 {
    address public tokenAddress;
    address public factoryAddress; // <--- new line

    constructor(address _token) ERC20("Zuniswap-V1", "ZUNI-V1") {
        require(_token != address(0), "invalid token address");

        tokenAddress = _token;
        factoryAddress = msg.sender;  // <--- new line

    }
    ...
}
```

And that’s it. It’s now ready to do token-to-token swap. Let’s implement that.

就是这样。它现在已经准备好进行代币到代币的交换。让我们实现它。



## Token-to-token swaps代币到代币互换

How do we swap a token for token when we have two exchanges linked by a registry? Maybe like that:

当我们有两个交易所通过注册表链接时，我们如何将代币换成代币？也许是这样的：

1. Begin the standard token-to-ether swap.开始标准的代币到以太币交换。
2. Instead of sending ethers to user, find an exchange for the token address provided by user.与其向用户发送以太币，不如为用户提供的代币地址找到一个交易所。
3. If the exchange exists, send the ethers to the exchange to swap them to tokens.如果交易所存在，则将以太币发送到交易所以将它们交换为代币。
4. Return swapped tokens to user.将交换的令牌返回给用户。

Looks good, doesn’t it? Let’s try building that.

看起来不错，不是吗？让我们试着构建它。

We’ll this function `tokenToTokenSwap`:

我们将这个函数"tokenToTokenSwap"：

```solidity
// Exchange.sol

function tokenToTokenSwap(
    uint256 _tokensSold,
    uint256 _minTokensBought,
    address _tokenAddress
) public {
    ...
```

The function takes three arguments: the amount of tokens to be sold, minimal amount of tokens to get in exchange, the address of the token to exchange sold tokens for.

该函数有三个参数：要出售的代币数量、要交换的最小代币数量、 要交换已售出代币的代币的地址。

We first check if there’s an exchange for the token address provided by user. If there’s none, it’ll throw an error.

我们首先检查是否有用户提供的令牌地址的交换。如果没有，它将引发错误。

```solidity
address exchangeAddress = IFactory(factoryAddress).getExchange(
    _tokenAddress
);
require(
    exchangeAddress != address(this) && exchangeAddress != address(0),
    "invalid exchange address"
);
```

We’re using `IFactory` which is an interface for the Factory contract. It’s a good practice to use interfaces when interacting with other contracts (or classes in OOP). However, interfaces don’t allow to access state variables and this is why we’ve implemented the `getExchange` function in the Factory contract – so we can use the contract via an interface.

我们正在使用 "IFactory" which 是 Factory 合约的接口。在以下情况下使用接口是一种很好的做法 与其他合约（或 OOP 中的类）交互。但是，接口不允许访问状态变量，这 这就是为什么我们在 "getExchange" Factory 合约中实现了这个功能——所以我们可以通过 接口。

```solidity
interface IFactory {
  function getExchange(address _tokenAddress) external returns (address);
}
```

Next, we’re using the current exchange to swap tokens for ethers and transfer user’s tokens to the exchange. This is the standard procedure of ether-to-tokens swapping:

接下来，我们将使用当前的交易所将代币换成以太币，并将用户的代币转移到交易所。这是 以太币与代币交换的标准程序：

```solidity
uint256 tokenReserve = getReserve();
uint256 ethBought = getAmount(
    _tokensSold,
    tokenReserve,
    address(this).balance
);

IERC20(tokenAddress).transferFrom(
    msg.sender,
    address(this),
    _tokensSold
);
```

Final step of the function is using the other exchange to swap ethers to tokens:

该函数的最后一步是使用另一个交易所将以太币交换为代币：

```solidity
IExchange(exchangeAddress).ethToTokenSwap{value: ethBought}(
    _minTokensBought
);
```

And we’re done!

大功告成！

Not really, actually. Can you see a problem? Let’s looks at the last line of `etherToTokenSwap`:

实际上并非如此。你能看出问题吗？让我们看一下最后一行"etherToTokenSwap"：

```solidity
IERC20(tokenAddress).transfer(msg.sender, tokensBought);
```

A-ha! It sends bought tokens to `msg.sender`. In Solidity, `msg.sender` is dynamic, not static, and it points at the one who (or what, in the case of a contract) initiated the current call. When user calls a contract function, it would point to user’s address. But when a contract calls another contract, `msg.sender` is the address of the calling contract!

啊哈！它将购买的代币发送到 "msg.sender"。在 Solidity 中"msg.sender"，它是动态的，而不是静态的，它指向一个 谁（或什么，在合同的情况下）发起了当前调用。当用户调用合约函数时，它会指向 到用户的地址。但是当一个合约调用另一个合约时， "msg.sender" 就是调用合约的地址！

Thus, `tokenToTokenSwap` would send tokens to the address of the first exchange! This is not a problem though because we can call `ERC20(_tokenAddress).transfer(...)` to send those tokens to the user. However, there’s a getter solution: let’s save some gas and send tokens directly to the user. For this, we’ll need to split the `etherToTokenSwap` function into two functions:

因此， "tokenToTokenSwap" 将代币发送到第一个交易所的地址！这不是问题，因为我们 可以调用 "ERC20(_tokenAddress).transfer(...)" 以将这些令牌发送给用户。但是，有一个 getter 解决方案：让我们 节省一些 gas 并直接向用户发送代币。为此，我们需要将 "etherToTokenSwap" 函数拆分为 两个功能：

```solidity
function ethToToken(uint256 _minTokens, address recipient) private {
  uint256 tokenReserve = getReserve();
  uint256 tokensBought = getAmount(
    msg.value,
    address(this).balance - msg.value,
    tokenReserve
  );

  require(tokensBought >= _minTokens, "insufficient output amount");

  IERC20(tokenAddress).transfer(recipient, tokensBought);
}

function ethToTokenSwap(uint256 _minTokens) public payable {
  ethToToken(_minTokens, msg.sender);
}
```

`ethToToken` is private function that everything `ethToTokenSwap` is used to do with only one difference: it takes a tokens recipient address, which gives us the flexibility of choosing who we want to send tokens to. `ethToTokenSwap`, in its turn, is now simply a wrapper for `ethToToken` that always passes `msg.sender` as a recipient.

"ethToToken" 是私有函数，所有东西 "ethToTokenSwap" 都用来做，只有一个区别：它需要一个 令牌接收者地址，这使我们能够灵活地选择要向谁发送令牌。"ethToTokenSwap"、 反过来，现在只是一个包装器，它 "ethToToken" 总是 "msg.sender" 作为收件人传递。

Now, we need another function to send tokens to a custom recipient. We could’ve used `ethToToken` for that but let’s leave it private and non-payable.

现在，我们需要另一个函数来向自定义收件人发送令牌。我们本来可以这样做 "ethToToken" 的，但让我们 将其保密且不支付。

```solidity
function ethToTokenTransfer(uint256 _minTokens, address _recipient)
  public
  payable
{
  ethToToken(_minTokens, _recipient);
}
```

This is simply a copy of `ethToTokenSwap` that allows to send tokens to a custom recipient. We can now use it in the `tokenToTokenSwap` function:

这只是 "ethToTokenSwap" 允许将令牌发送给自定义收件人的副本。我们现在可以在 "tokenToTokenSwap" 功能：

```solidity
...

    IExchange(exchangeAddress).ethToTokenTransfer{value: ethBought}(
        _minTokensBought,
        msg.sender
    );
}
```

We’re sending tokens tokens to whoever initiated the swap.

我们正在向发起交换的人发送代币代币。

And now, we’re done!

现在，我们完成了！

## Conclusion结论

Our copy of Uniswap V1 is now finished. If you have any ideas about how to improve it, give them a try! For example, there can a function in Exchange to calculate the output amount of tokens in token-to-token swap. If you have any problems understanding how something works, feel free to check [the tests](https://github.com/Jeiwan/zuniswap/tree/part_3/test). I have covered all the functions, including `tokenToTokenSwap`.

我们的 Uniswap V1 副本现已完成。如果您对如何改进它有任何想法，请尝试一下！例如 Exchange 中可以有一个函数来计算代币到代币交换中代币的输出数量。 如果您在理解某些东西的工作原理时遇到任何问题，请随时检查 测试。我已经涵盖了所有功能，包括 "tokenToTokenSwap"。

Next time, we’ll start learning Uniswap V2. While it’s mostly the same thing, the same set or core principles, it provides some new powerful features.

下次，我们将开始学习 Uniswap V2。虽然它大多是同一件事，相同的一套或核心原则，但它 提供了一些新的强大功能。
