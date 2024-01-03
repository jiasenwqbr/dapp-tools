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





