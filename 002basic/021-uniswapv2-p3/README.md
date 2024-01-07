# Programming DeFi: Uniswap V2. Part 3

## Factory contract

The factory contract is a registry of all deployed pair contracts. This contract is necessary because we don’t want to have pairs of identical tokens so liquidity is not split into multiple identical pairs. The contract also simplifies pair contracts deployment: instead of deploying the pair contract manually, one can simply call a method in the factory contract.

工厂合约是所有已部署的对合约的注册表。这份合同是必要的，因为我们不想 拥有成对的相同代币，因此流动性不会被拆分成多个相同的对。该合约还简化了配对 合约部署：无需手动部署成对合约，只需调用 工厂合同。

There’s only one factory contract deployed by the Uniswap team, and the contract serves as the official registry of Uniswap pairs. This is also useful in terms of pairs discovery: one can query the contract to find a pair by token addresses. Also, the history of contract’s events can be scanned to find all deployed pairs. Of course, nothing stops us from deploying our pair manually and not registering it with the factory contract.

Uniswap 团队只部署了一个工厂合约，该合约作为 Uniswap 对。这在货币对发现方面也很有用：可以查询合约以通过代币查找货币对 地址。此外，还可以扫描合约事件的历史记录以查找所有已部署的货币对。当然，没有什么能阻止我们 从手动部署我们的货币对而不是将其注册到工厂合同中。



```solidity
contract ZuniswapV2Factory {
    error IdenticalAddresses();
    error PairExists();
    error ZeroAddress();

    event PairCreated(
        address indexed token0,
        address indexed token1,
        address pair,
        uint256
    );

    mapping(address => mapping(address => address)) public pairs;
    address[] public allPairs;
...
```

The factory contract is minimal and plain: it only emits `PairCreated` event when a pair is created and it stores a list and a mapping of all created pairs.

工厂协定是最小和简单的：它只 "PairCreated" 在创建一对时发出事件，并存储一个列表 以及所有已创建对的映射。

Creating pairs is tricky though:

不过，创建配对是很棘手的：



```solidity
function createPair(address tokenA, address tokenB)
  public
  returns (address pair)
{
  if (tokenA == tokenB) revert IdenticalAddresses();

  (address token0, address token1) = tokenA < tokenB
    ? (tokenA, tokenB)
    : (tokenB, tokenA);

  if (token0 == address(0)) revert ZeroAddress();

  if (pairs[token0][token1] != address(0)) revert PairExists();

  bytes memory bytecode = type(ZuniswapV2Pair).creationCode;
  bytes32 salt = keccak256(abi.encodePacked(token0, token1));
  assembly {
    pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
  }

  IZuniswapV2Pair(pair).initialize(token0, token1);

  pairs[token0][token1] = pair;
  pairs[token1][token0] = pair;
  allPairs.push(pair);

  emit PairCreated(token0, token1, pair, allPairs.length);
}
```

First, we don’t allow pairs with identical tokens. Notice that we don’t check if the token contracts actually exist–we don’t care because it’s up to user to provide valid ERC20 token addresses.

首先，我们不允许使用相同令牌的货币对。请注意，我们不会检查代币合约是否真的存在——我们 不在乎，因为由用户提供有效的 ERC20 代币地址。

Next, we sort token addresses–this is important to avoid duplicates (the pair contract allows swaps in both directions). Also, pair token addresses are used to generate pair address–we’ll talk about this next.

接下来，我们对代币地址进行排序——这对于避免重复很重要（对合约允许双向互换）。 此外，对令牌地址用于生成对地址——我们接下来将讨论这个问题。

Next comes the main part of the function: deployment of a pair. And this part requires more attention.

接下来是该功能的主要部分：部署一对。而这部分需要更多的关注。

## Contracts deployment via CREATE2 opcode通过CREATE2操作码部署合约

In Ethereum, contracts can deploy contracts. One can call a function of a deployed contract, and this function will deploy another contract–this makes deployment of, let’s call them “template”, contracts much easier. You don’t need to compile and deploy a contract from you computer, you can do this via an existing contract.

在以太坊中，合约可以部署合约。可以调用已部署合约的函数，此函数将 部署另一个合约——这使得部署合约变得更加容易，我们称之为“模板”。你不需要 从您的计算机编译和部署合约，您可以通过现有合约来执行此操作。

In EVM, there are two opcodes that deploy contracts:

在 EVM 中，有两个操作码用于部署合约：

1. [CREATE](https://www.evm.codes/#f0), which was in EVM from the very beginning. This opcode creates a new account (Ethereum address) and deploys contract code at this address. The new address is calculated based on the deployer contract’s nonce–this is identically to how contract address is determined when you deploy contract manually. Nonce is the counter of address’ successful transactions: when you send a transaction, you increase your nonce. This dependence on nonce when generating new account address makes `CREATE` non-deterministic: the address depends on on the nonce of the deployer contract, which you cannot control. You do can know it, but by the time you deploy your contract, the nonce can be different.

   从一开始就存在于 EVM 中。此操作码创建一个新帐户 （以太坊地址）并在该地址部署合约代码。新地址是根据部署程序计算的 合约的随机数 – 这与手动部署合约时确定合约地址的方式相同。随机数 是地址成功交易的计数器：当您发送交易时，您会增加您的随机数。这 生成新账户地址时对随机数的依赖性使 `CREATE`得不可确定性：地址依赖于 在部署程序合约的随机数上，您无法控制。你确实可以知道它，但当你 部署您的合约，随机数可以不同。

2. [CREATE2](https://www.evm.codes/#f5), which was added in [EIP-1014](https://eips.ethereum.org/EIPS/eip-1014). This opcode acts exactly like `CREATE` but **it allows to generate new contract’s address deterministically**. `CREATE2` doesn’t use external state (like other contract’s nonce) to generate a contract address and lets us fully control how the address is generated. You don’t need to know `nonce`, you only need to know deployed contract bytecode (which is static) and salt (which is a sequence of bytes chosen by you).

   

   [CREATE2](https://www.evm.codes/#f5)，已添加到[EIP-1014](https://eips.ethereum.org/EIPS/eip-1014)中。 该操作码的行为与“CREATE”完全相同，但**它允许确定性地生成新合约的地址**。 `CREATE2` 不使用外部状态（如其他合约的随机数）来生成合约地址，并让我们完全控制地址的生成方式。 你不需要知道“nonce”，你只需要知道部署的合约字节码（这是静态的）和salt（这是你选择的字节序列）。

   

Let’s return to the code:

```solidity
...
bytes memory bytecode = type(ZuniswapV2Pair).creationCode;
bytes32 salt = keccak256(abi.encodePacked(token0, token1));
assembly {
    pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
}
...
```

In the first line, we get the creation bytecode of `ZuniswapV2Pair` contract. Creation bytecode is actual smart contract bytecode. It includes:

在第一行中，我们得到合约的创建字节码 "ZuniswapV2Pair" 。创建字节码是实际的智能合约 字节码。它包括：

1. Constructor logic. This part is responsible for smart contract initialization and deployment. It’s **not stored** on the blockchain.

   构造函数逻辑。这部分负责智能合约的初始化和部署。它 不存储在区块链上

2. Runtime bytecode, which is actual business logic of contract. It’s this bytecode that’s stored on the Ethereum blockchain.

   运行时字节码，是合约的实际业务逻辑。正是这个字节码存储在以太坊上 区块链。

We want to use full bytecode here.

我们想在这里使用全字节码。

Next line creates `salt`, a sequence of bytes that’s used to generate new contract’s address deterministically. We’re hashing pair’s token addresses to create the salt–this means that every unique pair of tokens will produce a unique salt, and every pair will have unique salt and address.

下一行创建 "salt" ，一个字节序列，用于确定性地生成新合约的地址。我们是 散列对的代币地址以创建 SALT——这意味着每对唯一的代币都会产生 一种独特的盐，每对都会有独特的盐和地址。

And the final line is where we’re calling `create2` to:

最后一行是我们呼吁的地方 "create2" ：

1. Create a new address deterministically using `bytecode` + `salt`.
2. Deploy a new `ZuniswapV2Pair` contract.
3. Get that pair’s address. 获取该货币对的地址。

> [This StackOverflow answer](https://ethereum.stackexchange.com/a/84844) does the great job of explaining CREATE2 parameters.

The rest of `createPair` should be clear:

1. After a pair is deployed, we need to initialize it, which simply means to set its tokens:

   部署一对后，我们需要初始化它，简单来说就是设置它的tokens：

   ```solidity
   // ZuniswapV2Pair.sol
   function initialize(address token0_, address token1_) public {
     if (token0 != address(0) || token1 != address(0))
       revert AlreadyInitialized();
   
     token0 = token0_;
     token1 = token1_;
   }
   ```

2. Then, the new pair is stored in the `pairs` mapping and `allPairs` array.

   然后，新对存储在 "pairs" 映射和 "allPairs" 数组中。

3. And finally, we can emit `PairCreated` event.

   最后，我们可以发出 "PairCreated" 事件。

Moving on!

## Router contract

We’re now ready to open a new bigger chapter of this series: we’re starting working on the `Router` contract.

现在，我们已准备好开启这个系列的一个新的更大的篇章：我们开始制定 "Router" 合约。

The `Router` contract is a high-level contract that serves as the entrypoint for most user applications. This contract makes it easier to create pairs, add and remove liquidity, calculate prices for all possible swap variations and perform actual swaps. `Router` works with all pairs deployed via the Factory contract, it’s a universal contract.

该"Router"协定是一个高级协定，用作大多数用户应用程序的入口点。本合同 可以更轻松地创建货币对、添加和删除流动性、计算所有可能的掉期变化的价格并执行 实际掉期。"Router" 适用于通过 Factory 合约部署的所有对，这是一个通用合约。

> It’s also a big contract and we probably won’t implement all of its functions because most of them are variants of swapping.
>
> *这也是一个大合约，我们可能不会实现它的所有功能，因为它们中的大多数都是交换的变体。*

In parallel to `Router`, we’ll be programming the `Library` contract, which implements all basic and core functions, most of which are swap amounts calculations.

同时"Router"，我们将对合约进行编程"Library"，该合约实现了所有基本和核心功能， 其中大部分是掉期金额计算。

Let’s look at Router’s constructor: router can deploy pairs thus it needs to know the address of the Factory contract.

让我们看一下 Router 的构造函数：router 可以部署对，因此它需要知道 Factory 合约的地址。

```solidity
ontract ZuniswapV2Router {
    error InsufficientAAmount();
    error InsufficientBAmount();
    error SafeTransferFailed();

    IZuniswapV2Factory factory;

    constructor(address factoryAddress) {
        factory = IZuniswapV2Factory(factoryAddress);
    }
    ...
```

we’ll implement only liquidity management, and next time we’ll finish the contract.

今天，我们将只实施流动性管理，下次我们将完成合约。

Let’s start with `addLiquidity`:

```solidity
function addLiquidity(
    address tokenA,
    address tokenB,
    uint256 amountADesired,
    uint256 amountBDesired,
    uint256 amountAMin,
    uint256 amountBMin,
    address to
)
    public
    returns (
        uint256 amountA,
        uint256 amountB,
        uint256 liquidity
    )
    ...
```



When compared to the `mint` function from the pair contract, this function has quite many parameters!

1. `tokenA` and `tokenB` are used to find (or create) the pair we want to add liquidity to.
2. `amountADesired` and `amountBDesired` are the amounts we want to deposit into the pair. These are upper bounds.
3. `amountAMin` and `amountBMin` are the minimal amounts we want to deposit. Remember that the `Pair` contract always issues smaller amount of LP tokens when we deposit unbalanced liquidity? (We discussed this in [Part1](https://jeiwan.net/posts/programming-defi-uniswapv2-1)). So, the `min` parameters allow us to control how much liquidity we’re ready to lose.
4. `to` address is the address that receives LP-tokens.

```solidity
...
if (factory.pairs(tokenA, tokenB) == address(0)) {
    factory.createPair(tokenA, tokenB);
}
...
```



Here’s where you start seeing the high abstraction nature of the `Router` contract: if there’s no pair contract for the specified ERC20 tokens, it’ll be created by the `Router` contract. `factory.pairs` method is the `pairs` mapping: Solidity made the helper method with two parameters since the mapping is nested.

```solidity
...
(amountA, amountB) = _calculateLiquidity(
    tokenA,
    tokenB,
    amountADesired,
    amountBDesired,
    amountAMin,
    amountBMin
);
...
```

In the next step, we’re calculating the amounts that will be deposited. We’ll return to this function a little bit later.

```solidity
...
address pairAddress = ZuniswapV2Library.pairFor(
    address(factory),
    tokenA,
    tokenB
);
_safeTransferFrom(tokenA, msg.sender, pairAddress, amountA);
_safeTransferFrom(tokenB, msg.sender, pairAddress, amountB);
liquidity = IZuniswapV2Pair(pairAddress).mint(to);
...
```

After we’ve calculated liquidity amounts, we can finally transfer tokens from the user and mint LP-tokens in exchange. Most of these lines should be already familiar to you, except the `pairFor` function–we’ll implement it right after implementing `_calculateLiquidity`. Also, notice that this contract doesn’t expect user to transfer tokens manually–it transfers them from user’s balance using the ERC20 `transferFrom` function.

```solidity
function _calculateLiquidity(
    address tokenA,
    address tokenB,
    uint256 amountADesired,
    uint256 amountBDesired,
    uint256 amountAMin,
    uint256 amountBMin
) internal returns (uint256 amountA, uint256 amountB) {
    (uint256 reserveA, uint256 reserveB) = ZuniswapV2Library.getReserves(
        address(factory),
        tokenA,
        tokenB
    );

    ...
```

In this function, we want to find the liquidity amounts that will satisfy our desired and minimal amounts. Since there’s a delay between when we choose liquidity amounts in UI and when our transaction gets processed, actual reserves ratio might change, which will result in us losing some LP-tokens (as a punishment for depositing unbalanced liquidity). By selecting desired and minimal amounts, we can minimize this loss.

> Refer to [Part1](https://jeiwan.net/posts/programming-defi-uniswapv2-1) to learn about how unbalanced liquidity affects issued LP-tokens.

First step in the function is to get pool reserves by using the library contract–we’ll implement this soon. Knowing pair reserves, we can calculate optimal liquidity amounts

```solidity
...
if (reserveA == 0 && reserveB == 0) {
    (amountA, amountB) = (amountADesired, amountBDesired);
...
```

If reserves are empty then this is a new pair, which means our liquidity will define the reserves ratio, which means we won’t get punished by providing unbalanced liquidity. Thus, we’re allowed to deposit full desired amounts.

```solidity
...
} else {
    uint256 amountBOptimal = ZuniswapV2Library.quote(
        amountADesired,
        reserveA,
        reserveB
    );
    if (amountBOptimal <= amountBDesired) {
        if (amountBOptimal <= amountBMin) revert InsufficientBAmount();
        (amountA, amountB) = (amountADesired, amountBOptimal);
...
```

Otherwise, we need to find optimal amounts, and we begin with finding optimal `tokenB` amount. `quote` is another function from the library contract: by taking input amount and pair reserves, it calculates output amount, which is `tokenA` price nominated in `tokenB` multiplied by input amount.

> `quote` is not how swap price is calculated! We’ll discuss prices calculation in details in next part.

If `amountBOptimal` is less or equal to our desired amount AND if it’s higher than our minimal amount, then it’s used. This difference between desired and minimal amounts is what protects us from slippage.

However, if `amountBOptimal` is greater than our desired amount, it cannot be used and we need to find a different, optimal, amount A.

```solidity
...
} else {
    uint256 amountAOptimal = ZuniswapV2Library.quote(
        amountBDesired,
        reserveB,
        reserveA
    );
    assert(amountAOptimal <= amountADesired);

    if (amountAOptimal <= amountAMin) revert InsufficientAAmount();
    (amountA, amountB) = (amountAOptimal, amountBDesired);
}
```

Using identical logic we’re finding `amountAOptimal`: it also must be within our minimal-desired range.

> If this logic is not clear for you, feel free experimenting with tests! Luckily, Foundry and Forge make writing Solidity tests so much easier!

Let’s put aside the Router contract and switch to the library.

## Library contract

The Library contract is a library (no pun intended 😬). Library, in Solidity, is a stateless contract (i.e. it doesn’t have mutable state) that implements a set of functions that can be used by other contracts–this is the main purpose of a library. Unlike contracts, libraries don’t have state: their functions are executed in caller’s state via [DELEGATECALL](https://www.evm.codes/#f4). But, like contracts, libraries must be deployed to be used. Luckily, Forge makes our life easier since [it supports automatic libraries linking](https://github.com/gakonst/foundry/pull/586) (we don’t need to deploy libraries in our tests).

Let’s implement the library!

```solidity
library ZuniswapV2Library {
    error InsufficientAmount();
    error InsufficientLiquidity();

    function getReserves(
        address factoryAddress,
        address tokenA,
        address tokenB
    ) public returns (uint256 reserveA, uint256 reserveB) {
        (address token0, address token1) = _sortTokens(tokenA, tokenB);
        (uint256 reserve0, uint256 reserve1, ) = IZuniswapV2Pair(
            pairFor(factoryAddress, token0, token1)
        ).getReserves();
        (reserveA, reserveB) = tokenA == token0
            ? (reserve0, reserve1)
            : (reserve1, reserve0);
    }
    ...
```

This is a high-level function, it can get reserves of any pair (don’t confuse it with the one from the pair contract–that one returns reserves of the specific pair).

First step in the function is token addresses sorting–we always want to do this when we want to find pair address by token addresses. And this is what we do in the next step: having factory address and sorted token addresses, we’re able to obtain the pair address–we’ll look at the `pairFor` function next.

Notice that the reserves are sorted back before being returned: we want to return them in the same order as token addresses were specified!

Now, let’s look at the `pairFor` function:

```solidity
function pairFor(
    address factoryAddress,
    address tokenA,
    address tokenB
) internal pure returns (address pairAddress) {
```

The function is used to find pair address by factory and token addresses. The straightforward way of doing that is by fetching pair address from the factory contract, like:

```solidity
ZuniswapV2Factory(factoryAddress).pairs(address(token0), address(token1))
```

But this would make an external call, which makes the function a little more expensive.

Uniswap uses are more advanced approach, and this is where we get a benefit from the deterministic address generation of `CREATE2` opcode.

```solidity
(address token0, address token1) = sortTokens(tokenA, tokenB);
pairAddress = address(
    uint160(
        uint256(
            keccak256(
                abi.encodePacked(
                    hex"ff",
                    factoryAddress,
                    keccak256(abi.encodePacked(token0, token1)),
                    keccak256(type(ZuniswapV2Pair).creationCode)
                )
            )
        )
    )
);
```

This piece of code generates an address in the same way `CREATE2` does.

1. First step is to sort token addresses. Remember the `createPair` function? We used sorted token addresses as salt.
2. Next, we build a sequence of bytes that includes:
   1. `0xff` – this first byte helps to avoid collisions with `CREATE` opcode. (More details are in [EIP-1014](https://eips.ethereum.org/EIPS/eip-1014).)
   2. `factoryAddress` – factory that was used to deploy the pair.
   3. salt – token addressees sorted and hashed.
   4. hash of pair contract bytecode – we hash `creationCode` to get this value.
3. Then, this sequence of bytes gets hashed (`keccak256`) and converted to `address` (`bytes`->`uint256`->`uint160`->`address`).

This whole process is defined in [EIP-1014](https://eips.ethereum.org/EIPS/eip-1014) and implemented in the `CREATE2` opcode. What we’re doing here is reimplementing address generation in Solidity!

Finally, we’ve reached the `quote` function.

```solidity
function quote(
  uint256 amountIn,
  uint256 reserveIn,
  uint256 reserveOut
) public pure returns (uint256 amountOut) {
  if (amountIn == 0) revert InsufficientAmount();
  if (reserveIn == 0 || reserveOut == 0) revert InsufficientLiquidity();

  return (amountIn * reserveOut) / reserveIn;
}
```

As we discussed earlier, this function calculates output amount based on input amount and pair reserves. This allows to find how much of token B we would get in exchange for a specific amount of token A. This function is only used in liquidity calculation. In swapping, a formula based on the constant product formula is used.

That’s it for today!

## Links

1. [evm.codes](https://www.evm.codes/) – an interactive reference to EVM opcodes.
2. [EIP-1014](https://eips.ethereum.org/EIPS/eip-1014) – CREATE2 opcode specification.
3. [UniswapV2 Whitepaper](https://uniswap.org/whitepaper.pdf) – worth reading and re-reading.







































