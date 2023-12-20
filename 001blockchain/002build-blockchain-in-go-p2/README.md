# Building Blockchain in Go. Part 2: Proof-of-Work 工作量证明
## Introduction 介绍
In the previous article we built a very simple data structure, which is the essence of blockchain database. And we made it possible to add blocks to it with the chain-like relation between them: each block is linked to the previous one. Alas, our blockchain implementation has one significant flaw: adding blocks to the chain is easy and cheap. One of the keystones of blockchain and Bitcoin is that adding new blocks is a hard work. Today we’re going to fix this flaw.
在上一篇文章中，我们构建了一个非常简单的数据结构，这是区块链数据库的本质。我们可以通过它们之间的链状关系向其添加块：每个块都链接到前一个块。唉，我们的区块链实现有一个重大缺陷：向链中添加区块既简单又便宜。区块链和比特币的基石之一是添加新区块是一项艰巨的工作。今天我们将修复这个缺陷。
## Proof-of-Work 工作量证明
A key idea of blockchain is that one has to perform some hard work to put data in it. It is this hard work that makes blockchain secure and consistent. Also, a reward is paid for this hard work (this is how people get coins for mining).
区块链的一个关键思想是，人们必须执行一些艰苦的工作才能将数据放入其中。正是这种艰苦的工作使区块链变得安全和一致。此外，这种辛勤工作也会得到奖励（这就是人们获得硬币进行挖矿的方式）。
This mechanism is very similar to the one from real life: one has to work hard to get a reward and to sustain their life. In blockchain, some participants (miners) of the network work to sustain the network, to add new blocks to it, and get a reward for their work. As a result of their work, a block is incorporated into the blockchain in a secure way, which maintains the stability of the whole blockchain database. It’s worth noting that, the one who finished the work has to prove this.
这种机制与现实生活中的机制非常相似：一个人必须努力工作才能获得奖励并维持生命。在区块链中，网络的一些参与者（矿工）致力于维持网络，向其添加新区块，并因其工作而获得奖励。由于他们的工作，一个区块以安全的方式被合并到区块链中，从而保持了整个区块链数据库的稳定性。值得注意的是，完成工作的人必须证明这一点。
This whole “do hard work and prove” mechanism is called proof-of-work. It’s hard because it requires a lot of computational power: even high performance computers cannot do it quickly. Moreover, the difficulty of this work increases from time to time to keep new blocks rate at about 6 blocks per hour. In Bitcoin, the goal of such work is to find a hash for a block, that meets some requirements. And it’s this hash that serves as a proof. Thus, finding a proof is the actual work.
整个“努力工作并证明”的机制称为工作量证明。这很难，因为它需要大量的计算能力：即使是高性能计算机也无法快速完成。此外，这项工作的难度会不时增加，以将新区块率保持在每小时 6 个区块左右。在比特币中，此类工作的目标是找到一个满足某些要求的区块的哈希值。正是这个哈希值作为证明。因此，找到证明是实际的工作。
One last thing to note. Proof-of-Work algorithms must meet a requirement: doing the work is hard, but verifying the proof is easy. A proof is usually handed to someone else, so for them, it shouldn’t take much time to verify it.
最后要注意的一点。工作量证明算法必须满足一个要求：做这项工作很困难，但验证证明很容易。证明通常会交给其他人，所以对他们来说，验证它不应该花费太多时间。
## Hashing 散列法
In this paragraph, we’ll discuss hashing. If you’re familiar with the concept, you can skip this part.
在本段中，我们将讨论哈希。如果您熟悉这个概念，则可以跳过此部分。
Hashing is a process of obtaining a hash for specified data. A hash is a unique representation of the data it was calculated on. A hash function is a function that takes data of arbitrary size and produces a fixed size hash. Here are some key features of hashing:
哈希是获取指定数据哈希的过程。哈希是计算它所依据的数据的唯一表示形式。哈希函数是获取任意大小的数据并生成固定大小哈希的函数。以下是哈希的一些主要功能：
1.Original data cannot be restored from a hash. Thus, hashing is not encryption.
无法从哈希还原原始数据。因此，哈希不是加密。
2.Certain data can have only one hash and the hash is unique.

某些数据只能有一个哈希值，并且该哈希值是唯一的。

3.Changing even one byte in the input data will result in a completely different hash.

即使更改输入数据中的一个字节，也会导致完全不同的哈希值。

<img src="https://jeiwan.net/images/hashing-example.png" alt="Hashing example" style="zoom:50%;" />
Hashing functions are widely used to check the consistency of data. Some software providers publish checksums in addition to a software package. After downloading a file you can feed it to a hashing function and compare produced hash with the one provided by the software developer.

哈希函数被广泛用于检查数据的一致性。除了软件包之外，一些软件提供商还会发布校验和。下载文件后，您可以将其提供给哈希函数，并将生成的哈希值与软件开发人员提供的哈希值进行比较。

In blockchain, hashing is used to guarantee the consistency of a block. The input data for a hashing algorithm contains the hash of the previous block, thus making it impossible (or, at least, quite difficult) to modify a block in the chain: one has to recalculate its hash and hashes of all the blocks after it.

在区块链中，哈希用于保证区块的一致性。哈希算法的输入数据包含前一个区块的哈希值，因此不可能（或至少相当困难）修改链中的区块：必须重新计算其哈希值和之后所有区块的哈希值。

## Hashcash哈希现金

Bitcoin uses [Hashcash](https://en.wikipedia.org/wiki/Hashcash), a Proof-of-Work algorithm that was initially developed to prevent email spam. It can be split into the following steps:

比特币使用Hashcash，这是一种工作量证明算法，最初是为了防止垃圾邮件而开发的。它可以分为以下步骤：

1. Take some publicly known data (in case of email, it’s receiver’s email address; in case of Bitcoin, it’s block headers).以一些公开已知的数据为例（如果是电子邮件，它是收件人的电子邮件地址;如果是比特币，它是区块头）。

2. Add a counter to it. The counter starts at 0.向其添加计数器。计数器从 0 开始。

3. Get a hash of the 获取组合的哈希 `data + counter` combination. 值。

4. Check that the hash meets certain requirements. 

   检查哈希值是否满足某些要求。

   1. If it does, you’re done.如果是这样，你就完成了。
   2. If it doesn’t, increase the counter and repeat the steps 3 and 4.如果没有，请增加计数器并重复步骤 3 和 4。

Thus, this is a brute force algorithm: you change the counter, calculate a new hash, check it, increment the counter, calculate a hash, etc. That’s why it’s computationally expensive.

因此，这是一种蛮力算法：您更改计数器、计算新的哈希值、检查它、增加计数器、计算哈希值等。这就是为什么它在计算上很昂贵。

Now let’s look closer at the requirements a hash has to meet. In the original Hashcash implementation, the requirement sounds like “first 20 bits of a hash must be zeros”. In Bitcoin, the requirement is adjusted from time to time, because, by design, a block must be generated every 10 minutes, despite computation power increasing with time and more and more miners joining the network.

现在让我们仔细看看哈希必须满足的要求。在最初的 Hashcash 实现中，该要求听起来像是“哈希值的前 20 位必须为零”。在比特币中，要求会不时调整，因为根据设计，必须每 10 分钟生成一个区块，尽管计算能力随着时间的推移而增加，越来越多的矿工加入网络。

To demonstrate this algorithm, I took the data from the previous example (“I like donuts”) and found a hash that starts with 3 zero-bytes:

为了演示这个算法，我从前面的示例（“我喜欢甜甜圈”）中获取数据，并找到了一个以 3 个零字节开头的哈希值：

<img src="https://jeiwan.net/images/hashcash-example.png" alt="Hashcash example" style="zoom:50%;" />

`ca07ca` is the hexadecimal value of the counter, which is 13240266 in the decimal system.

"ca07ca"是计数器的十六进制值，在十进制系统中13240266。

## Implementation实现

Ok, we’re done with the theory, let’s write code! First, let’s define the difficulty of mining:

好了，我们已经完成了理论，让我们编写代码吧！首先，我们来定义挖矿的难度：

```go
const targetBits = 24
```

In Bitcoin, “target bits” is the block header storing the difficulty at which the block was mined. We won’t implement a target adjusting algorithm, for now, so we can just define the difficulty as a global constant.

在比特币中，“目标位”是存储区块开采难度的区块头。我们暂时不会实现目标调整算法，因此我们可以将难度定义为全局常数。

24 is an arbitrary number, our goal is to have a target that takes less than 256 bits in memory. And we want the difference to be significant enough, but not too big, because the bigger the difference the more difficult it’s to find a proper hash.

24 是一个任意数字，我们的目标是让目标在内存中占用少于 256 位。我们希望差异足够大，但不要太大，因为差异越大，找到合适的哈希值就越困难。









