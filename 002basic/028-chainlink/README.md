# ChainLink

## 01 区块链的概念和Web3 

## The concept of blockchain and Web3

### 什么是区块链？ What is blockchain?

区块链是一个分布式账本，这个账本维护了一个连续增长的数据列表，这些有序地数据存储在区块中，这些区块通过密码学连在一起，形成了一条链。

Blockchain is a distributed ledger that maintains a continuously growing list of data. These ordered data are stored in blocks, which are linked together through cryptography to form a chain.

区块是一个数据的容器，并且永久存储在区块链中。A block is a container of data and is permanently stored in the blockchain.

一个区块包含：A block contains:

- 交易 Transactions
- 时间戳 Timestamp
- 上一个区块的哈希 Hash of the previous block
- 随机数（nonce） Random number (nonce)

<img src="images/image-20240913110124156.png" alt="image-20240913110124156"  />

![image-20240913110416519](images/image-20240913110416519.png)

### 什么是状态机 What is a state machine

有限状态机（finate-state machine，缩写：FSM）又称有限状态自动机，简称状态机，是表示有限个状态以及在这些状态之间的转移和动作等行为的数学计算模型。A finite state machine (FSM), also known as a finite state automaton or state machine for short, is a mathematical computational model that represents a finite number of states and behaviors such as transitions and actions between these states.

![image-20240913113255118](images/image-20240913113255118.png)

![image-20240913113357921](images/image-20240913113357921.png)

### 区块链基础 Blockchain Basics

- 密码学（Cryptography）

- 分布式系统(Distributed System)

- 博弈论(Game Theory)

![image-20240913114157334](images/image-20240913114157334.png)

#### 区块链中的密码学 

#### Cryptography in blockchain

- 哈希函数（Hash Function）
- 默克尔树（Merkle Tree）
- 公钥和私钥（Public key and Private Key）

![image-20240913115038039](images/image-20240913115038039.png)

将一个任意长度的数据转化为一个固定长度的数据。Convert a data of arbitrary length into a data of fixed length.

哈希函数的特性 Characteristics of hash functions

1、确定性：同样的数据生成的哈希值相同。Determinism: The same data generates the same hash value.

2、单向高效：计算数据的哈希算法很高效。容易通过计算数据的哈希值，但很难从哈希值反推出原始数据。One-way efficiency: The hash algorithm for calculating data is very efficient. It is easy to calculate the hash value of the data, but it is difficult to reverse the original data from the hash value.

3、碰撞抵御：微小的改动会导致生成哈希值完全不同，两个不同的数据生成相同的哈希值的概率很低。Collision resistance: Minor changes will result in completely different hash values, and the probability of two different data generating the same hash value is very low.

比特币中的默克尔树

“Merkle trees are used to summarize all the transactions in  a block,producing an overall digital fingerprint of the entire set of transactions,providing a very efficient process to verify whether a transaction is including in a block”.

默克尔树可以将一个区块的所有交易生成一个数字指纹，提供一种搞笑的验证方式，用以验证区块中是否存在某一笔交易。

![image-20240913121804496](images/image-20240913121804496.png)



![img](images/68747470733a2f2f692e696d6775722e636f6d2f5873784d4130622e706e67.png)

![image-20240913122621748](images/image-20240913122621748.png)

Cryptographic Keys

两种类型的秘钥：Two types of keys:

- 对称加密（单个秘钥）Symmetric encryption (single key)
- 非对称加密（一对秘钥） Asymmetric encryption (a pair of keys)

在区块链中，使用的非对称加密 In blockchain, asymmetric encryption is used

![image-20240913124116030](images/image-20240913124116030.png)

![image-20240913124235715](images/BigData.png)

#### 分布式网络

![image-20240913124658426](images/image-20240913124658426.png)

#### 博弈论 Game theory

研究在多方参与的场景中，个体如何决策的一门科学 A science that studies how individuals make decisions in scenarios involving multiple parties

系统中每个个体的决策 The decisions of each individual in the system

- 完全独立 Completely independent
- 依赖于其他个体 Dependent on other individuals

研究最终决策的概率 Study the probability of the final decision

区块链中的博弈论：共识算法 Game theory in blockchain: consensus algorithm

提升不诚实节点的作恶成本，增加诚实节点的经济激励，让作恶的收益小于成本。来保证在区块链网络中个体和个体的交互诚实。Increase the cost of dishonest nodes to commit evil, increase the economic incentives of honest nodes, and make the benefits of committing evil less than the cost. To ensure the honesty of interactions between individuals in the blockchain network.

共识算法就是用来保证所有参与方能够认同同一种共识的算法，它是基于博弈论的理论去实现的。The consensus algorithm is an algorithm used to ensure that all participants can agree on the same consensus, which is implemented based on the theory of game theory.

工作量证明（PoW）

- 正向激励：每次成功修改链上状态，都有经济激励，来激励诚实的节点。Positive incentives: Every successful modification of the on-chain state has economic incentives to motivate honest nodes.
- 负向激励：每次修改链上状态需要进行大量的计算，来提升参与成本。Negative incentives: Every modification of the on-chain state requires a lot of calculations, which increases the cost of participation.
- 哈希算法：提交一次交易很困难，但是验证一次交易很简单。Hash algorithm: Submitting a transaction is difficult, but verifying a transaction is simple.

![image-20240913131609092](images/image-20240913131609092.png)



![image-20240913131852107](images/image-20240913131852107.png)

![image-20240913132037593](images/image-20240913132037593.png)

### web3

![image-20240913132338789](images/image-20240913132338789.png)

![image-20240913132540146](images/image-20240913132540146.png)





## 02 安装使用钱包&Solidity基础





## 03 预言机，ERC-20 Token和Chainlink喂价





## 04 Chainlink随机数游戏应用



## 05 社区项目介绍



## 06 Chainlink Automation动态NFT应用



## 07 Chainlink Functions获取链下数据



## 08 CCIP跨链Token转移 -USDC



## 09 社区项目介绍