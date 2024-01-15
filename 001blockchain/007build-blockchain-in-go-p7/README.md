# Building Blockchain in Go. Part 7: Network

## Introduction 介绍

So far, we’ve build a blockchain that has all key features: anonymous, secure, and randomly generated addresses; blockchain data storage; Proof-of-Work system; reliable way to store transactions. While these features are crucial, it’s not enough. What makes these features really shine, and what make cryptocurrencies possible, is network. What’s the use of having such blockchain implementation running just on a single computer? What’s the use of those cryptography based features, when there’s just one user? It’s network that make all these mechanism work and be useful.

到目前为止，我们已经构建了一个具有所有关键特征的区块链：匿名、安全和随机生成的地址；区块链数据存储；工作量证明系统；存储交易的可靠方式。虽然这些功能至关重要，但这还不够。使这些功能真正发挥作用并使加密货币成为可能的是网络。让这样的区块链实现仅在一台计算机上运行有什么用？当只有一个用户时，这些基于密码学的功能有什么用？正是网络使所有这些机制发挥作用并发挥作用。

You can think of those blockchain features as rules, similar to the rules that people establish when they want to live and thrive together. A kind of social arrangements. Blockchain network is a community of programs that follow the same rules, and it’s this following the rules that makes the network alive. Similarly, when people share identical ideas, they become stronger and can together build a better life. If there are people that follow a different set of rules, they’ll live in a separate society (state, commune, etc.). Identically, if there’re blockchain nodes that follow different rules, they’ll form a separate network.

您可以将这些区块链功能视为规则，类似于人们想要共同生活和繁荣时建立的规则。一种社会安排。区块链网络是遵循相同规则的程序社区，正是这种遵循规则才使得网络具有活力。同样，当人们拥有相同的想法时，他们就会变得更强大，可以共同建设更美好的生活。如果有人遵循不同的规则，他们就会生活在一个独立的社会（国家、公社等）中。同样，如果存在遵循不同规则的区块链节点，它们将形成一个单独的网络。

**This is very important:** without a network and without a majority of nodes sharing identical rules, these rules are useless!

**这非常重要：**如果没有网络并且没有大多数节点共享相同的规则，这些规则就没用！

> DISCLAIMER: Unfortunately, I didn’t have enough time to implement a real P2P network prototype. In this article I’ll demonstrate a most common scenario, that involves nodes of different types. Improving this scenario and making this a P2P network can be a good challenge and practice for you! Also I cannot guarantee that other scenarios besides the one implemented in this article, will work. Sorry!

> This part introduces significant code changes, so it makes no sense explaining all of them here. Please refer to [this page](https://github.com/Jeiwan/blockchain_go/compare/part_6...part_7#files_bucket) to see all the changes since the last article.
>
> > 免责声明：不幸的是，我没有足够的时间来实现真正的 P2P 网络原型。在本文中，我将演示一个最常见的场景，其中涉及不同类型的节点。改进这个场景，使其成为一个P2P网络，对你来说是一个很好的挑战和练习！另外，我不能保证除了本文中实现的方案之外的其他方案也能起作用。对不起！
>
> > 这部分引入了重大的代码更改，因此在这里解释所有这些更改是没有意义的。请参阅[此页面](https://github.com/Jeiwan/blockchain_go/compare/part_6...part_7#files_bucket)以查看自上一篇文章以来的所有更改。

## Blockchain Network 区块链网络

Blockchain network is decentralized, which means there’re no servers that do stuff and clients that use servers to get or process data. In blockchain network there are nodes, and each node is a full-fledged member of the network. A node is everything: it’s both a client and a server. This is very important to keep in mind, because it’s very different from usual web applications.

区块链网络是去中心化的，这意味着没有服务器来做事，也没有客户端使用服务器来获取或处理数据。区块链网络中存在节点，每个节点都是网络的正式成员。节点就是一切：它既是客户端又是服务器。记住这一点非常重要，因为它与通常的 Web 应用程序非常不同

Blockchain network is a P2P (Peer-to-Peer) network, which means that nodes are connected directly to each other. It’s topology is flat, since there are no hierarchy in node roles. Here its schematic representation:

区块链网络是一个P2P（Peer-to-Peer）网络，这意味着节点之间直接连接。它的拓扑是扁平的，因为节点角色没有层次结构。这是其示意图：

![P2P Network Scheme](https://jeiwan.net/images/p2p-network.png) ([Business vector created by Dooder - Freepik.com](https://www.freepik.com/dooder))

Nodes in such network are more difficult to implement, because they have to perform a lot of operations. Each node must interact with multiple other nodes, it must request other node’s state, compare it with it’s own state, and update its state when it’s outdated.

这种网络中的节点更难以实现，因为它们必须执行大量操作。每个节点必须与多个其他节点交互，它必须请求其他节点的状态，将其与自己的状态进行比较，并在过时时更新其状态。

## Node Roles 节点角色

Despite being full-fledged, blockchain nodes can play different roles in the network. Here they are:

尽管区块链节点已经成熟，但它可以在网络中扮演不同的角色。他们来了：

1. Miner.矿工。
   Such nodes are run on powerful or specialized hardware (like ASIC), and their only goal is to mine new blocks as fast as possible. Miners are only possible in blockchains that use Proof-of-Work, because mining actually means solving PoW puzzles. In Proof-of-Stake blockchains, for example, there’s no mining.

   此类节点在强大或专用的硬件（如 ASIC）上运行，它们唯一的目标是尽快挖掘新块。矿工只能在使用工作量证明的区块链中使用，因为挖矿实际上意味着解决 PoW 难题。例如，在权益证明区块链中，没有挖矿。

2. Full node.全节点
   These nodes validate blocks mined by miners and verify transactions. To do this, they must have the whole copy of blockchain. Also, such nodes perform such routing operations, like helping other nodes to discover each other.
   It’s very crucial for network to have many full nodes, because it’s these nodes that make decisions: they decide if a block or transaction is valid.

   这些节点验证矿工开采的区块并验证交易。为此，他们必须拥有区块链的完整副本。此外，此类节点执行此类路由操作，例如帮助其他节点发现彼此。
   对于网络来说，拥有许多全节点非常重要，因为正是这些节点做出决策：它们决定区块或交易是否有效。

3. SPV.
   SPV stands for Simplified Payment Verification. These nodes don’t store a full copy of blockchain, but they still able to verify transactions (not all of them, but a subset, for example, those that were sent to specific address). An SPV node depends on a full node to get data from, and there could be many SPV nodes connected to one full node. SPV makes wallet applications possible: one don’t need to download full blockchain, but still can verify their transactions.

   SPV 代表简化付款验证。这些节点不存储区块链的完整副本，但它们仍然能够验证交易（不是全部，而是一部分，例如发送到特定地址的交易）。一个SPV节点依赖于一个全节点来获取数据，并且一个全节点可能连接有多个SPV节点。SPV使钱包应用成为可能：人们不需要下载完整的区块链，但仍然可以验证他们的交易。

## Network simplification 网络简化

To implement network in our blockchain, we have to simplify some things. The problem is that we don’t have many computers to simulate a network with multiple nodes. We could’ve used virtual machines or Docker to solve this problem, but it could make everything more difficult: you would have to solve possible virtual machine or Docker issues, while my goal is to concentrate on blockchain implementation only. So, we want to run multiple blockchain nodes on a single machine and at the same time we want them to have different addresses. To achieve this we’ll use **ports as node identifiers**, instead of IP addresses. E.g., there will be nodes with addresses: `127.0.0.1:3000`, `127.0.0.1:3001`, `127.0.0.1:3002`, etc. We’ll call the port node ID and use `NODE_ID` environment variable to set them. Thus, you can open multiple terminal windows, set different `NODE_ID`s and have different nodes running.

为了在我们的区块链中实现网络，我们必须简化一些事情。问题是我们没有很多计算机来模拟具有多个节点的网络。我们本来可以使用虚拟机或 Docker 来解决这个问题，但这可能会让一切变得更加困难：你必须解决可能的虚拟机或 Docker 问题，而我的目标是只专注于区块链实施。因此，我们希望在一台机器上运行多个区块链节点，同时我们希望它们具有不同的地址。为了实现这一点，我们将使用**端口作为节点标识符**，而不是 IP 地址。例如，将会有地址为：`127.0.0.1:3000`、`127.0.0.1:3001`、`127.0.0.1:3002`等的节点。我们将调用端口节点 ID 并使用`NODE_ID`环境变量来设置它们。因此，您可以打开多个终端窗口，设置不同的`NODE_ID`s 并运行不同的节点。

This approach also requires having different blockchains and wallet files. They now must depend on the node ID and be named like `blockchain_3000.db`, `blockchain_30001.db` and `wallet_3000.db`, `wallet_30001.db`, etc.

这种方法还需要拥有不同的区块链和钱包文件。它们现在必须依赖于节点 ID 并命名为`blockchain_3000.db`、`blockchain_30001.db`、`wallet_3000.db`、`wallet_30001.db`等。

## Implementation 实现

So, what happens when you download, say, Bitcoin Core and run it for the first time? It has to connect to some node to downloaded the latest state of the blockchain. Considering that your computer is not aware of all, or some, Bitcoin nodes, what’s this node?

那么，当您下载（例如）Bitcoin Core 并首次运行它时会发生什么？它必须连接到某个节点才能下载区块链的最新状态。考虑到您的计算机不知道全部或部分比特币节点，那么这个节点是什么？

Hardcoding a node address in Bitcoin Core would’ve been a mistake: the node could be attacked or shut down, which could result in new nodes not being able to join the network. Instead, in Bitcoin Core, there are [DNS seeds](https://bitcoin.org/en/glossary/dns-seed) hardcoded. These are not nodes, but DNS servers that know addresses of some nodes. When you start a clean Bitcoin Core, it’ll connect to one of the seeds and get a list of full nodes, which it’ll then download the blockchain from.

在比特币核心中硬编码节点地址将是一个错误：该节点可能会受到攻击或关闭，这可能会导致新节点无法加入网络。相反，在 Bitcoin Core 中，有硬编码的[DNS 种子](https://bitcoin.org/en/glossary/dns-seed)。这些不是节点，而是知道某些节点地址的 DNS 服务器。当您启动一个干净的比特币核心时，它将连接到其中一个种子并获取完整节点的列表，然后从中下载区块链。

In our implementation, there will be centralization though. We’ll have three nodes:

但在我们的实施中，将会出现集中化。我们将有三个节点：

1. The central node. This is the node all other nodes will connect to, and this is the node that’ll sends data between other nodes.中心节点。这是所有其他节点将连接到的节点，也是将在其他节点之间发送数据的节点。

2. A miner node. This node will store new transactions in mempool and when there’re enough of transactions, it’ll mine a new block.一个矿工节点。该节点将在内存池中存储新交易，当有足够的交易时，它将挖掘一个新块。

3. A wallet node. This node will be used to send coins between wallets. Unlike SPV nodes though, it’ll store a full copy of blockchain.钱包节点。该节点将用于在钱包之间发送硬币。但与 SPV 节点不同的是，它将存储区块链的完整副本。

   

### The Scenario 场景

The goal of this article is to implement the following scenario:

本文的目标是实现以下场景：

1. The central node creates a blockchain.中心节点创建区块链。
2. Other (wallet) node connects to it and downloads the blockchain.其他（钱包）节点连接到它并下载区块链。
3. One more (miner) node connects to the central node and downloads the blockchain.另一个（矿工）节点连接到中央节点并下载区块链。
4. The wallet node creates a transaction.钱包节点创建交易。
5. The miner nodes receives the transaction and keeps it in its memory pool.矿工节点接收交易并将其保存在内存池中。
6. When there are enough transactions in the memory pool, the miner starts mining a new block.当内存池中有足够的交易时，矿工开始挖掘新的区块。
7. When a new block is mined, it’s send to the central node.当新区块被开采时，它会被发送到中央节点。
8. The wallet node synchronizes with the central node.钱包节点与中心节点同步。
9. User of the wallet node checks that their payment was successful.钱包节点的用户检查他们的支付是否成功。

This is what it looks like in Bitcoin. Even though we’re not going to build a real P2P network, we’re going to implement a real, and the main and most important, use case of Bitcoin.

这就是比特币的样子。尽管我们不打算构建一个真正的 P2P 网络，但我们将实现一个真正的、主要也是最重要的比特币用例。

### version 版本

Nodes communicate by the means of messages. When a new node is run, it gets several nodes from a DNS seed, and sends them `version` message, which in our implementation will look like this:

节点通过消息的方式进行通信。当一个新节点运行时，它从 DNS 种子获取多个节点，并向它们发送`version`消息，在我们的实现中将如下所示：

```go
type version struct {
    Version    int
    BestHeight int
    AddrFrom   string
}
```

We have only one blockchain version, so the `Version` field won’t keep any important information. `BestHeight` stores the length of the node’s blockchain. `AddFrom` stores the address of the sender.我们只有一个区块链版本，因此该`Version`字段不会保留任何重要信息。`BestHeight`存储节点区块链的长度。`AddFrom`存储发件人的地址。

What should a node that receives a `version` message do? It’ll respond with its own `version` message. This is a kind of a handshake: no other interaction is possible without prior greeting of each other. But it’s not just politeness: `version` is used to find a longer blockchain. When a node receives a `version` message it checks if the node’s blockchain is longer than the value of `BestHeight`. If it’s not, the node will request and download missing blocks.收到消息的节点应该做什么`version`？它会用自己的`version`消息进行响应。这是一种握手：如果没有事先互相问候，就不可能有其他互动。但这不仅仅是礼貌：`version`用于寻找更长的区块链。当节点收到`version`消息时，它会检查该节点的区块链是否长于 的值`BestHeight`。如果不是，节点将请求并下载丢失的块。

In order to receive message, we need a server:

为了接收消息，我们需要一个服务器：

```go
var nodeAddress string
var knownNodes = []string{"localhost:3000"}

func StartServer(nodeID, minerAddress string) {
    nodeAddress = fmt.Sprintf("localhost:%s", nodeID)
    miningAddress = minerAddress
    ln, err := net.Listen(protocol, nodeAddress)
    defer ln.Close()

    bc := NewBlockchain(nodeID)

    if nodeAddress != knownNodes[0] {
        sendVersion(knownNodes[0], bc)
    }

    for {
        conn, err := ln.Accept()
        go handleConnection(conn, bc)
    }
}
```

First, we hardcode the address of the central node: every node must know where to connect to initially. `minerAddress` argument specifies the address to receive mining rewards to. This piece:

首先，我们对中心节点的地址进行硬编码：每个节点都必须知道最初连接到哪里。`minerAddress`参数指定接收挖矿奖励的地址。这件作品：

```go
if nodeAddress != knownNodes[0] {
    sendVersion(knownNodes[0], bc)
}
```

Means that if current node is not the central one, it must send `version` message to the central node to find out if its blockchain is outdated.

意味着如果当前节点不是中心节点，则必须`version`向中心节点发送消息以了解其区块链是否已过时。

```go
func sendVersion(addr string, bc *Blockchain) {
    bestHeight := bc.GetBestHeight()
    payload := gobEncode(version{nodeVersion, bestHeight, nodeAddress})

    request := append(commandToBytes("version"), payload...)

    sendData(addr, request)
}
```

Our messages, on the lower level, are sequences of bytes. First 12 bytes specify command name (“version” in this case), and the latter bytes will contain `gob`-encoded message structure. `commandToBytes` looks like this:

我们的消息在较低级别上是字节序列。前 12 个字节指定命令名称（在本例中为“版本”），后面的字节将包含`gob`编码的消息结构。`commandToBytes`看起来像这样：

```go
func commandToBytes(command string) []byte {
    var bytes [commandLength]byte

    for i, c := range command {
        bytes[i] = byte(c)
    }

    return bytes[:]
}
```

It creates a 12-byte buffer and fills it with the command name, leaving rest bytes empty. There’s an opposite function:

它创建一个 12 字节缓冲区并用命令名称填充它，将其余字节留空。有一个相反的函数：

```go
func bytesToCommand(bytes []byte) string {
    var command []byte

    for _, b := range bytes {
        if b != 0x0 {
            command = append(command, b)
        }
    }

    return fmt.Sprintf("%s", command)
}
```

When a node receives a command, it runs `bytesToCommand` to extract command name and processes command body with correct handler:

当节点收到命令时，它会运行`bytesToCommand`以提取命令名称并使用正确的处理程序处理命令正文：

```go
func handleConnection(conn net.Conn, bc *Blockchain) {
    request, err := ioutil.ReadAll(conn)
    command := bytesToCommand(request[:commandLength])
    fmt.Printf("Received %s command\n", command)

    switch command {
    ...
    case "version":
        handleVersion(request, bc)
    default:
        fmt.Println("Unknown command!")
    }

    conn.Close()
}
```

Ok, this is what the `version` command handler looks like:

好的，`version`命令处理程序如下所示：

```go
func handleVersion(request []byte, bc *Blockchain) {
    var buff bytes.Buffer
    var payload verzion

    buff.Write(request[commandLength:])
    dec := gob.NewDecoder(&buff)
    err := dec.Decode(&payload)

    myBestHeight := bc.GetBestHeight()
    foreignerBestHeight := payload.BestHeight

    if myBestHeight < foreignerBestHeight {
        sendGetBlocks(payload.AddrFrom)
    } else if myBestHeight > foreignerBestHeight {
        sendVersion(payload.AddrFrom, bc)
    }

    if !nodeIsKnown(payload.AddrFrom) {
        knownNodes = append(knownNodes, payload.AddrFrom)
    }
}
```

First, we need to decode the request and extract the payload. This is similar to all the handlers, so I’ll omit this piece in the future code snippets.

Then a node compares its `BestHeight` with the one from the message. If the node’s blockchain is longer, it’ll reply with `version` message; otherwise, it’ll send `getblocks` message.

首先，我们需要解码请求并提取有效负载。这与所有处理程序类似，因此我将在以后的代码片段中省略这一部分。

然后节点将其`BestHeight`与消息中的进行比较。如果节点的区块链较长，则会回复`version`消息；否则，它会发送`getblocks`消息。

### getblocks

```go
type getblocks struct {
    AddrFrom string
}
```

`getblocks` means “show me what blocks you have” (in Bitcoin, it’s more complex). Pay attention, it doesn’t say “give me all your blocks”, instead it requests a list of block hashes. This is done to reduce network load, because blocks can be downloaded from different nodes, and we don’t want to download dozens of gigabytes from one node.

Handling the command as easy as:

`getblocks`意思是“告诉我你有什么区块”（在比特币中，它更复杂）。请注意，它并没有说“给我所有的块”，而是请求一个块哈希列表。这样做是为了减少网络负载，因为块可以从不同的节点下载，我们不想从一个节点下载几十GB。

处理命令就像这样简单：



```go
func handleGetBlocks(request []byte, bc *Blockchain) {
    ...
    blocks := bc.GetBlockHashes()
    sendInv(payload.AddrFrom, "block", blocks)
}
```

In our simplified implementation, it’ll return **all block hashes**.

### inv

```go
type inv struct {
    AddrFrom string
    Type     string
    Items    [][]byte
}
```

Bitcoin uses `inv` to show other nodes what blocks or transactions current node has. Again, it doesn’t contain whole blocks and transactions, just their hashes. The `Type` field says whether these are blocks or transactions.

Handling `inv` is more difficult:

比特币用于`inv`向其他节点显示当前节点有哪些区块或交易。同样，它不包含整个块和交易，只包含它们的哈希值。该`Type`字段表示这些是块还是交易。

处理起来`inv`比较困难：

```go
func handleInv(request []byte, bc *Blockchain) {
    ...
    fmt.Printf("Recevied inventory with %d %s\n", len(payload.Items), payload.Type)

    if payload.Type == "block" {
        blocksInTransit = payload.Items

        blockHash := payload.Items[0]
        sendGetData(payload.AddrFrom, "block", blockHash)

        newInTransit := [][]byte{}
        for _, b := range blocksInTransit {
            if bytes.Compare(b, blockHash) != 0 {
                newInTransit = append(newInTransit, b)
            }
        }
        blocksInTransit = newInTransit
    }

    if payload.Type == "tx" {
        txID := payload.Items[0]

        if mempool[hex.EncodeToString(txID)].ID == nil {
            sendGetData(payload.AddrFrom, "tx", txID)
        }
    }
}
```

If blocks hashes are transferred, we want to save them in `blocksInTransit` variable to track downloaded blocks. This allows us to download blocks from different nodes. Right after putting blocks into the transit state, we send `getdata` command to the sender of the `inv` message and update `blocksInTransit`. In a real P2P network, we would want to transfer blocks from different nodes.

In our implementation, we’ll never send `inv` with multiple hashes. That’s why when `payload.Type == "tx"` only the first hash is taken. Then we check if we already have the hash in our mempool, and if not, `getdata` message is sent.

如果传输块哈希，我们希望将它们保存在`blocksInTransit`变量中以跟踪下载的块。这允许我们从不同的节点下载块。将块置于传输状态后，我们立即`getdata`向消息发送者发送命令`inv`并更新`blocksInTransit`。在真实的 P2P 网络中，我们希望从不同的节点传输区块。

在我们的实现中，我们永远不会发送`inv`多个哈希值。这就是为什么`payload.Type == "tx"`只采用第一个哈希值的原因。然后我们检查内存池中是否已经有哈希值，如果没有，则`getdata`发送消息。

### getdata

```go
type getdata struct {
    AddrFrom string
    Type     string
    ID       []byte
}
```

`getdata` is a request for certain block or transaction, and it can contain only one block/transaction ID.

```go
func handleGetData(request []byte, bc *Blockchain) {
    ...
    if payload.Type == "block" {
        block, err := bc.GetBlock([]byte(payload.ID))

        sendBlock(payload.AddrFrom, &block)
    }

    if payload.Type == "tx" {
        txID := hex.EncodeToString(payload.ID)
        tx := mempool[txID]

        sendTx(payload.AddrFrom, &tx)
    }
}
```

The handler is straightforward: if they request a block, return the block; if they request a transaction, return the transaction. Notice, that we don’t check if we actually have this block or transaction. This is a flaw :)

处理程序很简单：如果他们请求一个块，则返回该块；如果他们请求交易，则返回交易。请注意，我们不会检查我们是否确实拥有此块或交易。这是一个缺陷:)

### block and tx

```go
type block struct {
    AddrFrom string
    Block    []byte
}

type tx struct {
    AddFrom     string
    Transaction []byte
}
```

It’s these messages that actually transfer the data.真正传输数据的是这些消息。

Handling the `block` message is easy:

```go
func handleBlock(request []byte, bc *Blockchain) {
    ...

    blockData := payload.Block
    block := DeserializeBlock(blockData)

    fmt.Println("Recevied a new block!")
    bc.AddBlock(block)

    fmt.Printf("Added block %x\n", block.Hash)

    if len(blocksInTransit) > 0 {
        blockHash := blocksInTransit[0]
        sendGetData(payload.AddrFrom, "block", blockHash)

        blocksInTransit = blocksInTransit[1:]
    } else {
        UTXOSet := UTXOSet{bc}
        UTXOSet.Reindex()
    }
}
```

When we received a new block, we put it into our blockchain. If there’re more blocks to download, we request them from the same node we downloaded the previous block. When we finally downloaded all the blocks, the UTXO set is reindexed.

当我们收到一个新块时，我们将其放入区块链中。如果有更多块需要下载，我们会从下载前一个块的同一节点请求它们。当我们最终下载所有块时，UTXO 集被重新索引。

> TODO: Instead of trusting unconditionally, we should validate every incoming block before adding it to the blockchain.
>
> *TODO：我们不应该无条件信任，而应该在将每个传入块添加到区块链之前对其进行验证。*

> TODO: Instead of running UTXOSet.Reindex(), UTXOSet.Update(block) should be used, because if blockchain is big, it’ll take a lot of time to reindex the whole UTXO set.
>
> *TODO：不应运行 UTXOSet.Reindex()，而应使用 UTXOSet.Update(block)，因为如果区块链很大，则需要花费大量时间来重新索引整个 UTXO 集。*

Handling `tx` messages is the most difficult part:

```go
func handleTx(request []byte, bc *Blockchain) {
    ...
    txData := payload.Transaction
    tx := DeserializeTransaction(txData)
    mempool[hex.EncodeToString(tx.ID)] = tx

    if nodeAddress == knownNodes[0] {
        for _, node := range knownNodes {
            if node != nodeAddress && node != payload.AddFrom {
                sendInv(node, "tx", [][]byte{tx.ID})
            }
        }
    } else {
        if len(mempool) >= 2 && len(miningAddress) > 0 {
        MineTransactions:
            var txs []*Transaction

            for id := range mempool {
                tx := mempool[id]
                if bc.VerifyTransaction(&tx) {
                    txs = append(txs, &tx)
                }
            }

            if len(txs) == 0 {
                fmt.Println("All transactions are invalid! Waiting for new ones...")
                return
            }

            cbTx := NewCoinbaseTX(miningAddress, "")
            txs = append(txs, cbTx)

            newBlock := bc.MineBlock(txs)
            UTXOSet := UTXOSet{bc}
            UTXOSet.Reindex()

            fmt.Println("New block is mined!")

            for _, tx := range txs {
                txID := hex.EncodeToString(tx.ID)
                delete(mempool, txID)
            }

            for _, node := range knownNodes {
                if node != nodeAddress {
                    sendInv(node, "block", [][]byte{newBlock.Hash})
                }
            }

            if len(mempool) > 0 {
                goto MineTransactions
            }
        }
    }
}
```

First thing to do is to put new transaction in the mempool (again, transactions must be verified before being placed into the mempool). Next piece:

首先要做的是将新交易放入内存池（同样，交易在放入内存池之前必须经过验证）。下一篇：

```go
if nodeAddress == knownNodes[0] {
    for _, node := range knownNodes {
        if node != nodeAddress && node != payload.AddFrom {
            sendInv(node, "tx", [][]byte{tx.ID})
        }
    }
}
```

Checks whether the current node is the central one. In our implementation, the central node won’t mine blocks. Instead, it’ll forward the new transactions to other nodes in the network.

The next big piece is only for miner nodes. Let’s split it into smaller pieces:

检查当前节点是否为中心节点。在我们的实现中，中心节点不会挖掘区块。相反，它将新交易转发到网络中的其他节点。

下一个大块仅适用于矿工节点。让我们把它分成更小的部分：

```go
if len(mempool) >= 2 && len(miningAddress) > 0 {
```

`miningAddress` is only set on miner nodes. When there are 2 or more transactions in the mempool of the current (miner) node, mining begins.

`miningAddress`仅在矿工节点上设置。当当前（矿工）节点的内存池中有 2 个或更多交易时，挖矿开始。

```go
for id := range mempool {
    tx := mempool[id]
    if bc.VerifyTransaction(&tx) {
        txs = append(txs, &tx)
    }
}

if len(txs) == 0 {
    fmt.Println("All transactions are invalid! Waiting for new ones...")
    return
}
```

First, all transactions in the mempool are verified. Invalid transactions are ignored, and if there are no valid transactions, mining is interrupted.

首先，内存池中的所有交易都经过验证。无效交易将被忽略，如果没有有效交易，挖矿就会中断。

```go
cbTx := NewCoinbaseTX(miningAddress, "")
txs = append(txs, cbTx)

newBlock := bc.MineBlock(txs)
UTXOSet := UTXOSet{bc}
UTXOSet.Reindex()

fmt.Println("New block is mined!")
```

Verified transactions are being put into a block, as well as a coinbase transaction with the reward. After mining the block, the UTXO set is reindexed.

经过验证的交易以及带有奖励的币库交易将被放入一个区块中。挖掘区块后，UTXO 集将被重新索引。

> TODO: Again, UTXOSet.Update should be used instead of UTXOSet.Reindex

```go
for _, tx := range txs {
    txID := hex.EncodeToString(tx.ID)
    delete(mempool, txID)
}

for _, node := range knownNodes {
    if node != nodeAddress {
        sendInv(node, "block", [][]byte{newBlock.Hash})
    }
}

if len(mempool) > 0 {
    goto MineTransactions
}
```

After a transaction is mined, it’s removed from the mempool. Every other nodes the current node is aware of, receive `inv` message with the new block’s hash. They can request the block after handling the message.

## Result

Let’s play the scenario we defined earlier.

First, set `NODE_ID` to 3000 (`export NODE_ID=3000`) in the first terminal window. I’ll use badges like `NODE 3000` or `NODE 3001` before next paragraphs, for you to know what node to perform actions on.

`NODE 3000`
Create a wallet and a new blockchain:

让我们来玩一下我们之前定义的场景。

首先，在第一个终端窗口中设置`NODE_ID`为 3000 ( )。我将在下一段或之前`export NODE_ID=3000`使用徽章，以便您了解要在哪个节点上执行操作。`NODE 3000``NODE 3001`

`NODE 3000`
创建钱包和新的区块链：

```shell
$ blockchain_go createblockchain -address CENTREAL_NODE
```

(I’ll use fake addresses for clarity and brevity)

After that, the blockchain will contain single genesis block. We need to save the block and use it in other nodes. Genesis blocks serve as identifiers of blockchains (in Bitcoin Core, the genesis block is hardcoded).

（为了清楚和简洁起见，我将使用假地址）

之后，区块链将包含单个创世块。我们需要保存该块并在其他节点中使用它。创世块充当区块链的标识符（在比特币核心中，创世块是硬编码的）。

```shell
$ cp blockchain_3000.db blockchain_genesis.db 
```

`NODE 3001`
Next, open a new terminal window and set node ID to 3001. This will be a wallet node. Generate some addresses with `blockchain_go createwallet`, we’ll call these addresses `WALLET_1`, `WALLET_2`, `WALLET_3`.

`NODE 3000`
Send some coins to the wallet addresses:

`NODE 3001`
接下来，打开一个新的终端窗口并将节点 ID 设置为 3001。这将是一个钱包节点。生成一些地址`blockchain_go createwallet`，我们将这些地址称为`WALLET_1`, `WALLET_2`, `WALLET_3`。

`NODE 3000`
发送一些硬币到钱包地址：

```shell
$ blockchain_go send -from CENTREAL_NODE -to WALLET_1 -amount 10 -mine
$ blockchain_go send -from CENTREAL_NODE -to WALLET_2 -amount 10 -mine
```

`-mine` flag means that the block will be immediately mined by the same node. We have to have this flag because initially there are no miner nodes in the network.
Start the node:

`-mine`flag 表示该区块将立即由同一节点开采。我们必须有这个标志，因为最初网络中没有矿工节点。
启动节点：

```shell
$ blockchain_go startnode
```

The node must be running until the end of the scenario.

`NODE 3001`
Start the node’s blockchain with the genesis block saved above:

```shell
$ cp blockchain_genesis.db blockchain_3001.db
```

Run the node:

```shell
$ blockchain_go startnode
```

It’ll download all the blocks from the central node. To check that everything’s ok, stop the node and check the balances:

```shell
$ blockchain_go getbalance -address WALLET_1
Balance of 'WALLET_1': 10

$ blockchain_go getbalance -address WALLET_2
Balance of 'WALLET_2': 10
```

Also, you can check the balance of the `CENTRAL_NODE` address, because the node 3001 now has its blockchain:

```shell
$ blockchain_go getbalance -address CENTRAL_NODE
Balance of 'CENTRAL_NODE': 10
```

`NODE 3002`
Open a new terminal window and set its ID to 3002, and generate a wallet. This will be a miner node. Initialize the blockchain:

```shell
$ cp blockchain_genesis.db blockchain_3002.db
```

And start the node:

```shell
$ blockchain_go startnode -miner MINER_WALLET
```

`NODE 3001`
Send some coins:

```shell
$ blockchain_go send -from WALLET_1 -to WALLET_3 -amount 1
$ blockchain_go send -from WALLET_2 -to WALLET_4 -amount 1
```

`NODE 3002`
Quickly! Switch to the miner node and see it mining a new block! Also, check the output of the central node.

`NODE 3001`
Switch to the wallet node and start it:

```shell
$ blockchain_go startnode
```

It’ll download the newly mined block!

Stop it and check balances:

```shell
$ blockchain_go getbalance -address WALLET_1
Balance of 'WALLET_1': 9

$ blockchain_go getbalance -address WALLET_2
Balance of 'WALLET_2': 9

$ blockchain_go getbalance -address WALLET_3
Balance of 'WALLET_3': 1

$ blockchain_go getbalance -address WALLET_4
Balance of 'WALLET_4': 1

$ blockchain_go getbalance -address MINER_WALLET
Balance of 'MINER_WALLET': 10
```

That’s it!

## Conclusion

This was the final part of the series. I could’ve publish some more posts implementing a real prototype of a P2P network, but I just don’t have time for this. I hope this article answers some of your questions about the Bitcoin technology and raises new ones, for which you can find answers yourself. There are more interesting things hidden in the Bitcoin technology! Good luck!

这是该系列的最后一部分。我本可以发表更多的帖子来实现 P2P 网络的真实原型，但我只是没有时间这样做。我希望本文能够回答您有关比特币技术的一些问题并提出新的问题，您可以自己找到答案。比特币技术中还隐藏着更多有趣的东西！祝你好运！

P.S. You can start improving the network with implementing the `addr` message, as described in the Bitcoin network protocol (link is below). This is a very important message, because it allows nodes to discover each other. I started implementing it, but haven’t finished!

PS 您可以通过实施该`addr`消息来开始改进网络，如比特币网络协议中所述（链接如下）。这是一条非常重要的消息，因为它允许节点相互发现。我开始实施它，但还没有完成！

Links:

1. [Source codes](https://github.com/Jeiwan/blockchain_go/tree/part_7)
2. [Bitcoin protocol documentation](https://en.bitcoin.it/wiki/Protocol_documentation)
3. [Bitcoin network](https://en.bitcoin.it/wiki/Network)
