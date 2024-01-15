# Building Blockchain in Go. Part 6: Transactions 2

## Reward报酬

One tiny thing we skipped in a previous article is rewards for mining. And we already have everything to implement it.

我们在上一篇文章中跳过的一件小事是挖矿奖励。我们已经具备了实施它的一切条件。

The reward is just a coinbase transaction. When a mining node starts mining a new block, it takes transactions from the queue and prepends a coinbase transaction to them. The coinbase transaction’s only output contains miner’s public key hash.

奖励只是一笔 coinbase 交易。当挖掘节点开始挖掘新区块时，它会从队列中获取交易并在其前面添加一个 coinbase 交易。coinbase 交易的唯一输出包含矿工的公钥哈希。

Implementing rewards is as easy as updating the send command:

实施奖励就像更新`send`命令一样简单：



```go
func (cli *CLI) send(from, to string, amount int) {
  ...
  bc := NewBlockchain()
  UTXOSet := UTXOSet{bc}
  defer bc.db.Close()
  tx := NewUTXOTransaction(from, to, amount, &UTXOSet)
  cbTx := NewCoinbaseTX(from, "")
  txs := []*Transaction{cbTx, tx}

  newBlock := bc.MineBlock(txs)
  fmt.Println("Success!")
}
```

In our implementation, the one who creates a transaction mines the new block, and thus, receives a reward.

在我们的实现中，创建交易的人会挖掘新区块，从而获得奖励。

## The UTXO Set
In Part 3: Persistence and CLI we studied the way Bitcoin Core stores blocks in a database. It was said that blocks are stored in blocks database and transaction outputs are stored in chainstate database. Let me remind you what the structure of chainstate is:

在[第 3 部分：持久性和 CLI](https://jeiwan.net/posts/building-blockchain-in-go-part-3/)中，我们研究了 Bitcoin Core 在数据库中存储区块的方式。据说区块存储在`blocks`数据库中，交易输出也存储在`chainstate`数据库中。让我提醒一下您的结构`chainstate`是什么：

1. `'c' + 32-byte transaction hash -> unspent transaction output record for that transaction`
2. `'B' -> 32-byte block hash: the block hash up to which the database represents the unspent transaction outputs`

Since that article, we’ve already implemented transactions, but we haven’t used the chainstate to store their outputs. So, this is what we’re going to do now.

自那篇文章以来，我们已经实现了交易，但我们还没有使用它`chainstate`来存储它们的输出。所以，这就是我们现在要做的。

chainstate doesn’t store transactions. Instead, it stores what is called the UTXO set, or the set of unspent transaction outputs. Besides this, it stores “the block hash up to which the database represents the unspent transaction outputs”, which we’ll omit for now because we’re not using block heights (but we’ll implement them in next articles).

`chainstate`不存储交易。相反，它存储所谓的 UTXO 集，或未花费的交易输出集。除此之外，它还存储“数据库代表未使用交易输出的块哈希”，我们现在将省略它，因为我们不使用块高度（但我们将在下一篇文章中实现它们）。

So, why do we want to have the UTXO set?

那么，为什么我们要设置UTXO呢？

Consider the Blockchain.FindUnspentTransactions method we’ve implemented earlier:

```go
func (bc *Blockchain) FindUnspentTransactions(pubKeyHash []byte) []Transaction {
    ...
    bci := bc.Iterator()

    for {
        block := bci.Next()

        for _, tx := range block.Transactions {
            ...
        }

        if len(block.PrevBlockHash) == 0 {
            break
        }
    }
    ...
}
```

The function finds transactions with unspent outputs. Since transactions are stored in blocks, it iterates over each block in the blockchain and checks every transaction in it. As of September 18, 2017, there’re 485,860 blocks in Bitcoin and the whole database takes 140+ Gb of disk space. This means that one has to run a full node to validate transactions. Moreover, validating transactions would require iterating over many blocks.

该函数查找具有未花费输出的交易。由于交易存储在块中，因此它会迭代区块链中的每个块并检查其中的每个交易。截至 2017 年 9 月 18 日，比特币有 485,860 个区块，整个数据库占用 140+ Gb 的磁盘空间。这意味着必须运行完整节点来验证交易。此外，验证交易需要迭代许多块。

The solution to the problem is to have an index that stores only unspent outputs, and this is what the UTXO set does: this is a cache that is built from all blockchain transactions (by iterating over blocks, yes, but this is done only once), and is later used to calculate balance and validate new transactions. The UTXO set is about 2.7 Gb as of September 2017.

问题的解决方案是拥有一个仅存储未花费输出的索引，这就是 UTXO 集的作用：这是一个由所有区块链交易构建的缓存（通过迭代块，是的，但这只完成一次），稍后用于计算余额并验证新交易。截至 2017 年 9 月，UTXO 集约为 2.7 GB。

Alright, let’s think what we need to change to implement the UTXO set. Currently, the following methods are used to find transactions:

好吧，让我们想想我们需要改变什么来实现 UTXO 集。目前，查找交易的方法有以下几种：

1. `Blockchain.FindUnspentTransactions` – the main function that finds transactions with unspent outputs. It’s this function where the iteration of all blocks happens.
2. `Blockchain.FindSpendableOutputs` – this function is used when a new transaction is created. If finds the enough number of outputs holding required amount. Uses `Blockchain.FindUnspentTransactions`.
3. `Blockchain.FindUTXO` – finds unspent outputs for a public key hash, used to get balance. Uses `Blockchain.FindUnspentTransactions`.
4. `Blockchain.FindTransaction` – finds a transaction in the blockchain by its ID. It iterates over all blocks until finds it.

1. `Blockchain.FindUnspentTransactions`– 查找具有未花费输出的交易的主要功能。所有块的迭代都是在这个函数中发生的。
2. `Blockchain.FindSpendableOutputs`– 创建新交易时使用此函数。如果找到足够数量的输出来容纳所需的金额。用途`Blockchain.FindUnspentTransactions`.
3. `Blockchain.FindUTXO`– 查找公钥哈希的未使用输出，用于获得平衡。用途`Blockchain.FindUnspentTransactions`.
4. `Blockchain.FindTransaction`– 通过 ID 在区块链中查找交易。它迭代所有块直到找到它。

As you can see, all the methods iterate over blocks in the database. But we cannot improve all of them for now, because the UTXO set doesn’t store all transactions, but only those that have unspent outputs. Thus, it cannot be used in `Blockchain.FindTransaction`.

正如您所看到的，所有方法都会迭代数据库中的块。但我们暂时无法改进所有这些，因为 UTXO 集并不存储所有交易，而只存储那些具有未花费输出的交易。因此，它不能用在`Blockchain.FindTransaction`.

So, we want the following methods:

所以，我们需要以下方法：

1. `Blockchain.FindUTXO` – finds all unspent outputs by iterating over blocks.
2. `UTXOSet.Reindex` — uses `FindUTXO` to find unspent outputs, and stores them in a database. This is where caching happens.
3. `UTXOSet.FindSpendableOutputs` – analog of `Blockchain.FindSpendableOutputs`, but uses the UTXO set.
4. `UTXOSet.FindUTXO` – analog of `Blockchain.FindUTXO`, but uses the UTXO set.
5. `Blockchain.FindTransaction` remains the same.

1. `Blockchain.FindUTXO`– 通过迭代块来查找所有未使用的输出。
2. `UTXOSet.Reindex`— 用于`FindUTXO`查找未使用的输出，并将其存储在数据库中。这就是缓存发生的地方。
3. `UTXOSet.FindSpendableOutputs`– 与 类似`Blockchain.FindSpendableOutputs`，但使用 UTXO 集。
4. `UTXOSet.FindUTXO`– 与 类似`Blockchain.FindUTXO`，但使用 UTXO 集。
5. `Blockchain.FindTransaction`保持不变。

Thus, the two most frequently used functions will use the cache from now! Let’s start coding.

因此，从现在开始，两个最常用的功能将使用缓存！让我们开始编码。

```go
type UTXOSet struct {
    Blockchain *Blockchain
}
```

We’ll use a single database, but we’ll store the UTXO set in a different bucket. Thus, `UTXOSet` is coupled with `Blockchain`.

我们将使用单个数据库，但将 UTXO 集存储在不同的存储桶中。因此，`UTXOSet`与 耦合`Blockchain`。

```go
func (u UTXOSet) Reindex() {
    db := u.Blockchain.db
    bucketName := []byte(utxoBucket)

    err := db.Update(func(tx *bolt.Tx) error {
        err := tx.DeleteBucket(bucketName)
        _, err = tx.CreateBucket(bucketName)
    })

    UTXO := u.Blockchain.FindUTXO()

    err = db.Update(func(tx *bolt.Tx) error {
        b := tx.Bucket(bucketName)

        for txID, outs := range UTXO {
            key, err := hex.DecodeString(txID)
            err = b.Put(key, outs.Serialize())
        }
    })
}
```

This method creates the UTXO set initially. First, it removes the bucket if it exists, then it gets all unspent outputs from blockchain, and finally it saves the outputs to the bucket.

该方法最初创建 UTXO 集。首先，它删除存储桶（如果存在），然后从区块链获取所有未使用的输出，最后将输出保存到存储桶中。

`Blockchain.FindUTXO` is almost identical to `Blockchain.FindUnspentTransactions`, but now it returns a map of `TransactionID → TransactionOutputs` pairs.

`Blockchain.FindUTXO`与 几乎相同`Blockchain.FindUnspentTransactions`，但现在它返回一个`TransactionID → TransactionOutputs`成对的映射。

Now, the UTXO set can be used to send coins:

现在，UTXO集可以用来发送硬币：

```go
func (u UTXOSet) FindSpendableOutputs(pubkeyHash []byte, amount int) (int, map[string][]int) {
    unspentOutputs := make(map[string][]int)
    accumulated := 0
    db := u.Blockchain.db

    err := db.View(func(tx *bolt.Tx) error {
        b := tx.Bucket([]byte(utxoBucket))
        c := b.Cursor()

        for k, v := c.First(); k != nil; k, v = c.Next() {
            txID := hex.EncodeToString(k)
            outs := DeserializeOutputs(v)

            for outIdx, out := range outs.Outputs {
                if out.IsLockedWithKey(pubkeyHash) && accumulated < amount {
                    accumulated += out.Value
                    unspentOutputs[txID] = append(unspentOutputs[txID], outIdx)
                }
            }
        }
    })

    return accumulated, unspentOutputs
}
```

Or check balance:

```go
func (u UTXOSet) FindUTXO(pubKeyHash []byte) []TXOutput {
    var UTXOs []TXOutput
    db := u.Blockchain.db

    err := db.View(func(tx *bolt.Tx) error {
        b := tx.Bucket([]byte(utxoBucket))
        c := b.Cursor()

        for k, v := c.First(); k != nil; k, v = c.Next() {
            outs := DeserializeOutputs(v)

            for _, out := range outs.Outputs {
                if out.IsLockedWithKey(pubKeyHash) {
                    UTXOs = append(UTXOs, out)
                }
            }
        }

        return nil
    })

    return UTXOs
}
```

These are slightly modified versions of corresponding `Blockchain` methods. Those `Blockchain` methods are not needed anymore.

这些是相应方法的稍微修改版本`Blockchain`。这些`Blockchain`方法不再需要了。

Having the UTXO set means that our data (transactions) are now split into to storages: actual transactions are stored in the blockchain, and unspent outputs are stored in the UTXO set. Such separation requires solid synchronization mechanism because we want the UTXO set to always be updated and store outputs of most recent transactions. But we don’t want to reindex every time a new block is mined because it’s these frequent blockchain scans that we want to avoid. Thus, we need a mechanism of updating the UTXO set:

拥有 UTXO 集意味着我们的数据（交易）现在被分成存储：实际交易存储在区块链中，未花费的输出存储在 UTXO 集中。这种分离需要可靠的同步机制，因为我们希望 UTXO 集始终被更新并存储最近交易的输出。但我们不想每次开采新区块时都重新索引，因为我们想要避免这些频繁的区块链扫描。因此，我们需要一种更新 UTXO 集的机制：

```go
func (u UTXOSet) Update(block *Block) {
    db := u.Blockchain.db

    err := db.Update(func(tx *bolt.Tx) error {
        b := tx.Bucket([]byte(utxoBucket))

        for _, tx := range block.Transactions {
            if tx.IsCoinbase() == false {
                for _, vin := range tx.Vin {
                    updatedOuts := TXOutputs{}
                    outsBytes := b.Get(vin.Txid)
                    outs := DeserializeOutputs(outsBytes)

                    for outIdx, out := range outs.Outputs {
                        if outIdx != vin.Vout {
                            updatedOuts.Outputs = append(updatedOuts.Outputs, out)
                        }
                    }

                    if len(updatedOuts.Outputs) == 0 {
                        err := b.Delete(vin.Txid)
                    } else {
                        err := b.Put(vin.Txid, updatedOuts.Serialize())
                    }

                }
            }

            newOutputs := TXOutputs{}
            for _, out := range tx.Vout {
                newOutputs.Outputs = append(newOutputs.Outputs, out)
            }

            err := b.Put(tx.ID, newOutputs.Serialize())
        }
    })
}
```

The method looks big, but what it does is quite straightforward. When a new block is mined, the UTXO set should be updated. Updating means removing spent outputs and adding unspent outputs from newly mined transactions. If a transaction which outputs were removed, contains no more outputs, it’s removed as well. Quite simple!

该方法看起来很大，但它的作用非常简单。当新的区块被开采时，UTXO 集应该被更新。更新意味着删除已花费的输出并从新开采的交易中添加未花费的输出。如果输出被删除的交易不再包含输出，它也会被删除。非常简单！

Let’s now use the UTXO set where it’s necessary:

现在让我们在必要时使用 UTXO 集：

```go
func (cli *CLI) createBlockchain(address string) {
    ...
    bc := CreateBlockchain(address)
    defer bc.db.Close()

    UTXOSet := UTXOSet{bc}
    UTXOSet.Reindex()
    ...
}
```

Reindexing happens right after a new blockchain is created. For now, this is the only place where `Reindex` is used, even though it looks excessive here because in the beginning of a blockchain there’s only one block with one transaction, and `Update` could’ve been used instead. But we might need the reindexing mechanism in the future.

重新索引发生在新区块链创建后。目前，这是唯一`Reindex`使用的地方，尽管它在这里看起来有点多余，因为在区块链的开始，只有一个区块有一个交易，并且`Update`可以被使用。但将来我们可能需要重新索引机制。

```go
func (cli *CLI) send(from, to string, amount int) {
    ...
    newBlock := bc.MineBlock(txs)
    UTXOSet.Update(newBlock)
}
```

And the UTXO set is updated after a new block is mined.

并且在新区块被开采后，UTXO 集会更新。

Let’s check that it works

让我们检查一下它是否有效

```shell
$ blockchain_go createblockchain -address 1JnMDSqVoHi4TEFXNw5wJ8skPsPf4LHkQ1
00000086a725e18ed7e9e06f1051651a4fc46a315a9d298e59e57aeacbe0bf73

Done!

$ blockchain_go send -from 1JnMDSqVoHi4TEFXNw5wJ8skPsPf4LHkQ1 -to 12DkLzLQ4B3gnQt62EPRJGZ38n3zF4Hzt5 -amount 6
0000001f75cb3a5033aeecbf6a8d378e15b25d026fb0a665c7721a5bb0faa21b

Success!

$ blockchain_go send -from 1JnMDSqVoHi4TEFXNw5wJ8skPsPf4LHkQ1 -to 12ncZhA5mFTTnTmHq1aTPYBri4jAK8TacL -amount 4
000000cc51e665d53c78af5e65774a72fc7b864140a8224bf4e7709d8e0fa433

Success!

$ blockchain_go getbalance -address 1JnMDSqVoHi4TEFXNw5wJ8skPsPf4LHkQ1
Balance of '1F4MbuqjcuJGymjcuYQMUVYB37AWKkSLif': 20

$ blockchain_go getbalance -address 12DkLzLQ4B3gnQt62EPRJGZ38n3zF4Hzt5
Balance of '1XWu6nitBWe6J6v6MXmd5rhdP7dZsExbx': 6

$ blockchain_go getbalance -address 12ncZhA5mFTTnTmHq1aTPYBri4jAK8TacL
Balance of '13UASQpCR8Nr41PojH8Bz4K6cmTCqweskL': 4
```

Nice! The `1JnMDSqVoHi4TEFXNw5wJ8skPsPf4LHkQ1` address received reward 3 times:

1. Once for mining the genesis blocks.
2. Once for mining the block `0000001f75cb3a5033aeecbf6a8d378e15b25d026fb0a665c7721a5bb0faa21b`.
3. And once for mining the block `000000cc51e665d53c78af5e65774a72fc7b864140a8224bf4e7709d8e0fa433 `.

## Merkle Tree 默克尔树

There’s one more optimization mechanism I’d like to discuss in this post.

As it was said above, the full Bitcoin database (i.e., blockchain) takes more than 140 Gb of disk space. Because of the decentralized nature of Bitcoin, every node in the network must be independent and self-sufficient, i.e. every node must store a full copy of the blockchain. With many people starting using Bitcoin, this rule becomes more difficult to follow: it’s not likely that everyone will run a full node. Also, since nodes are full-fledged participants of the network, they have responsibilities: they must verify transactions and blocks. Also, there’s certain internet traffic required to interact with other nodes and download new blocks.

如上所述，完整的比特币数据库（即区块链）需要超过 140 GB 的磁盘空间。由于比特币的去中心化性质，网络中的每个节点都必须是独立且自给自足的，即每个节点都必须存储区块链的完整副本。随着许多人开始使用比特币，这条规则变得更加难以遵循：不太可能每个人都会运行完整节点。此外，由于节点是网络的正式参与者，因此它们有责任：它们必须验证交易和区块。此外，与其他节点交互和下载新块需要一定的互联网流量。

In [the original Bitcoin paper](https://bitcoin.org/bitcoin.pdf) published by Satoshi Nakamoto, there was a solution for this problem: Simplified Payment Verification (SPV). SPV is a light Bitcoin node that doesn’t download the whole blockchain and **doesn’t verify blocks and transactions**. Instead, it finds transactions in blocks (to verify payments) and is linked to a full node to retrieve just necessary data. This mechanism allows having multiple light wallet nodes with running just one full node.

在中本聪发表的[原始比特币论文](https://bitcoin.org/bitcoin.pdf)中，针对这个问题有一个解决方案：简化支付验证（SPV）。SPV是一个轻量级的比特币节点，它不下载整个区块链，**也不验证区块和交易**。相反，它在区块中查找交易（以验证付款），并链接到完整节点以检索必要的数据。这一机制允许多个轻钱包节点只运行一个完整节点。

For SPV to be possible, there should be a way to check if a block contains certain transaction without downloading the whole block. And this is where Merkle tree comes into play.

为了使 SPV 成为可能，应该有一种方法可以检查一个块是否包含某些交易，而无需下载整个块。这就是默克尔树发挥作用的地方。

Merkle trees are used by Bitcoin to obtain transactions hash, which is then saved in block headers and is considered by the proof-of-work system. Until now, we just concatenated hashes of each transaction in a block and applied `SHA-256` to them. This is also a good way of getting a unique representation of block transactions, but it doesn’t have benefits of Merkle trees.

比特币使用默克尔树来获取交易哈希，然后将其保存在区块头中并由工作量证明系统考虑。到目前为止，我们只是将块中每笔交易的哈希值连接起来并应用于`SHA-256`它们。这也是获得区块交易的唯一表示的好方法，但它没有 Merkle 树的优点。

Let’s look at a Merkle tree:

让我们看一下默克尔树：

![Merkle tree diagram](https://jeiwan.net/images/merkle-tree-diagram.png)

A Merkle tree is built for each block, and it starts with leaves (the bottom of the tree), where a leaf is a transaction hash (Bitcoins uses double `SHA256` hashing). The number of leaves must be even, but not every block contains an even number of transactions. In case there is an odd number of transactions, the last transaction is duplicated (in the Merkle tree, not in the block!).

为每个块构建一棵 Merkle 树，它从叶子（树的底部）开始，其中叶子是交易哈希（比特币使用双重`SHA256`哈希）。叶子的数量必须是偶数，但并非每个块都包含偶数个交易。如果交易数量为奇数，则最后一个交易会被重复（在 Merkle 树中，而不是在区块中！）。

Moving from the bottom up, leaves are grouped in pairs, their hashes are concatenated, and a new hash is obtained from the concatenated hashes. The new hashes form new tree nodes. This process is repeated until there’s just one node, which is called the root of the tree. The root hash is then used as the unique representation of the transactions, is saved in block headers, and is used in the proof-of-work system.

从下往上，叶子成对分组，它们的哈希值被连接起来，并从连接的哈希值中获得一个新的哈希值。新的哈希值形成新的树节点。重复这一过程，直到只剩下一个节点，该节点称为树的根。然后，根哈希用作交易的唯一表示，保存在块头中，并在工作量证明系统中使用。

The benefit of Merkle trees is that a node can verify membership of certain transaction without downloading the whole block. Just a transaction hash, a Merkle tree root hash, and a Merkle path are required for this.

Merkle 树的好处是节点可以验证特定交易的成员资格，而无需下载整个区块。为此只需要一个交易哈希、一个 Merkle 树根哈希和一个 Merkle 路径。

Finally, let’s write code:

```go
type MerkleTree struct {
    RootNode *MerkleNode
}

type MerkleNode struct {
    Left  *MerkleNode
    Right *MerkleNode
    Data  []byte
}
```

We start with structs. Every `MerkleNode` keeps data and links to its branches. `MerkleTree` is actually the root node linked to the next nodes, which are in their turn linked to further nodes, etc.

Let’s create a new node first:

我们从结构开始。每个都`MerkleNode`保存数据并链接到其分支机构。`MerkleTree`实际上是链接到下一个节点的根节点，下一个节点又链接到其他节点，等等。

我们先创建一个新节点：

```go
func NewMerkleNode(left, right *MerkleNode, data []byte) *MerkleNode {
    mNode := MerkleNode{}

    if left == nil && right == nil {
        hash := sha256.Sum256(data)
        mNode.Data = hash[:]
    } else {
        prevHashes := append(left.Data, right.Data...)
        hash := sha256.Sum256(prevHashes)
        mNode.Data = hash[:]
    }

    mNode.Left = left
    mNode.Right = right

    return &mNode
}
```

Every node contains some data. When a node is a leaf, the data is passed from the outside (a serialized transaction in our case). When a node is linked to other nodes, it takes their data and concatenates and hashes it.

每个节点都包含一些数据。当节点是叶子时，数据从外部传递（在我们的例子中是序列化事务）。当一个节点链接到其他节点时，它会获取它们的数据并将其连接和散列。

```go
func NewMerkleTree(data [][]byte) *MerkleTree {
    var nodes []MerkleNode

    if len(data)%2 != 0 {
        data = append(data, data[len(data)-1])
    }

    for _, datum := range data {
        node := NewMerkleNode(nil, nil, datum)
        nodes = append(nodes, *node)
    }

    for i := 0; i < len(data)/2; i++ {
        var newLevel []MerkleNode

        for j := 0; j < len(nodes); j += 2 {
            node := NewMerkleNode(&nodes[j], &nodes[j+1], nil)
            newLevel = append(newLevel, *node)
        }

        nodes = newLevel
    }

    mTree := MerkleTree{&nodes[0]}

    return &mTree
}
```

When a new tree is created, the first thing to ensure is that there is an even number of leaves. After that, `data` (which is an array of serialized transactions) is converted into tree leaves, and a tree is grown from these leaves.

Now, let’s modify `Block.HashTransactions`, which is used in the proof-of-work system to obtain transactions hash:

当创建一棵新树时，首先要确保叶子的数量是偶数。之后，`data`（这是一个序列化事务的数组）被转换为树叶，并从这些树叶中长出一棵树。

现在，让我们修改`Block.HashTransactions`，它在工作量证明系统中用于获取交易哈希：

```go
func (b *Block) HashTransactions() []byte {
    var transactions [][]byte

    for _, tx := range b.Transactions {
        transactions = append(transactions, tx.Serialize())
    }
    mTree := NewMerkleTree(transactions)

    return mTree.RootNode.Data
}
```

First, transactions are serialized (using `encoding/gob`), and then they are used to build a Merkle tree. The root of the tree will serve as the unique identifier of block’s transactions.

首先，交易被序列化（使用`encoding/gob`），然后它们被用来构建 Merkle 树。树的根将作为区块交易的唯一标识符。

## P2PKH

There’s one more thing I’d like to discuss in more detail.

As you remember, in Bitcoin there is the *Script* programming language, which is used to lock transaction outputs; and transaction inputs provide data to unlock outputs. The language is simple, and code in this language is just a sequence of data and operators. Consider this example:

我还想更详细地讨论一件事。

你还记得，比特币中有一种*脚本*编程语言，用于锁定交易输出；交易输入提供数据来解锁输出。该语言很简单，该语言中的代码只是一系列数据和运算符。考虑这个例子：

```shell
5 2 OP_ADD 7 OP_EQUAL
```

`5`, `2`, and `7` are data. `OP_ADD` and `OP_EQUAL` are operators. *Script* code is executed from left to right: every piece of data is put into the stack and the next operator is applied to the top stack elements. *Script*’s stack is just a simple FILO (First Input Last Output) memory storage: the first element in the stack is the last to be taken, with every further element being put on the previous one.

Let’s break the execution of the above script into steps:

`5`、`2`、 、`7`是数据。`OP_ADD`和`OP_EQUAL`是运营商。*脚本*代码从左到右执行：每条数据都放入堆栈中，并将下一个运算符应用于堆栈顶部的元素。*Script*的堆栈只是一个简单的 FILO（先输入后输出）内存存储：堆栈中的第一个元素是最后取出的，后面的每个元素都放在前一个元素上。

让我们将上述脚本的执行分为几个步骤：

1. Stack: empty. Script: `5 2 OP_ADD 7 OP_EQUAL`.
2. Stack: `5`. Script: `2 OP_ADD 7 OP_EQUAL`.
3. Stack: `5 2`. Script: `OP_ADD 7 OP_EQUAL`.
4. Stack: `7`. Script: `7 OP_EQUAL`.
5. Stack: `7 7`. Script: `OP_EQUAL`.
6. Stack: `true`. Script: empty.

`OP_ADD` takes two elements from the stack, summarizes them, and push the sum into the stack. `OP_EQUAL` takes two elements from the stack and compares them: if they’re equal it pushes `true` to the stack; otherwise it pushes `false`. A result of a script execution is the value of the top stack element: in our case, it’s `true`, which means that the script finished successfully.

`OP_ADD`从堆栈中取出两个元素，对它们求和，然后将和压入堆栈。`OP_EQUAL`从堆栈中取出两个元素并比较它们：如果相等，则将其推`true`入堆栈；如果相等，则将其推入堆栈。否则它会推动`false`。脚本执行的结果是顶部堆栈元素的值：在我们的例子中，它是`true`，这意味着脚本成功完成。

Now let’s look at the script that is used in Bitcoin to perform payments:

现在让我们看一下比特币中用于执行支付的脚本：

```shell
<signature> <pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
```

This script is called *Pay to Public Key Hash* (P2PKH), and this is the most commonly used script in Bitcoin. It literally pays to a public key hash, i.e. locks coins with a certain public key. This is **the heart of Bitcoin payments**: there are no accounts, no funds transferring between them; there’s just a script that checks that provided signature and public key are correct.

The script is actually stored in two parts:

该脚本称为*支付公钥哈希*（P2PKH），这是比特币中最常用的脚本。它实际上支付给公钥哈希，即用某个公钥锁定硬币。这是**比特币支付的核心**：没有账户，账户之间没有资金转移；只有一个脚本可以检查提供的签名和公钥是否正确。

该脚本实际上存储为两部分：

1. The first piece, `<signature> <pubKey>`, is stored in input’s `ScriptSig` field.
2. The second piece, `OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG` is stored in output’s `ScriptPubKey`.

Thus, it’s outputs that define unlocking logic, and it’s inputs that provide data to unlock outputs. Let’s execute the script:

1. Stack: empty
   Script: `<signature> <pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
2. Stack: `<signature>`
   Script: `<pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
3. Stack: `<signature> <pubKey>`
   Script: `OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
4. Stack: `<signature> <pubKey> <pubKey>`
   Script: `OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
5. Stack: `<signature> <pubKey> <pubKeyHash>`
   Script: `<pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
6. Stack: `<signature> <pubKey> <pubKeyHash> <pubKeyHash>`
   Script: `OP_EQUALVERIFY OP_CHECKSIG`
7. Stack: `<signature> <pubKey>`
   Script: `OP_CHECKSIG`
8. Stack: `true` or `false`. Script: empty.

`OP_DUP` duplicates the top stack element. `OP_HASH160` takes the top stack element and hashes it with `RIPEMD160`; the result is pushed back to the stack. `OP_EQUALVERIFY` compares two top stack elements, and if they’re not equal, interrupts the script. `OP_CHECKSIG` validates the signature of a transaction by hashing the transaction and using `<signature>` and `<pubKey>`. The latter operator is quite complex: it makes a trimmed copy of the transaction, hashes it (because it’s a hash of a transaction that’s signed), and checks that the signature is correct using provided `<signature>` and `<pubKey>`.

Having such scripting language allows Bitcoin to be also a smart-contract platform: the language makes possible other payment schemes besides transferring to a single key. For example,

1. 第一部分`<signature> <pubKey>`存储在输入`ScriptSig`字段中。
2. 第二部分`OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`存储在输出的`ScriptPubKey`.

因此，输出定义解锁逻辑，输入提供数据以解锁输出。让我们执行脚本：

1. 堆栈：空
   脚本：`<signature> <pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
2. 堆栈：`<signature>`
   脚本：`<pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
3. 堆栈：`<signature> <pubKey>`
   脚本：`OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
4. 堆栈：`<signature> <pubKey> <pubKey>`
   脚本：`OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
5. 堆栈：`<signature> <pubKey> <pubKeyHash>`
   脚本：`<pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG`
6. 堆栈：`<signature> <pubKey> <pubKeyHash> <pubKeyHash>`
   脚本：`OP_EQUALVERIFY OP_CHECKSIG`
7. 堆栈：`<signature> <pubKey>`
   脚本：`OP_CHECKSIG`
8. 堆栈：`true`或`false`. 脚本：空。

`OP_DUP`复制顶部堆栈元素。`OP_HASH160`获取栈顶元素并用`RIPEMD160`;对其进行哈希处理 结果被推回堆栈。`OP_EQUALVERIFY`比较两个顶部堆栈元素，如果它们不相等，则中断脚本。`OP_CHECKSIG`通过对交易进行哈希处理并使用`<signature>`和来验证交易的签名`<pubKey>`。后一个运算符非常复杂：它生成交易的修剪副本，对其进行哈希处理（因为它是已签名交易的哈希值），并使用提供的`<signature>`和检查签名是否正确`<pubKey>`。

拥有这样的脚本语言使比特币也成为一个智能合约平台：除了转移到单个密钥之外，该语言还使其他支付方案成为可能。例如，

## Conclusion

And that’s it! We’ve implemented almost all key feature of a blockchain-based cryptocurrency. We have blockchain, addresses, mining, and transactions. But there’s one more thing that gives life to all these mechanisms and makes Bitcoin a global system: consensus. In the next article, we’ll start implementing the “decentralized” part of the blockchain. Stay tuned!

就是这样！我们已经实现了基于区块链的加密货币的几乎所有关键功能。我们有区块链、地址、挖矿和交易。但还有一件事赋予了所有这些机制生命力，并使比特币成为一个全球系统：共识。在下一篇文章中，我们将开始实现区块链的“去中心化”部分。

Links:

1. [Full source codes](https://github.com/Jeiwan/blockchain_go/tree/part_6)
2. [The UTXO Set](https://en.bitcoin.it/wiki/Bitcoin_Core_0.11_(ch_2):_Data_Storage#The_UTXO_set_.28chainstate_leveldb.29)
3. [Merkle Tree](https://en.bitcoin.it/wiki/Protocol_documentation#Merkle_Trees)
4. [Script](https://en.bitcoin.it/wiki/Script)
5. [“Ultraprune” Bitcoin Core commit](https://github.com/sipa/bitcoin/commit/450cbb0944cd20a06ce806e6679a1f4c83c50db2)
6. [UTXO set statistics](https://statoshi.info/dashboard/db/unspent-transaction-output-set)
7. [Smart contracts and Bitcoin](https://medium.com/@maraoz/smart-contracts-and-bitcoin-a5d61011d9b1)
8. [Why every Bitcoin user should understand “SPV security”](https://medium.com/@jonaldfyookball/why-every-bitcoin-user-should-understand-spv-security-520d1d45e0b9)