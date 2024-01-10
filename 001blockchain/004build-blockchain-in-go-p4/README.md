# Building Blockchain in Go. Part 4: Transactions 1

## Introduction

Transactions are the heart of Bitcoin and the only purpose of blockchain is to store transactions in a secure and reliable way, so no one could modify them after they are created. Today we’re starting implementing transactions. But because this is quite a big topic, I’ll split it into two parts: in this part, we’ll implement the general mechanism of transactions and in the second part we’ll work through details.

交易是比特币的核心，区块链的唯一目的是以安全可靠的方式存储交易，因此没有人可以在创建后对其进行修改。今天，我们开始实现交易。但是因为这是一个相当大的话题，所以我会把它分成两部分：在这一部分中，我们将实现交易的一般机制，在第二部分中，我们将研究细节。



## There is no spoon 没有勺子

If you’ve ever developed a web application, in order to implement payments you would likely to create these tables in a DB: `accounts` and `transactions`. An account would store information about a user, including their personal information and balance, and a transaction would store information about money transferring from one account to another. In Bitcoin, payments are realized in completely different way. There are:

如果您曾经开发过 Web 应用程序，为了实现付款，您可能会在 DB： "accounts" 和 "transactions".账户将存储有关用户的信息，包括他们的个人信息和余额，交易将存储有关从一个账户转移到另一个账户的信息。在比特币中，支付以完全不同的方式实现。有：

1. No accounts.没有帐户。
2. No balances.没有余额。
3. No addresses.没有地址。
4. No coins.没有硬币。
5. No senders and receivers.没有发送者和接收者。

Since blockchain is a public and open database, we don’t want to store sensitive information about wallet owners. Coins are not collected in accounts. Transactions do not transfer money from one address to another. There’s no field or attribute that holds account balance. There are only transactions. But what’s inside a transaction?

由于区块链是一个公开和开放的数据库，我们不想存储有关钱包所有者的敏感信息。硬币不会在帐户中收集。交易不会将资金从一个地址转移到另一个地址。没有用于保存帐户余额的字段或属性。只有交易。但是交易里面有什么呢？



## Bitcoin Transaction

A transaction is a combination of inputs and outputs:

事务是输入和输出的组合：

```go
type Transaction struct {
	ID   []byte
	Vin  []TXInput
	Vout []TXOutput
}
```

Inputs of a new transaction reference outputs of a previous transaction (there’s an exception though, which we’ll discuss later). Outputs are where coins are actually stored. The following diagram demonstrates the interconnection of transactions:

新事务的输入引用前一个事务的输出（但有一个例外，我们将在后面讨论）。输出是实际存储硬币的地方。下图演示了事务的互连：

![Transactions](https://jeiwan.net/images/transactions-diagram.png)

Notice that:

请注意：

1. There are outputs that are not linked to inputs.有些输出未链接到输入。
2. In one transaction, inputs can reference outputs from multiple transactions.在一个事务中，输入可以引用多个事务的输出。
3. An input must reference an output.输入必须引用输出。

Throughout this article, we’ll use words like “money”, “coins”, “spend”, “send”, “account”, etc. But there are no such concepts in Bitcoin. Transactions just lock values with a script, which can be unlocked only by the one who locked them.

在整篇文章中，我们将使用“金钱”、“硬币”、“花费”、“发送”、“帐户”等词。但是比特币中没有这样的概念。事务只是使用脚本锁定值，只有锁定它们的人才能解锁。

## Transaction Outputs 事务输出

Let’s start with outputs first:

让我们先从输出开始：

```go
type TXOutput struct {
	Value        int
	ScriptPubKey string
}
```

Actually, it’s outputs that store “coins” (notice the `Value` field above). And storing means locking them with a puzzle, which is stored in the `ScriptPubKey`. Internally, Bitcoin uses a scripting language called *Script*, that is used to define outputs locking and unlocking logic. The language is quite primitive (this is made intentionally, to avoid possible hacks and misuses), but we won’t discuss it in details. You can find a detailed explanation of it [here](https://en.bitcoin.it/wiki/Script).

实际上，它是存储“硬币”的输出（注意上面的 "Value" 字段）。存储意味着用一个谜题锁定它们，该谜题存储在 "ScriptPubKey".在内部，比特币使用一种称为脚本的脚本语言，用于定义输出锁定和解锁逻辑。该语言非常原始（这是故意制作的，以避免可能的黑客攻击和误用），但我们不会详细讨论它。你可以在这里找到它的详细说明。

> In Bitcoin, the *value* field stores the number of *satoshis*, not the number of BTC. A *satoshi* is a hundred millionth of a bitcoin (0.00000001 BTC), thus this is the smallest unit of currency in Bitcoin (like a cent).
>
> *在比特币中，value 字段存储的是 satoshis 的数量，而不是 BTC 的数量。聪是比特币（0.000000001 BTC）的一亿分之一，因此这是比特币中最小的货币单位（如一美分）。*

Since we don’t have addresses implemented, we’ll avoid the whole scripting related logic for now. `ScriptPubKey` will store an arbitrary string (user defined wallet address).

由于我们没有实现地址，因此我们暂时将避免整个脚本相关逻辑。 "ScriptPubKey" 将存储任意字符串（用户定义的钱包地址）。

> By the way, having such scripting language means that Bitcoin can be used as a smart-contract platform as well.
>
> *顺便说一句，拥有这样的脚本语言意味着比特币也可以用作智能合约平台。*

One important thing about outputs is that they are **indivisible**, which means that you cannot reference a part of its value. When an output is referenced in a new transaction, it’s spent as a whole. And if its value is greater than required, a change is generated and sent back to the sender. This is similar to a real world situation when you pay, say, a 5 banknote for something that costs 1 and get a change of $4.

关于输出的一件重要事情是它们是不可分割的，这意味着您不能引用其值的一部分。当输出在新事务中被引用时，它将作为一个整体使用。如果其值大于要求值，则会生成更改并将其发送回发送方。这类似于现实世界的情况，例如，您支付一张 5 美元的钞票，购买 1 美元的东西，然后换取 4 美元的零钱。

## Transaction Inputs 交易输入

And here’s the input:

```go
type TXInput struct {
	Txid      []byte
	Vout      int
	ScriptSig string
}
```

As mentioned earlier, an input references a previous output: `Txid` stores the ID of such transaction, and `Vout` stores an index of an output in the transaction. `ScriptSig` is a script which provides data to be used in an output’s `ScriptPubKey`. If the data is correct, the output can be unlocked, and its value can be used to generate new outputs; if it’s not correct, the output cannot be referenced in the input. This is the mechanism that guarantees that users cannot spend coins belonging to other people.

如前所述，输入引用先前"Txid"的输出：存储此类事务的 ID，并在事务 "Vout" 中存储输出的索引。 "ScriptSig" 是一个脚本，它提供要在输出中使用的数据"ScriptPubKey"。如果数据正确，则输出可以解锁，其值可用于生成新的输出;如果不正确，则无法在输入中引用输出。这是保证用户不能花费属于其他人的硬币的机制。

Again, since we don’t have addresses implemented yet, `ScriptSig` will store just an arbitrary user defined wallet address. We’ll implement public keys and signatures checking in the next article.

同样，由于我们还没有实现地址，因此 "ScriptSig" 将只存储任意用户定义的钱包地址。我们将在下一篇文章中实现公钥和签名检查。

Let’s sum it up. Outputs are where “coins” are stored. Each output comes with an unlocking script, which determines the logic of unlocking the output. Every new transaction must have at least one input and output. An input references an output from a previous transaction and provides data (the `ScriptSig` field) that is used in the output’s unlocking script to unlock it and use its value to create new outputs.

让我们总结一下。输出是存储“硬币”的地方。每个输出都带有一个解锁脚本，该脚本决定了解锁输出的逻辑。每个新事务必须至少有一个输入和输出。输入引用上一个事务的输出，并提供数据（字段"ScriptSig"），在输出的解锁脚本中用于解锁它，并使用其值创建新的输出。

But what came first: inputs or outputs?

但首先出现的是输入还是输出？

## The egg

In Bitcoin, it’s the egg that came before the chicken. The inputs-referencing-outputs logic is the classical “chicken or the egg” situation: inputs produce outputs and outputs make inputs possible. And in Bitcoin, outputs come before inputs.

在比特币中，它是先于鸡的鸡蛋。输入-引用-输出逻辑是经典的“先有鸡还是先有蛋”的情况：输入产生输出，输出使输入成为可能。在比特币中，输出先于输入。

When a miner starts mining a block, it adds a **coinbase transaction** to it. A coinbase transaction is a special type of transactions, which doesn’t require previously existing outputs. It creates outputs (i.e., “coins”) out of nowhere. The egg without a chicken. This is the reward miners get for mining new blocks.

当矿工开始挖掘一个区块时，它会向其添加一个 coinbase 交易。coinbase 交易是一种特殊类型的交易，它不需要以前存在的输出。它凭空创造输出（即“硬币”）。没有鸡的鸡蛋。这是矿工开采新区块获得的奖励。

As you know, there’s the genesis block in the beginning of a blockchain. It’s this block that generates the very first output in the blockchain. And no previous outputs are required since there are no previous transactions and no such outputs.

如您所知，区块链的开头有一个创世区块。正是这个区块在区块链中生成了第一个输出。并且不需要以前的输出，因为没有以前的交易，也没有这样的输出。

Let’s create a coinbase transaction:

让我们创建一个 coinbase 交易：

```go
func NewCoinbaseTX(to, data string) *Transaction {
	if data == "" {
		data = fmt.Sprintf("Reward to '%s'", to)
	}

	txin := TXInput{[]byte{}, -1, data}
	txout := TXOutput{subsidy, to}
	tx := Transaction{nil, []TXInput{txin}, []TXOutput{txout}}
	tx.SetID()

	return &tx
}
```

A coinbase transaction has only one input. In our implementation its `Txid` is empty and `Vout` equals to -1. Also, a coinbase transaction doesn’t store a script in `ScriptSig`. Instead, arbitrary data is stored there.

coinbase 交易只有一个输入。在我们的实现中，它是 "Txid" 空的， "Vout" 等于 -1。此外，coinbase 交易不会将脚本存储在 "ScriptSig".相反，任意数据存储在那里。

> In Bitcoin, the very first coinbase transaction contains the following message: “The Times 03/Jan/2009 Chancellor on brink of second bailout for banks”. [You can see it yourself](https://blockchain.info/tx/4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b?show_adv=true).
>
> *在比特币中，第一笔coinbase交易包含以下信息：“泰晤士报2009年1月3日财政大臣即将对银行进行第二次救助”。 你可以自己看到它。*

`subsidy` is the amount of reward. In Bitcoin, this number is not stored anywhere and calculated based only on the total number of blocks: the number of blocks is divided by `210000`. Mining the genesis block produced 50 BTC, and every `210000` blocks the reward is halved. In our implementation, we’ll store the reward as a constant (at least for now 😉 ).

"subsidy" 是奖励金额。在比特币中，这个数字不会存储在任何地方，而是仅根据区块总数计算：区块数除以 "210000"。挖出创世区块产生了 50 BTC，每个区块 "210000" 的奖励减半。在我们的实现中，我们会将奖励存储为常量（至少现在😉是这样）。

## Storing Transactions in Blockchain 在区块链中存储交易

From now on, every block must store at least one transaction and it’s no more possible to mine blocks without transactions. This means that we should remove the `Data` field of `Block` and store transactions instead:

从现在开始，每个区块必须至少存储一笔交易，没有交易就不可能再挖区块了。这意味着我们应该删除 "Data" 的字段 "Block" 并存储交易：

```go
type Block struct {
	Timestamp     int64
	Transactions  []*Transaction
	PrevBlockHash []byte
	Hash          []byte
	Nonce         int
}
```

`NewBlock` and `NewGenesisBlock` also must be changed accordingly:

```go
func NewBlock(transactions []*Transaction, prevBlockHash []byte) *Block {
	block := &Block{time.Now().Unix(), transactions, prevBlockHash, []byte{}, 0}
	...
}

func NewGenesisBlock(coinbase *Transaction) *Block {
	return NewBlock([]*Transaction{coinbase}, []byte{})
}
```

Next thing to change is the creation of a new blockchain:

```go
func CreateBlockchain(address string) *Blockchain {
	...
	err = db.Update(func(tx *bolt.Tx) error {
		cbtx := NewCoinbaseTX(address, genesisCoinbaseData)
		genesis := NewGenesisBlock(cbtx)

		b, err := tx.CreateBucket([]byte(blocksBucket))
		err = b.Put(genesis.Hash, genesis.Serialize())
		...
	})
	...
}
```

Now, the function takes an address which will receive the reward for mining the genesis block.

## Proof-of-Work

The Proof-of-Work algorithm must consider transactions stored in a block, to guarantee the consistency and reliability of blockchain as a storage of transaction. So now we must modify the `ProofOfWork.prepareData` method:

工作量证明算法必须考虑存储在区块中的交易，以保证区块链作为交易存储的一致性和可靠性。所以现在我们必须修改 "ProofOfWork.prepareData" 方法：

```go
func (pow *ProofOfWork) prepareData(nonce int) []byte {
	data := bytes.Join(
		[][]byte{
			pow.block.PrevBlockHash,
			pow.block.HashTransactions(), // This line was changed
			IntToHex(pow.block.Timestamp),
			IntToHex(int64(targetBits)),
			IntToHex(int64(nonce)),
		},
		[]byte{},
	)

	return data
}
```

Instead of `pow.block.Data` we now use `pow.block.HashTransactions()` which is:

而不是 "pow.block.Data" 我们现在使用的 "pow.block.HashTransactions()" 是：

```go
func (b *Block) HashTransactions() []byte {
	var txHashes [][]byte
	var txHash [32]byte

	for _, tx := range b.Transactions {
		txHashes = append(txHashes, tx.ID)
	}
	txHash = sha256.Sum256(bytes.Join(txHashes, []byte{}))

	return txHash[:]
}
```

Again, we’re using hashing as a mechanism of providing unique representation of data. We want all transactions in a block to be uniquely identified by a single hash. To achieve this, we get hashes of each transaction, concatenate them, and get a hash of the concatenated combination.

同样，我们使用哈希作为提供数据唯一表示的机制。我们希望区块中的所有交易都由单个哈希值唯一标识。为了实现这一点，我们获取每笔交易的哈希值，将它们连接起来，并得到连接组合的哈希值。

> Bitcoin uses a more elaborate technique: it represents all transactions containing in a block as a [Merkle tree](https://en.wikipedia.org/wiki/Merkle_tree) and uses the root hash of the tree in the Proof-of-Work system. This approach allows to quickly check if a block contains certain transaction, having only just the root hash and without downloading all the transactions.
>
> *比特币使用了一种更复杂的技术：它将区块中包含的所有交易表示为默克尔树，并在工作量证明系统中使用该树的根哈希值。这种方法允许快速检查区块是否包含某些交易，只有根哈希，而无需下载所有交易。*

Let’s check that everything is correct so far:

让我们检查一下到目前为止一切是否正确：

```shell
$ blockchain_go createblockchain -address Ivan
00000093450837f8b52b78c25f8163bb6137caf43ff4d9a01d1b731fa8ddcc8a

Done!
```

Good! We received out first mining reward. But how do we check the balance?

好！我们收到了第一个挖矿奖励。但是我们如何检查余额呢？

## Unspent Transaction Outputs未花费的交易输出

We need to find all unspent transaction outputs (UTXO). *Unspent* means that these outputs weren’t referenced in any inputs. On the diagram above, these are:

我们需要找到所有未花费的交易输出 （UTXO）。 “未使用”表示这些输出未在任何输入中引用。在上图中，这些是：

1. tx0, output 1;
2. tx1, output 0;
3. tx3, output 0;
4. tx4, output 0.

Of course, when we check balance, we don’t need all of them, but only those that can be unlocked by the key we own (currently we don’t have keys implemented and will use user defined addresses instead). First, let’s define locking-unlocking methods on inputs and outputs:

当然，当我们检查余额时，我们不需要所有余额，而只需要那些可以通过我们拥有的密钥解锁的密钥（目前我们没有实现密钥，而是使用用户定义的地址）。首先，让我们定义输入和输出的锁定-解锁方法：

```go
func (in *TXInput) CanUnlockOutputWith(unlockingData string) bool {
	return in.ScriptSig == unlockingData
}

func (out *TXOutput) CanBeUnlockedWith(unlockingData string) bool {
	return out.ScriptPubKey == unlockingData
}
```

Here we just compare the script fields with `unlockingData`. These pieces will be improved in a future article, after we implement addresses based on private keys.

在这里，我们只是将脚本字段与 进行比较"unlockingData"。在我们实现基于私钥的地址之后，这些部分将在以后的文章中得到改进。

The next step - finding transactions containing unspent outputs - is quite difficult:

下一步 - 查找包含未花费输出的交易 - 非常困难：

```go
func (bc *Blockchain) FindUnspentTransactions(address string) []Transaction {
  var unspentTXs []Transaction
  spentTXOs := make(map[string][]int)
  bci := bc.Iterator()

  for {
    block := bci.Next()

    for _, tx := range block.Transactions {
      txID := hex.EncodeToString(tx.ID)

    Outputs:
      for outIdx, out := range tx.Vout {
        // Was the output spent?
        if spentTXOs[txID] != nil {
          for _, spentOut := range spentTXOs[txID] {
            if spentOut == outIdx {
              continue Outputs
            }
          }
        }

        if out.CanBeUnlockedWith(address) {
          unspentTXs = append(unspentTXs, *tx)
        }
      }

      if tx.IsCoinbase() == false {
        for _, in := range tx.Vin {
          if in.CanUnlockOutputWith(address) {
            inTxID := hex.EncodeToString(in.Txid)
            spentTXOs[inTxID] = append(spentTXOs[inTxID], in.Vout)
          }
        }
      }
    }

    if len(block.PrevBlockHash) == 0 {
      break
    }
  }

  return unspentTXs
}
```

Since transactions are stored in blocks, we have to check every block in a blockchain. We start with outputs:

由于交易存储在区块中，因此我们必须检查区块链中的每个区块。我们从输出开始：

```go
if out.CanBeUnlockedWith(address) {
	unspentTXs = append(unspentTXs, tx)
}
```

If an output was locked by the same address we’re searching unspent transaction outputs for, then this is the output we want. But before taking it, we need to check if an output was already referenced in an input:

如果输出被我们搜索未花费交易输出的同一地址锁定，那么这就是我们想要的输出。但在获取它之前，我们需要检查输出是否已经在输入中被引用：

```go
if spentTXOs[txID] != nil {
	for _, spentOut := range spentTXOs[txID] {
		if spentOut == outIdx {
			continue Outputs
		}
	}
}
```

We skip those that were referenced in inputs (their values were moved to other outputs, thus we cannot count them). After checking outputs we gather all inputs that could unlock outputs locked with the provided address (this doesn’t apply to coinbase transactions, since they don’t unlock outputs):

我们跳过那些在输入中引用的那些（它们的值被移动到其他输出，因此我们无法计算它们）。检查输出后，我们收集所有可以解锁使用提供的地址锁定的输出的输入（这不适用于 coinbase 交易，因为它们不会解锁输出）：

```go
if tx.IsCoinbase() == false {
    for _, in := range tx.Vin {
        if in.CanUnlockOutputWith(address) {
            inTxID := hex.EncodeToString(in.Txid)
            spentTXOs[inTxID] = append(spentTXOs[inTxID], in.Vout)
        }
    }
}
```

The function returns a list of transactions containing unspent outputs. To calculate balance we need one more function that takes the transactions and returns only outputs:

该函数返回包含未花费输出的事务列表。为了计算余额，我们还需要一个函数来接受交易并仅返回输出：

```go
func (bc *Blockchain) FindUTXO(address string) []TXOutput {
       var UTXOs []TXOutput
       unspentTransactions := bc.FindUnspentTransactions(address)

       for _, tx := range unspentTransactions {
               for _, out := range tx.Vout {
                       if out.CanBeUnlockedWith(address) {
                               UTXOs = append(UTXOs, out)
                       }
               }
       }

       return UTXOs
}
```

That’s it! Now we can implement `getbalance` command:

就是这样！现在我们可以实现 "getbalance" 命令了：

```go
func (cli *CLI) getBalance(address string) {
	bc := NewBlockchain(address)
	defer bc.db.Close()

	balance := 0
	UTXOs := bc.FindUTXO(address)

	for _, out := range UTXOs {
		balance += out.Value
	}

	fmt.Printf("Balance of '%s': %d\n", address, balance)
}
```

The account balance is the sum of values of all unspent transaction outputs locked by the account address.

账户余额是账户地址锁定的所有未花费交易输出的价值之和。

Let’s check our balance after mining the genesis block:

让我们在挖掘创世区块后检查一下我们的余额：

```shell
$ blockchain_go getbalance -address Ivan
Balance of 'Ivan': 10
```

This is our first money!

这是我们的第一笔钱！

## Sending Coins 发送硬币

Now, we want to send some coins to someone else. For this, we need to create a new transaction, put it in a block, and mine the block. So far, we implemented only the coinbase transaction (which is a special type of transactions), now we need a general transaction:

现在，我们想向其他人发送一些硬币。为此，我们需要创建一个新交易，将其放入一个区块中，然后挖掘该区块。到目前为止，我们只实现了 coinbase 事务（这是一种特殊类型的事务），现在我们需要一个通用事务：

```go
func NewUTXOTransaction(from, to string, amount int, bc *Blockchain) *Transaction {
	var inputs []TXInput
	var outputs []TXOutput

	acc, validOutputs := bc.FindSpendableOutputs(from, amount)

	if acc < amount {
		log.Panic("ERROR: Not enough funds")
	}

	// Build a list of inputs
	for txid, outs := range validOutputs {
		txID, err := hex.DecodeString(txid)

		for _, out := range outs {
			input := TXInput{txID, out, from}
			inputs = append(inputs, input)
		}
	}

	// Build a list of outputs
	outputs = append(outputs, TXOutput{amount, to})
	if acc > amount {
		outputs = append(outputs, TXOutput{acc - amount, from}) // a change
	}

	tx := Transaction{nil, inputs, outputs}
	tx.SetID()

	return &tx
}
```

Before creating new outputs, we first have to find all unspent outputs and ensure that they store enough value. This is what `FindSpendableOutputs` method does. After that, for each found output an input referencing it is created. Next, we create two outputs:

在创建新输出之前，我们首先必须找到所有未使用的输出，并确保它们存储了足够的价值。这就是 "FindSpendableOutputs" 方法的作用。之后，为每个找到的输出创建一个引用它的输入。接下来，我们创建两个输出：

1. One that’s locked with the receiver address. This is the actual transferring of coins to other address.一个与接收方地址锁定的地址。这是硬币到其他地址的实际转移。
2. One that’s locked with the sender address. This is a change. It’s only created when unspent outputs hold more value than required for the new transaction. Remember: outputs are **indivisible**.一个与发件人地址一起锁定的地址。这是一个变化。仅当未花费的输出持有的价值超过新事务所需的价值时，才会创建它。请记住：输出是**indivisible不可分割**.。

`FindSpendableOutputs` method is based on the `FindUnspentTransactions` method we defined earlier:

```go
func (bc *Blockchain) FindSpendableOutputs(address string, amount int) (int, map[string][]int) {
	unspentOutputs := make(map[string][]int)
	unspentTXs := bc.FindUnspentTransactions(address)
	accumulated := 0

Work:
	for _, tx := range unspentTXs {
		txID := hex.EncodeToString(tx.ID)

		for outIdx, out := range tx.Vout {
			if out.CanBeUnlockedWith(address) && accumulated < amount {
				accumulated += out.Value
				unspentOutputs[txID] = append(unspentOutputs[txID], outIdx)

				if accumulated >= amount {
					break Work
				}
			}
		}
	}

	return accumulated, unspentOutputs
}
```

The method iterates over all unspent transactions and accumulates their values. When the accumulated value is more or equals to the amount we want to transfer, it stops and returns the accumulated value and output indices grouped by transaction IDs. We don’t want to take more than we’re going to spend.

该方法遍历所有未花费的事务并累积其值。当累计值大于或等于我们要转账的金额时，它会停止并返回累积值和按交易 ID 分组的输出指数。我们不想拿走比我们要花的更多的钱。

Now we can modify the `Blockchain.MineBlock` method:

```go
func (bc *Blockchain) MineBlock(transactions []*Transaction) {
	...
	newBlock := NewBlock(transactions, lastHash)
	...
}
```

Finally, let’s implement `send` command:

```go
func (cli *CLI) send(from, to string, amount int) {
	bc := NewBlockchain(from)
	defer bc.db.Close()

	tx := NewUTXOTransaction(from, to, amount, bc)
	bc.MineBlock([]*Transaction{tx})
	fmt.Println("Success!")
}
```

Sending coins means creating a transaction and adding it to the blockchain via mining a block. But Bitcoin doesn’t do this immediately (as we do). Instead, it puts all new transactions into memory pool (or mempool), and when a miner is ready to mine a block, it takes all transactions from the mempool and creates a candidate block. Transactions become confirmed only when a block containing them is mined and added to the blockchain.

发送硬币意味着创建一个交易，并通过挖掘一个区块将其添加到区块链中。但比特币不会立即这样做（就像我们所做的那样）。取而代之的是，它将所有新交易放入内存池（或内存池）中，当矿工准备挖掘一个区块时，它会从内存池中获取所有交易并创建一个候选区块。只有当包含交易的区块被挖掘并添加到区块链中时，交易才会得到确认。

Let’s check that sending coins works:

```shell
$ blockchain_go send -from Ivan -to Pedro -amount 6
00000001b56d60f86f72ab2a59fadb197d767b97d4873732be505e0a65cc1e37

Success!

$ blockchain_go getbalance -address Ivan
Balance of 'Ivan': 4

$ blockchain_go getbalance -address Pedro
Balance of 'Pedro': 6
```

Nice! Now, let’s create more transactions and ensure that sending from multiple outputs works fine:

```shell
$ blockchain_go send -from Pedro -to Helen -amount 2
00000099938725eb2c7730844b3cd40209d46bce2c2af9d87c2b7611fe9d5bdf

Success!

$ blockchain_go send -from Ivan -to Helen -amount 2
000000a2edf94334b1d94f98d22d7e4c973261660397dc7340464f7959a7a9aa

Success!
```

Now, Helen’s coins are locked in two outputs: one from Pedro and one from Ivan. Let’s send them to someone else:

```shell
$ blockchain_go send -from Helen -to Rachel -amount 3
000000c58136cffa669e767b8f881d16e2ede3974d71df43058baaf8c069f1a0

Success!

$ blockchain_go getbalance -address Ivan
Balance of 'Ivan': 2

$ blockchain_go getbalance -address Pedro
Balance of 'Pedro': 4

$ blockchain_go getbalance -address Helen
Balance of 'Helen': 1

$ blockchain_go getbalance -address Rachel
Balance of 'Rachel': 3
```

Looks fine! Now let’s test a failure:

```shell
$ blockchain_go send -from Pedro -to Ivan -amount 5
panic: ERROR: Not enough funds

$ blockchain_go getbalance -address Pedro
Balance of 'Pedro': 4

$ blockchain_go getbalance -address Ivan
Balance of 'Ivan': 2
```

## Conclusion

Phew! It wasn’t easy, but we have transactions now! Although, some key features of a Bitcoin-like cryptocurrency are missing:

唷！这并不容易，但我们现在有交易了！虽然，缺少类似比特币的加密货币的一些关键特征：

1. Addresses. We don’t have real, private key based addresses yet.地址。我们还没有真正的、基于私钥的地址。
2. Rewards. Mining blocks is absolutely not profitable!奖励。挖块绝对是无利可图的！
3. UTXO set. Getting balance requires scanning the whole blockchain, which can take very long time when there are many and many blocks. Also, it can take a lot of time if we want to validate later transactions. UTXO set is intended to solve these problems and make operations with transactions fast.UTXO 集。获得平衡需要扫描整个区块链，当有很多很多区块时，这可能需要很长时间。此外，如果我们想验证以后的交易，可能需要很多时间。UTXO 集旨在解决这些问题，并快速进行交易操作。
4. Mempool. This is where transactions are stored before being packed in a block. In our current implementation, a block contains only one transaction, and this is quite inefficient.内存池。这是交易在打包到区块中之前存储的地方。在我们目前的实现中，一个区块只包含一个交易，这是相当低效的。

Links:

1. [Full source codes](https://github.com/Jeiwan/blockchain_go/tree/part_4)
2. [Transaction](https://en.bitcoin.it/wiki/Transaction)
3. [Merkle tree](https://en.bitcoin.it/wiki/Protocol_documentation#Merkle_Trees)
4. [Coinbase](https://en.bitcoin.it/wiki/Coinbase)
