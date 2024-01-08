# Building Blockchain in Go. Part 3: Persistence and CLI 持久性和 CLI

## Introduction介绍

##### [So](https://jeiwan.net/posts/building-blockchain-in-go-part-1/) [far](https://jeiwan.net/posts/building-blockchain-in-go-part-2/), we’ve built a blockchain with a proof-of-work system, which makes mining possible. Our implementation is getting closer to a fully functional blockchain, but it still lacks some important features. Today will start storing a blockchain in a database, and after that we’ll make a simple command-line interface to perform operations with the blockchain. In its essence, blockchain is a distributed database. We’re going to omit the “distributed” part for now and focus on the “database” part.

到目前为止，我们已经建立了一个带有工作量证明系统的区块链，这使得挖矿成为可能。我们的实现越来越接近一个功能齐全的区块链，但它仍然缺乏一些重要的功能。今天将开始将区块链存储在数据库中，之后我们将制作一个简单的命令行界面来执行区块链操作。从本质上讲，区块链是一个分布式数据库。我们现在将省略“分布式”部分，而专注于“数据库”部分。

## Database Choice数据库选择

###### Currently, there’s no database in our implementation; instead, we create blocks every time we run the program and store them in memory. We cannot reuse a blockchain, we cannot share it with others, thus we need to store it on the disk.

目前，我们的实现中没有数据库;取而代之的是，我们每次运行程序时都会创建块并将它们存储在内存中。我们不能重用区块链，我们不能与他人共享它，因此我们需要将其存储在磁盘上。

###### Which database do we need? Actually, any of them. In [the original Bitcoin paper](https://bitcoin.org/bitcoin.pdf), nothing is said about using a certain database, so it’s up to a developer what DB to use. [Bitcoin Core](https://github.com/bitcoin/bitcoin), which was initially published by Satoshi Nakamoto and which is currently a reference implementation of Bitcoin, uses [LevelDB](https://github.com/google/leveldb) (although it was introduced to the client only in 2012). And we’ll use…

我们需要哪个数据库？实际上，他们中的任何一个。在最初的比特币论文中，没有提到使用某个数据库，因此由开发人员决定使用什么数据库。 Bitcoin Core 最初由 Satoshi Nakamoto 发布，目前是比特币的参考实现，它使用 LevelDB（尽管它仅在 2012 年才引入客户端）。我们将使用...

## BoltDB

Because:

因为：

1. It’s simple and minimalistic.它简单而简约。
2. It’s implemented in Go.它是在 Go 中实现的。
3. It doesn’t require to run a server.它不需要运行服务器。
4. It allows to build the data structure we want.它允许构建我们想要的数据结构。

From the BoltDB’s [README on Github](https://github.com/boltdb/bolt):

来自 Github 上 BoltDB 的 README：

> Bolt is a pure Go key/value store inspired by Howard Chu’s LMDB project. The goal of the project is to provide a simple, fast, and reliable database for projects that don’t require a full database server such as Postgres or MySQL.
>
> Bolt 是一个纯 Go 键/值存储，灵感来自 Howard Chu 的 LMDB 项目。该项目的目标是为不需要完整数据库服务器的项目（如 Postgres 或 MySQL）提供一个简单、快速和可靠的数据库。

> Since Bolt is meant to be used as such a low-level piece of functionality, simplicity is key. The API will be small and only focus on getting values and setting values. That’s it.
>
> 由于 Bolt 旨在用作此类低级功能，因此简单性是关键。API 将很小，只专注于获取值和设置值。就是这样。

Sounds perfect for our needs! Let’s spend a minute reviewing it.

听起来非常适合我们的需求！让我们花一分钟时间回顾一下。

BoltDB is a key/value storage, which means there’re no tables like in SQL RDBMS (MySQL, PostgreSQL, etc.), no rows, no columns. Instead, data is stored as key-value pairs (like in Golang maps). Key-value pairs are stored in buckets, which are intended to group similar pairs (this is similar to tables in RDBMS). Thus, in order to get a value, you need to know a bucket and a key.

BoltDB 是一个键/值存储，这意味着没有像 SQL RDBMS（MySQL、PostgreSQL 等）那样的表，没有行，没有列。相反，数据存储为键值对（如在 Golang 映射中）。键值对存储在存储桶中，用于对相似的对进行分组（这类似于 RDBMS 中的表）。因此，为了获得一个值，你需要知道一个存储桶和一个键。

One important thing about BoltDB is that there are no data types: keys and values are byte arrays. Since we’ll store Go structs (`Block`, in particular) in it, we’ll need to serialize them, i.e. implement a mechanism of converting a Go struct into a byte array and restoring it back from a byte array. We’ll use [encoding/gob](https://golang.org/pkg/encoding/gob/) for this, but `JSON`, `XML`, `Protocol Buffers`, etc. can be used as well. We’re using `encoding/gob` because it’s simple and is a part of the standard Go library.

BoltDB 的一个重要方面是没有数据类型：键和值是字节数组。由于我们将"Block" Go 结构体（特别是）存储在其中，因此我们需要序列化它们，即实现一种将 Go 结构体转换为字节数组并从字节数组恢复的机制。为此，我们将使用 encoding/gob，但"JSON"也可以使用 、 、 "XML" "Protocol Buffers"等。我们之所以使用 "encoding/gob" ，是因为它很简单，并且是标准 Go 库的一部分。

## Database Structure数据库结构

Before starting implementing persistence logic, we first need to decide how we’ll store data in the DB. And for this, we’ll refer to the way Bitcoin Core does that.

在开始实现持久性逻辑之前，我们首先需要决定如何在数据库中存储数据。为此，我们将参考 Bitcoin Core 做到这一点的方式。

In simple words, Bitcoin Core uses two “buckets” to store data:

简单来说，Bitcoin Core使用两个“桶”来存储数据：

1. `blocks` stores metadata describing all the blocks in a chain.存储描述链中所有块的元数据。
2. `chainstate` stores the state of a chain, which is all currently unspent transaction outputs and some metadata.存储链的状态，即所有当前未花费的交易输出和一些元数据。

Also, blocks are stored as separate files on the disk. This is done for a performance purpose: reading a single block won’t require loading all (or some) of them into memory. We won’t implement this.

此外，块作为单独的文件存储在磁盘上。这样做是出于性能目的：读取单个块不需要将全部（或部分）块加载到内存中。我们不会实现这一点。

In `blocks`, the `key -> value` pairs are:

在 中"blocks"，这些 "key -> value" 对是：

1. `'b' + 32-byte block hash -> block index record`
2. `'f' + 4-byte file number -> file information record`
3. `'l' -> 4-byte file number: the last block file number used`
4. `'R' -> 1-byte boolean: whether we're in the process of reindexing`
5. `'F' + 1-byte flag name length + flag name string -> 1 byte boolean: various flags that can be on or off`
6. `'t' + 32-byte transaction hash -> transaction index record`

In `chainstate`, the `key -> value` pairs are:

在 中"chainstate"，这些 "key -> value" 对是：

1. `'c' + 32-byte transaction hash -> unspent transaction output record for that transaction`
2. `'B' -> 32-byte block hash: the block hash up to which the database represents the unspent transaction outputs`

*(Detailed explanation can be found [here](https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_2):_Data_Storage))*

（详细说明可以在这里找到）

Since we don’t have transactions yet, we’re going to have only `blocks` bucket. Also, as said above, we will store the whole DB as a single file, without storing blocks in separate files. So we won’t need anything related to file numbers. So these are `key -> value` pairs we’ll use:

由于我们还没有事务，因此我们将只有 "blocks" 存储桶。此外，如上所述，我们将整个数据库存储为单个文件，而不会将块存储在单独的文件中。因此，我们不需要任何与文件编号相关的内容。因此， "key -> value" 我们将使用这些对：

1. `32-byte block-hash -> Block structure (serialized)`
2. `'l' -> the hash of the last block in a chain`

That’s all we need to know to start implementing the persistence mechanism.

这就是我们开始实现持久性机制所需要知道的全部内容。



## Serialization序列化

As said before, in BoltDB values can be only of `[]byte` type, and we want to store `Block` structs in the DB. We’ll use [encoding/gob](https://golang.org/pkg/encoding/gob/) to serialize the structs.

如前所述，在 BoltDB 中，值只能是 "[]byte" 类型，我们希望"Block"将结构存储在数据库中。我们将使用 encoding/gob 来序列化结构。

Let’s implement `Serialize` method of `Block` (errors processing is omitted for brevity):

让我们实现 "Serialize" 的方法 "Block" （为简洁起见，省略了错误处理）：

```go
func (b *Block) Serialize() []byte {
	var result bytes.Buffer
	encoder := gob.NewEncoder(&result)

	err := encoder.Encode(b)

	return result.Bytes()
}
```

The piece is straightforward: at first, we declare a buffer that will store serialized data; then we initialize a `gob` encoder and encode the block; the result is returned as a byte array.

这部分很简单：首先，我们声明一个存储序列化数据的缓冲区;然后我们初始化一个"gob"编码器并对块进行编码;结果以字节数组的形式返回。

Next, we need a deserializing function that will receive a byte array as input and return a `Block`. This won’t be a method but an independent function:

接下来，我们需要一个反序列化函数，该函数将接收一个字节数组作为输入并返回一个 "Block".这不是一个方法，而是一个独立的函数：

```go
func DeserializeBlock(d []byte) *Block {
	var block Block

	decoder := gob.NewDecoder(bytes.NewReader(d))
	err := decoder.Decode(&block)

	return &block
}
```

And that’s it for the serialization!

序列化就是这样！

## Persistence

Let’s start with the `NewBlockchain` function. Currently, it creates a new instance of `Blockchain` and adds the genesis block to it. What we want it to do is to:

让我们从函数开始 "NewBlockchain" 。目前，它创建一个新实例 "Blockchain" 并向其添加创世块。我们希望它做的是：

1. Open a DB file.打开数据库文件。

2. Check if there’s a blockchain stored in it.检查其中是否存储了区块链。

3. If there’s a blockchain: 

   如果有区块链：

   1. Create a new 创建一个新 `Blockchain` instance. 实例。
   2. Set the tip of the 将`Blockchain` instance to the last block hash stored in the DB.设置为存储在数据库中的最后一个块哈希值。

4. If there’s no existing blockchain: 

   如果没有现成的区块链：

   1. Create the genesis block.创建创世区块。
   2. Store in the DB.存储在数据库中。
   3. Save the genesis block’s hash as the last block hash.将创世区块的哈希值保存为最后一个区块哈希值。
   4. Create a new 创建一个新 `Blockchain` instance with its tip pointing at the genesis block. 实例，其尖端指向创世块。

In code, it looks like this:

在代码中，它如下所示：

```go
func NewBlockchain() *Blockchain {
	var tip []byte
	db, err := bolt.Open(dbFile, 0600, nil)

	err = db.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))

		if b == nil {
			genesis := NewGenesisBlock()
			b, err := tx.CreateBucket([]byte(blocksBucket))
			err = b.Put(genesis.Hash, genesis.Serialize())
			err = b.Put([]byte("l"), genesis.Hash)
			tip = genesis.Hash
		} else {
			tip = b.Get([]byte("l"))
		}

		return nil
	})

	bc := Blockchain{tip, db}

	return &bc
}
```



Let’s review this piece by piece.

让我们一点一点地回顾一下。

```go
db, err := bolt.Open(dbFile, 0600, nil)
```

This is a standard way of opening a BoltDB file. Notice that it won’t return an error if there’s no such file.

这是打开BoltDB文件的标准方法。请注意，如果没有此类文件，它不会返回错误。

```go
err = db.Update(func(tx *bolt.Tx) error {
...
})
```

In BoltDB, operations with a database are run within a transaction. And there are two types of transactions: read-only and read-write. Here, we open a read-write transaction (`db.Update(...)`), because we expect to put the genesis block in the DB.

在 BoltDB 中，数据库操作在事务中运行。有两种类型的事务：只读和读写。在这里，我们打开一个读写事务（"db.Update(...)"），因为我们期望将创世块放在数据库中。

```go
b := tx.Bucket([]byte(blocksBucket))

if b == nil {
	genesis := NewGenesisBlock()
	b, err := tx.CreateBucket([]byte(blocksBucket))
	err = b.Put(genesis.Hash, genesis.Serialize())
	err = b.Put([]byte("l"), genesis.Hash)
	tip = genesis.Hash
} else {
	tip = b.Get([]byte("l"))
}
```



This is the core of the function. Here, we obtain the bucket storing our blocks: if it exists, we read the `l` key from it; if it doesn’t exist, we generate the genesis block, create the bucket, save the block into it, and update the `l` key storing the last block hash of the chain.

这是功能的核心。在这里"l"，我们获得存储区块的存储桶：如果它存在，我们从中读取密钥;如果它不存在，我们生成创世区块，创建存储桶，将区块保存到其中，并更新 "l" 存储链最后一个区块哈希值的密钥。

Also, notice the new way of creating a `Blockchain`:

另外，请注意创建 "Blockchain"：

```go
bc := Blockchain{tip, db}
```

We don’t store all the blocks in it anymore, instead only the tip of the chain is stored. Also, we store a DB connection, because we want to open it once and keep it open while the program is running. Thus, the `Blockchain` structure now looks like this:

我们不再将所有块都存储在其中，而是只存储链的尖端。此外，我们存储一个数据库连接，因为我们希望打开它一次，并在程序运行时保持打开状态。因此，结构 "Blockchain" 现在如下所示：

```go
type Blockchain struct {
	tip []byte
	db  *bolt.DB
}
```

Next thing we want to update is the `AddBlock` method: adding blocks to a chain now is not as easy as adding an element to an array. From now on we’ll store blocks in the DB:

接下来我们要更新的是 "AddBlock" 方法：现在向链中添加块并不像向数组中添加元素那么容易。从现在开始，我们将在数据库中存储块：

```go
func (bc *Blockchain) AddBlock(data string) {
	var lastHash []byte

	err := bc.db.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		lastHash = b.Get([]byte("l"))

		return nil
	})

	newBlock := NewBlock(data, lastHash)

	err = bc.db.Update(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		err := b.Put(newBlock.Hash, newBlock.Serialize())
		err = b.Put([]byte("l"), newBlock.Hash)
		bc.tip = newBlock.Hash

		return nil
	})
}
```

Let’s review this piece by piece:

让我们逐一回顾一下：

```go
err := bc.db.View(func(tx *bolt.Tx) error {
	b := tx.Bucket([]byte(blocksBucket))
	lastHash = b.Get([]byte("l"))

	return nil
})
```

This is the other (read-only) type of BoltDB transactions. Here we get the last block hash from the DB to use it to mine a new block hash.

```go
newBlock := NewBlock(data, lastHash)
b := tx.Bucket([]byte(blocksBucket))
err := b.Put(newBlock.Hash, newBlock.Serialize())
err = b.Put([]byte("l"), newBlock.Hash)
bc.tip = newBlock.Hash
```

After mining a new block, we save its serialized representation into the DB and update the `l` key, which now stores the new block’s hash.

Done! It wasn’t hard, was it?

## Inspecting Blockchain 检查区块链

All new blocks are now saved in a database, so we can reopen a blockchain and add a new block to it. But after implementing this, we lost a nice feature: we cannot print out blockchain blocks anymore because we don’t store blocks in an array any longer. Let’s fix this flaw!

现在，所有新区块都保存在数据库中，因此我们可以重新打开区块链并向其添加新区块。但是在实现这一点之后，我们失去了一个很好的功能：我们不能再打印出区块链区块了，因为我们不再将区块存储在数组中。让我们修复这个缺陷！

BoltDB allows to iterate over all the keys in a bucket, but the keys are stored in byte-sorted order, and we want blocks to be printed in the order they take in a blockchain. Also, because we don’t want to load all the blocks into memory (our blockchain DB could be huge!.. or let’s just pretend it could), we’ll read them one by one. For this purpose, we’ll need a blockchain iterator:

BoltDB 允许遍历存储桶中的所有密钥，但密钥是按字节排序顺序存储的，我们希望区块按照它们在区块链中的顺序打印。此外，因为我们不想将所有块加载到内存中（我们的区块链数据库可能很大..或者让我们假装可以），我们将一个接一个地读取它们。为此，我们需要一个区块链迭代器：

```go
type BlockchainIterator struct {
	currentHash []byte
	db          *bolt.DB
}
```

An iterator will be created each time we want to iterate over blocks in a blockchain and it’ll store the block hash of the current iteration and a connection to a DB. Because of the latter, an iterator is logically attached to a blockchain (it’s a `Blockchain` instance that stores a DB connection) and, thus, is created in a `Blockchain` method:

每次我们想遍历区块链中的区块时，都会创建一个迭代器，它将存储当前迭代的区块哈希值和与数据库的连接。由于后者，迭代器在逻辑上附加到区块链（它是 "Blockchain" 存储数据库连接的实例），因此在方法中创建 "Blockchain" ：

```go
func (bc *Blockchain) Iterator() *BlockchainIterator {
	bci := &BlockchainIterator{bc.tip, bc.db}

	return bci
}
```

Notice that an iterator initially points at the tip of a blockchain, thus blocks will be obtained from top to bottom, from newest to oldest. In fact, **choosing a tip means “voting” for a blockchain**. A blockchain can have multiple branches, and it’s the longest of them that’s considered main. After getting a tip (it can be any block in the blockchain) we can reconstruct the whole blockchain and find its length and the work required to build it. This fact also means that a tip is a kind of an identifier of a blockchain.

请注意，迭代器最初指向区块链的顶端，因此区块将从上到下，从最新到最旧获得。事实上，选择小费意味着为区块链“投票”。区块链可以有多个分支，其中最长的分支被认为是主要的。在获得提示（可以是区块链中的任何区块）后，我们可以重建整个区块链并找到其长度和构建它所需的工作。这一事实也意味着小费是区块链的一种标识符。

`BlockchainIterator` will do only one thing: it’ll return the next block from a blockchain.

"BlockchainIterator"只会做一件事：它将从区块链返回下一个区块。

```go
func (i *BlockchainIterator) Next() *Block {
	var block *Block

	err := i.db.View(func(tx *bolt.Tx) error {
		b := tx.Bucket([]byte(blocksBucket))
		encodedBlock := b.Get(i.currentHash)
		block = DeserializeBlock(encodedBlock)

		return nil
	})

	i.currentHash = block.PrevBlockHash

	return block
}
```

That’s it for the DB part!

DB部分就到此为止！

## CLI

Until now our implementation hasn’t provided any interface to interact with the program: we’ve simply executed `NewBlockchain`, `bc.AddBlock` in the `main` function. Time to improve this! We want to have these commands:

到目前为止，我们的实现还没有提供任何与程序交互的接口：我们只是 "NewBlockchain" 在函数中"bc.AddBlock"执行了 。 "main"是时候改进这一点了！我们希望有这些命令：

```
blockchain_go addblock "Pay 0.031337 for a coffee"
blockchain_go printchain
```

All command-line related operations will be processed by the `CLI` struct:

所有与命令行相关的操作都将由结构体处理 "CLI" ：

```go
type CLI struct {
	bc *Blockchain
}
```

Its “entrypoint” is the `Run` function:

它的“入口点”是 "Run" 函数：

```go
func (cli *CLI) Run() {
	cli.validateArgs()

	addBlockCmd := flag.NewFlagSet("addblock", flag.ExitOnError)
	printChainCmd := flag.NewFlagSet("printchain", flag.ExitOnError)

	addBlockData := addBlockCmd.String("data", "", "Block data")

	switch os.Args[1] {
	case "addblock":
		err := addBlockCmd.Parse(os.Args[2:])
	case "printchain":
		err := printChainCmd.Parse(os.Args[2:])
	default:
		cli.printUsage()
		os.Exit(1)
	}

	if addBlockCmd.Parsed() {
		if *addBlockData == "" {
			addBlockCmd.Usage()
			os.Exit(1)
		}
		cli.addBlock(*addBlockData)
	}

	if printChainCmd.Parsed() {
		cli.printChain()
	}
}
```

We’re using the standard [flag](https://golang.org/pkg/flag/) package to parse command-line arguments.

我们使用标准标志包来解析命令行参数。

```go
addBlockCmd := flag.NewFlagSet("addblock", flag.ExitOnError)
printChainCmd := flag.NewFlagSet("printchain", flag.ExitOnError)
addBlockData := addBlockCmd.String("data", "", "Block data")
```

First, we create two subcommands, `addblock` and `printchain`, then we add `-data` flag to the former. `printchain` won’t have any flags.

首先，我们创建两个子命令， "addblock" 然后"printchain"为"-data"前者添加标志。 "printchain" 不会有任何标志。

```go
switch os.Args[1] {
case "addblock":
	err := addBlockCmd.Parse(os.Args[2:])
case "printchain":
	err := printChainCmd.Parse(os.Args[2:])
default:
	cli.printUsage()
	os.Exit(1)
}
```

Next we check the command provided by user and parse related `flag` subcommand.

接下来，我们检查用户提供的命令并解析相关的 "flag" 子命令。

```go
if addBlockCmd.Parsed() {
	if *addBlockData == "" {
		addBlockCmd.Usage()
		os.Exit(1)
	}
	cli.addBlock(*addBlockData)
}

if printChainCmd.Parsed() {
	cli.printChain()
}
```

Next we check which of the subcommands were parsed and run related functions.

接下来，我们检查解析了哪些子命令并运行相关函数。

```go
func (cli *CLI) addBlock(data string) {
	cli.bc.AddBlock(data)
	fmt.Println("Success!")
}

func (cli *CLI) printChain() {
	bci := cli.bc.Iterator()

	for {
		block := bci.Next()

		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
		fmt.Printf("Data: %s\n", block.Data)
		fmt.Printf("Hash: %x\n", block.Hash)
		pow := NewProofOfWork(block)
		fmt.Printf("PoW: %s\n", strconv.FormatBool(pow.Validate()))
		fmt.Println()

		if len(block.PrevBlockHash) == 0 {
			break
		}
	}
}
```

This piece is very similar to the one we had before. The only difference is that we’re now using a `BlockchainIterator` to iterate over blocks in a blockchain.

这件作品与我们之前的作品非常相似。唯一的区别是，我们现在使用 a "BlockchainIterator" 来迭代区块链中的区块。

Also let’s not forget to modify the `main` function accordingly:

另外，我们不要忘记相应地修改 "main" 函数：

```go
func main() {
	bc := NewBlockchain()
	defer bc.db.Close()

	cli := CLI{bc}
	cli.Run()
}
```

Note that a new `Blockchain` is created no matter what command-line arguments are provided.

请注意"Blockchain"，无论提供什么命令行参数，都会创建一个新的参数。

And that’s it! Let’s check that everything works as expected:

就是这样！让我们检查一下一切是否按预期工作：

```shell
$ blockchain_go printchain
No existing blockchain found. Creating a new one...
Mining the block containing "Genesis Block"
000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b

Prev. hash:
Data: Genesis Block
Hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
PoW: true

$ blockchain_go addblock -data "Send 1 BTC to Ivan"
Mining the block containing "Send 1 BTC to Ivan"
000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13

Success!

$ blockchain_go addblock -data "Pay 0.31337 BTC for a coffee"
Mining the block containing "Pay 0.31337 BTC for a coffee"
000000aa0748da7367dec6b9de5027f4fae0963df89ff39d8f20fd7299307148

Success!

$ blockchain_go printchain
Prev. hash: 000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13
Data: Pay 0.31337 BTC for a coffee
Hash: 000000aa0748da7367dec6b9de5027f4fae0963df89ff39d8f20fd7299307148
PoW: true

Prev. hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
Data: Send 1 BTC to Ivan
Hash: 000000d7b0c76e1001cdc1fc866b95a481d23f3027d86901eaeb77ae6d002b13
PoW: true

Prev. hash:
Data: Genesis Block
Hash: 000000edc4a82659cebf087adee1ea353bd57fcd59927662cd5ff1c4f618109b
PoW: true
```

*(sound of a beer can opening)*

## Conclusion

Next time we’ll implement addresses, wallets, and (probably) transactions. So stay tuned!

下次我们将实现地址、钱包和（可能）交易。敬请期待！

## Links

1. [Full source codes](https://github.com/Jeiwan/blockchain_go/tree/part_3)
2. [Bitcoin Core Data Storage](https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_2):_Data_Storage)
3. [boltdb](https://github.com/boltdb/bolt)
4. [encoding/gob](https://golang.org/pkg/encoding/gob/)
5. [flag](https://golang.org/pkg/flag/)

