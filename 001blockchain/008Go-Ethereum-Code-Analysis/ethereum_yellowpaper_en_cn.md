ETHEREUM: A SECURE DECENTRALISED GENERALISED TRANSACTION LEDGER PARIS VERSION 2f36cf0 – 2024-01-12

以太坊：一种安全去中心化的通用交易账本 **BERLIN** 版本 **f72032b - 2022-06-05**

DR. GAVIN WOOD FOUNDER, ETHEREUM & PARITY GAVIN@PARITY.IO

 Abstract. 摘要

The blockchain paradigm when coupled with cryptographically-secured transactions has demonstrated its utility through a number of projects, with Bitcoin being one of the most notable ones. Each such project can be seen as a simple application on a decentralised, but singleton, compute resource. We can call this paradigm a transactional singleton machine with shared-state. Ethereum implements this paradigm in a generalised manner. Furthermore it provides a plurality of such resources, each with a distinct state and operating code but able to interact through a message-passing framework with others. We discuss its design, implementation issues, the opportunities it provides and the future hurdles we envisage.

由密码学安全交易（cryptographically-secured transactions）所构成的区块链范式，已经通过一系列项目展示了它的实用性，不止是比特币。每一个这类项目都可以看作是一个基于去中心化的单实例计算资源所构建的简单应用程序。我们可以把这种范式称为具有共享状态（shared-state）的交易化单例状态机（transactional singleton machine）。以太坊以更通用的方式实现了这种范式。它提供了大量的资源，每一个资源都拥有独立的状态和操作码，并且可以通过消息传递方式和其它资源交互。我们将讨论它的设计、实现上的问题、它所提供的机遇以及我们所设想的未来障碍。

## 1. Introduction 简介

With ubiquitous internet connections in most places of the world, global information transmission has become incredibly cheap. Technology-rooted movements like Bitcoin have demonstrated through the power of the default, consensus mechanisms, and voluntary respect of the social contract, that it is possible to use the internet to make a decentralised value-transfer system that can be shared across the world and virtually free to use. This system can be said to be a very specialised version of a cryptographically secure, transaction-based state machine. Follow-up systems such as Namecoin adapted this original “currency application” of the technology into other applications, albeit rather simplistic ones.

随着互联网连接了世界上绝大多数地方，全球信息共享的成本越来越低。比特币网络通过共识机制、自愿遵守的社会合约，实现了一个去中心化的价值转移系统且可以在全球范围内自由使用，这样的技术改革展示了它的巨大力量。这样的系统可以说是加密安全、基于交易的状态机的一种具体应用。尽管还很简陋，但类似域名币（Namecoin）这样的后续系统，把这种技术从最初的“货币应用”发展到了其它领域。

 Ethereum is a project which attempts to build the generalised technology; technology on which all transactionbased state machine concepts may be built. Moreover it aims to provide to the end-developer a tightly integrated end-to-end system for building software on a hitherto unexplored compute paradigm in the mainstream: a trustful object messaging compute framework. 

以太坊是一个尝试达到通用性的技术项目，可以构建任何基于交易的状态机。而且以太坊致力于为开发者提供一个紧密整合的端到端系统，这个系统提供了一种可信对象消息传递计算框架（a trustful object messaging compute framework），使开发者可以用一种前所未有的计算范式来构建软件。

### 1.1. Driving Factors.驱动因素

   There are many goals of this project; one key goal is to facilitate transactions between consenting individuals who would otherwise have no means to trust one another. This may be due to geographical separation, interfacing difficulty, or perhaps the incompatibility, incompetence, unwillingness, expense, uncertainty, inconvenience, or corruption of existing legal systems. By specifying a state-change system through a rich and unambiguous language, and furthermore architecting a system such that we can reasonably expect that an agreement will be thus enforced autonomously, we can provide a means to this end. 

这个项目有很多目标，其中最重要的目标是为了促成那些本来无法信任对方的个体之间的交易。这种不信任可能是因为地理上的分隔、对接的困难，或者是不兼容、不称职、不情愿、费用、不确定、不方便，或现有法律系统的腐败。于是我们想用一个丰富且清晰的语言去实现一个状态变化的系统，期望协议可以自动被执行，我们可以为此提供一种实现方式。

​     Dealings in this proposed system would have several attributes not often found in the real world. The incorruptibility of judgement, often difficult to find, comes naturally from a disinterested algorithmic interpreter. Transparency, or being able to see exactly how a state or judgement came about through the transaction log and rules or instructional codes, never happens perfectly in human-based systems since natural language is necessarily vague, information is often lacking, and plain old prejudices are difficult to shake.

这里所提议的系统中的交易，有一些在现实世界中并不常见的属性。公允的裁判通常很难找到，但无私的算法解释器却可以天然地提供这种特性。自然语言必然是模糊的，会导致信息的缺失，同时那些平白的、旧有的成见很难被动摇，所以透明性，或者说通过交易日志和规则或代码指令来清晰地查看状态变化或者裁判结果的能力，在基于人来构建的系统中从来无法完美实现。

   Overall, we wish to provide a system such that users can be guaranteed that no matter with which other individuals, systems or organisations they interact, they can do so with absolute confidence in the possible outcomes and how those outcomes might come about. 

总的来说，我希望能提供一个系统，来给用户提供一些保证：无论是与其他个体、系统还是组织进行交互，他们都能完全信任可能的结果以及产生结果的过程。



### 1.2. Previous Work. 前人工作

Buterin [2013] first proposed the kernel of this work in late November, 2013. Though now evolved in many ways, the key functionality of a blockchain with a Turing-complete language and an effectively unlimited inter-transaction storage capability remains unchanged. 

Buterin [2013a] 在 2013 年 11 月下旬第一次提出了这种系统的核心机制。虽然现在已经在很多方面都有了进化，但其核心特性没有变化。那就是：这是一个具有图灵完备的语言和实质上具有无限的内部交易数据存储容量的区块链。

   Dwork and Naor [1992] provided the first work into the usage of a cryptographic proof of computational expenditure (“proof-of-work”) as a means of transmitting a value signal over the Internet. The value-signal was utilised here as a spam deterrence mechanism rather than any kind of currency, but critically demonstrated the potential for a basic data channel to carry a strong economic signal, allowing a receiver to make a physical assertion without having to rely upon trust. Back [2002] later produced a system in a similar vein. 

Dwork and Naor [1992] 第一次提出了使用计算开销的密码学证明（“proof-of-work”，即工作量证明）来作为在互联网上传递价值信号的一种手段。这里所使用的价值信号，是作为对抗垃圾邮件的震慑机制的，并不是任何形式的货币；但这却极大地论证了一种承载强大的经济信号的基本数据通道的潜在可能，这种通道允许数据接受者不依赖任何信任就可以做出物理断言。Back [2002] 后来设计了一个类似的系统。

The first example of utilising the proof-of-work as a strong economic signal to secure a currency was by Vishnumurthy et al. [2003]. In this instance, the token was used to keep peer-to-peer file trading in check, providing “consumers” with the ability to make micro-payments to “suppliers” for their services. The security model afforded by the proof-of-work was augmented with digital signatures and a ledger in order to ensure that the historical record couldn’t be corrupted and that malicious actors could not spoof payment or unjustly complain about service delivery. Five years later, Nakamoto [2008] introduced another such proof-of-work-secured value token, somewhat wider in scope. The fruits of this project, Bitcoin, became the first widely adopted global decentralised transaction ledger.

Vishnumurthy et al. [2003] 最早使用工作量证明作为强大的经济信号保证货币安全。在这个案例中，代币被用来在点到点（peer-to-peer）文件交易中检查并确保“消费者”可以向服务“提供商”进行小额支付。通过在工作量证明中增加数字签名和账本，一种新的安全模型产生了。这种模型是为了防止历史记录被篡改，并使恶意用户无法伪造交易或者不公平的抗议服务的交付。五年后（2008 年），中本聪 Nakamoto [2008] 引入了另一种更广泛的工作量证明安全价值代币。这个项目的成果就是比特币，它也成为了第一个被广泛认可的全球化去中心化交易账本。

 Other projects built on Bitcoin’s success; the alt-coins introduced numerous other currencies through alteration to the protocol. Some of the best known are Litecoin and Primecoin, discussed by Sprankel [2013]. Other projects sought to take the core value content mechanism of the protocol and repurpose it; Aron [2012] discusses, for example,the Namecoin project which aims to provide a decentralised name-resolution system.

其他建立在比特币成功基础上的项目； 通过修改协议，山寨币引入了许多其他货币。 其中最著名的是 Litecoin 和 Primecoin，由 Sprankel [2013] 讨论。 其他项目试图采用协议的核心价值内容机制并重新利用它； 例如，Aron [2012] 讨论了 Namecoin 项目，该项目旨在提供去中心化的名称解析系统。

Other projects still aim to build upon the Bitcoin network itself, leveraging the large amount of value placed in the system and the vast amount of computation that goes into the consensus mechanism. The Mastercoin project, first proposed by Willett [2013], aims to build a richer protocol involving many additional high-level features on top of the Bitcoin protocol through utilisation of a number of auxiliary parts to the core protocol. The Coloured Coins project, proposed by Rosenfeld et al. [2012], takes a similar but more simplified strategy, embellishing the rules of a transaction in order to break the fungibility of Bitcoin’s base currency and allow the creation and tracking of tokens through a special “chroma-wallet”-protocol-aware piece of software. 

其他项目仍然旨在建立在比特币网络本身的基础上，利用系统中的大量价值和共识机制中的大量计算。 Mastercoin 项目最初由 Willett [2013] 提出，旨在通过利用核心协议的许多辅助部分，在比特币协议之上构建一个更丰富的协议，涉及许多额外的高级功能。 彩色硬币项目，由罗森菲尔德等人提出。 [2012]，采取了类似但更简化的策略，完善了交易规则，以打破比特币基础货币的可互换性，并允许通过特殊的“色度钱包”协议感知部分创建和跟踪代币 软件。

Additional work has been done in the area with discarding the decentralisation foundation; Ripple, discussed by Boutellier and Heinzen [2014], has sought to create a “federated” system for currency exchange, effectively creating a new financial clearing system. It has demonstrated that high efficiency gains can be made if the decentralisation premise is discarded. Early work on smart contracts has been done by Szabo [1997] and Miller [1997]. Around the 1990s it became clear that algorithmic enforcement of agreements could become a significant force in human cooperation. Though no specific system was proposed to implement such a system, it was proposed that the future of law would be heavily affected by such systems. In this light, Ethereum may be seen as a general implementation of such a crypto-law system. 

在该领域已经完成了额外的工作，放弃了权力下放的基础； Boutellier 和 Heinzen [2014] 讨论的 Ripple 试图创建一个用于货币兑换的“联合”系统，从而有效地创建一个新的金融清算系统。 事实证明，如果放弃去中心化的前提，就能获得高效率的收益。 智能合约的早期工作是由 Szabo [1997] 和 Miller [1997] 完成的。 20 世纪 90 年代左右，人们逐渐认识到，协议的算法执行可能成为人类合作的重要力量。 尽管没有提出具体的制度来实施这样的制度，但有人提出，法律的未来将受到这种制度的严重影响。 从这个角度来看，以太坊可以被视为这种加密法律系统的一般实现。

For a list of terms used in this paper, refer to Appendix A.

本文中所使用的术语列表，请参考附录 A。

## 2.The Blockchain Paradigm 

Ethereum, taken as a whole, can be viewed as a transaction-based state machine: we begin with a genesis state and incrementally execute transactions to morph it into some current state. It is this current state which we accept as the canonical “version” of the world of Ethereum. The state can include such information as account balances, reputations, trust arrangements, data pertaining to information of the physical world; in short, anything that can currently be represented by a computer is admissible. Transactions thus represent a valid arc between two states; the ‘valid’ part is important—there exist far more invalid state changes than valid state changes. Invalid state changes might, e.g., be things such as reducing an account balance without an equal and opposite increase elsewhere. A valid state transition is one which comes about through a transaction. Formally:

以太坊在整体上可以看作一个基于交易的状态机：起始于一个创世区块（Genesis）状态，然后随着交易的执行状态逐步改变一直到最终状态，这个最终状态是以太坊世界的权威“版本”。状态中包含的信息有：账户余额、名誉度、信誉度、现实世界的附属数据等；简而言之，能包含电脑可以描绘的任何信息。而交易则成为了连接两个状态的有效桥梁；“有效”非常重要——因为无效的状态改变远超过有效的状态改变。例如：无效的状态改变可能是减少账户余额，但是没有在其它账户上加上同等的额度。一个有效的状态转换是通过交易进行的。可以正式地描述为：
$$
σ_{t+1} ≡ Υ(σ_t, T)
$$
where $Υ$ is the Ethereum state transition function. In Ethereum, $Υ$, together with $σ$ are considerably more powerful than any existing comparable system; $Υ$allows components to carry out arbitrary computation, while σ allows components to store arbitrary state between transactions.

$Υ$是以太坊状态转换函数。在以太坊中，$Υ$和 **σ** 比已有的任何比较系统都强;$ Υ$ 可以执行任意计算，而 $σ$ 可以储存交易中的任意状态。

Transactions are collated into blocks; blocks are chained together using a cryptographic hash as a means of reference. Blocks function as a journal, recording a series of transactions together with the previous block and an identifier for the final state (though do not store the final state itself—that would be far too big). 

交易被整理成区块； 使用加密哈希作为参考手段将块链接在一起。 块充当日志，记录一系列交易以及前一个块和最终状态的标识符（尽管不存储最终状态本身 - 这会太大）。

Formally, we expand to:

正式地，我们扩展到：
$$
σ_{t+1} ≡ Π(σ_t, B)
$$

$$
B ≡ (...,(T0, T1, ...), ...)
$$


$$
Π(σ, B) ≡ Υ(Υ(σ, T0), T1)...
$$
Where B is this block, which includes a series of transactions amongst some other components and $Π$ is the block-level state-transition function. 

其中 B 是该块，其中包括一些其他组件之间的一系列交易，$Π$ 是块级状态转换函数。

This is the basis of the blockchain paradigm, a model that forms the backbone of not only Ethereum, but all decentralised consensus-based transaction systems to date.

这是区块链范式的基础，该模型不仅构成了以太坊的支柱，而且构成了迄今为止所有基于共识的去中心化交易系统的支柱。

### 2.1. Value. 

In order to incentivise computation within the network, there needs to be an agreed method for transmitting value. To address this issue, Ethereum has an intrinsic currency, Ether, known also as ETH and sometimes referred to by the Old English $¯D$. The smallest subdenomination of Ether, and thus the one in which all integer values of the currency are counted, is the Wei. One Ether is defined as being $ 10^{18} $ Wei. There exist other subdenominations of Ether: 

为了激励网络内的计算，需要有一个商定的方法来传输价值。 为了解决这个问题，以太坊有一种内在货币，即以太币，也称为 ETH，有时用古英语 $¯D$ 来指代。 以太币的最小子面额，也就是所有货币整数值都被计算在内的子面额，是 Wei。 1 以太被定义为 $ 10^{18} $ Wei。 以太币还存在其他子名称：

| Multiplier | Name   |
| ---------- | ------ |
| $10^0$     | Wei    |
| $10^{12}$  | Szabo  |
| $10^{15}$  | Finney |
| $10^{18}$  | Ether  |

Throughout the present work, any reference to value, in the context of Ether, currency, a balance or a payment, should be assumed to be counted in Wei.

在当前的工作中，任何涉及以太币、货币、余额或支付的价值的提及都应假定以 Wei 计算。

### 2.2. Which History? 如何选择历史

Since the system is decentralised and all parties have an opportunity to create a new block on some older pre-existing block, the resultant structure is necessarily a tree of blocks. In order to form a consensus as to which path, from root (the genesis block) to leaf (the block containing the most recent transactions) through this tree structure, known as the blockchain, there must be an agreed-upon scheme. If there is ever a disagreement between nodes as to which root-to-leaf path down the block tree is the ‘best’ blockchain, then a fork occurs.

因为这是一个去中心化的系统，所有人都有机会在之前的某一个区块之后创建新的区块，这必然会形成一个区块树。为了能在这个区块树上从根节点（the genesis block）到叶节点（包含最新交易的区块）形成一个一致的区块链，必须有一个共识方案。如果各个节点所认为的从根节点到叶节点的路径（最佳区块链）不同，那么就会发生分叉。

This would mean that past a given point in time (block), multiple states of the system may coexist: some nodes believing one block to contain the canonical transactions, other nodes believing some other block to be canonical, potentially containing radically different or incompatible transactions. This is to be avoided at all costs as the uncertainty that would ensue would likely kill all confidence in the entire system.

这意味着在给定的时间点（区块）之后，系统的多种状态可能会共存：一些节点相信一个区块包含规范交易，其他节点相信其他某个区块是规范的，可能包含完全不同或不兼容的交易 。 应不惜一切代价避免这种情况，因为随之而来的不确定性可能会摧毁对整个系统的所有信心。

Since the Paris hard fork, reaching consensus on new blocks is managed by a protocol called the Beacon Chain. It is known as the consensus layer of Ethereum, and it defines the rules for determining the canonical history of Ethereum blocks. This document describes the execution layer of Ethereum. The execution layer defines the rules for interacting with and updating the state of the Ethereum virtual machine. The consensus layer is described in greater detail in the consensus specifications. How the consensus layer is used to determine the canonical state of Ethereum is discussed in section 11.

自巴黎硬分叉以来，就新区块达成共识是通过称为信标链的协议来管理的。 它被称为以太坊的共识层，它定义了确定以太坊区块的规范历史的规则。 本文档描述了以太坊的执行层。 执行层定义了与以太坊虚拟机交互和更新状态的规则。 共识层在共识规范中有更详细的描述。 第 11 节讨论了如何使用共识层来确定以太坊的规范状态。

There are many versions of Ethereum, as the protocol has undergone a number of updates. These updates can be specified to occur: • at a particular block number in the case of preParis updates, • after reaching a terminal total difficulty in the case of the Paris update, or • at a particular block timestamp in the case of post-Paris updates

以太坊有很多版本，因为协议已经经历了多次更新。 这些更新可以指定在以下情况发生： • 在巴黎前更新的情况下，在特定区块编号处； • 在巴黎更新的情况下，在达到终端总难度后；或者 • 在巴黎后更新的情况下，在特定区块时间戳处发生 更新

This document describes the Paris version. In order to follow back the history of a path, one must reference multiple versions of this document. Here are the block numbers of protocol updates on the Ethereum main network:

本文档描述了巴黎版本。 为了追溯一条路径的历史，必须引用该文档的多个版本。 以下是以太坊主网协议更新的区块号：

| Name                   | First Block Number |
| ---------------------- | ------------------ |
| $F_{Homestead}$        | 1150000            |
| $F_{TangerineWhistle}$ | 2463000            |
| $F_{SpuriousDragon}$   | 2675000            |
| $F_{Byzantium}$        | 4370000            |
| $F_{Constantinople}$   | 7280000            |
| $F_{Petersburg}$       | 7280000            |
| $F_{Istanbul}$         | 9069000            |
| $F_{MuirGlacier}$      | 9200000            |
| $F_{Berlin}$           | 12244000           |
| $F_{London}$           | 12965000           |
| $F_{ArrowGlacier}$     | 13773000           |
| $F_{GrayGlacier}$      | 15050000           |
| $F_{Paris}$            | 15537394           |
|                        |                    |

Occasionally actors do not agree on a protocol change, and a permanent fork occurs. In order to distinguish between diverged blockchains, EIP-155 by Buterin [2016] introduced the concept of chain ID, which we denote by β. For the Ethereum main network

偶尔，参与者在协议更改上不达成一致，就会出现永久性的分叉。为了区分不同的区块链，Buterin[2016]的EIP-155引入了链ID的概念，我们用β表示。对于以太坊主网络
$$
β = 1
$$


## 3.Conventions 约定

We use a number of typographical conventions for the formal notation, some of which are quite particular to the present work:

我们在正式符号中使用了许多印刷约定，其中一些对于当前的工作来说非常特殊：

The two sets of highly structured, ‘top-level’, state values, are denoted with bold lowercase Greek letters. They fall into those of world-state, which are denoted σ (or a variant thereupon) and those of machine-state, µ.

两组高度结构化的“顶级”国家价值观用粗体小写希腊字母表示。 它们属于世界状态，用 σ（或其变体）表示，以及机器状态，μ。

Functions operating on highly structured values are denoted with an upper-case Greek letter, e.g. Υ, the Ethereum state transition function.

对高度结构化值进行操作的函数用大写希腊字母表示，例如 Y，以太坊状态转换函数。

For most functions, an uppercase letter is used, e.g. C, the general cost function. These may be subscripted to denote specialised variants, e.g. CSSTORE, the cost function for the SSTORE operation. For specialised and possibly externally defined functions, we may format as typewriter text, e.g. the Keccak-256 hash function (as per version 3 of the winning entry to the SHA-3 contest by Bertoni et al. [2011], rather than the final SHA-3 specification), is denoted KEC (and generally referred to as plain Keccak). Also, KEC512 refers to the Keccak-512 hash function.

对于大多数函数，使用大写字母，例如 C、一般成本函数。 这些可以加下标来表示专门的变体，例如 CSSTORE，SSTORE 操作的成本函数。 对于专门的和可能外部定义的函数，我们可以将其格式化为打字机文本，例如 Keccak-256 哈希函数（根据 Bertoni 等人 [2011] 的 SHA-3 竞赛获奖作品的版本 3，而不是最终的 SHA-3 规范）表示为 KEC（通常称为普通 凯卡克）。 另外，KEC512 指的是 Keccak-512 哈希函数。

Tuples are typically denoted with an upper-case letter, e.g. T, is used to denote an Ethereum transaction. This symbol may, if accordingly defined, be subscripted to refer to an individual component, e.g. Tn, denotes the nonce of said transaction. The form of the subscript is used to denote its type; e.g. uppercase subscripts refer to tuples with subscriptable components.

元组通常用大写字母表示，例如 T，用于表示以太坊交易。 如果相应地定义，该符号可以带有下标以指代单个组件，例如 Tn，表示所述交易的随机数。 下标的形式用于表示其类型； 例如 大写下标指的是具有可下标组件的元组。

Scalars and fixed-size byte sequences (or, synonymously, arrays) are denoted with a normal lower-case letter, e.g. n is used in the document to denote a transaction nonce. Those with a particularly special meaning may be Greek, e.g. δ, the number of items required on the stack for a given operation.

标量和固定大小的字节序列（或同义的数组）用普通的小写字母表示，例如 文档中使用 n 表示交易随机数。 那些具有特别特殊含义的可能是希腊语，例如 δ，给定操作的堆栈上所需的项目数。

Arbitrary-length sequences are typically denoted as a bold lower-case letter, e.g. o is used to denote the byte sequence given as the output data of a message call. For particularly important values, a bold uppercase letter may be used.

任意长度的序列通常表示为粗体小写字母，例如 o 用于表示作为消息调用的输出数据给出的字节序列。 对于特别重要的值，可以使用粗体大写字母。

Throughout, we assume scalars are non-negative integers and thus belong to the set N. The set of all byte sequences is B, formally defined in Appendix B. If such a set of sequences is restricted to those of a particular length, it is denoted with a subscript, thus the set of all byte sequences of length 32 is named $B_{32}$and the set of all non-negative integers smaller than $2^{256}$ is named $N_{256}$. This is formally defined in section 4.3.

自始至终，我们假设标量是非负整数，因此属于集合 N。所有字节序列的集合是 B，在附录 B 中正式定义。如果这样的序列集合被限制为特定长度的序列，则它是 用下标表示，因此长度为 32 的所有字节序列的集合称为 $B_{32}$，所有小于 $2^{256}$ 的非负整数的集合称为 $N_{256}$。 这在第 4.3 节中有正式定义。

Square brackets are used to index into and reference individual components or subsequences of sequences, e.g. µs [0] denotes the first item on the machine’s stack. For subsequences, ellipses are used to specify the intended range, to include elements at both limits, e.g. µm[0..31] denotes the first 32 items of the machine’s memory.

方括号用于索引和引用单个组件或序列的子序列，例如 µs [0] 表示机器堆栈上的第一项。 对于子序列，省略号用于指定预期范围，以包括两个限制处的元素，例如 µm[0..31] 表示机器内存的前 32 项。

In the case of the global state σ, which is a sequence of accounts, themselves tuples, the square brackets are used to reference an individual account.

在全局状态 σ 的情况下，它是一个账户序列，它们本身就是元组，方括号用于引用单个账户。

When considering variants of existing values, we follow the rule that within a given scope for definition, if we assume that the unmodified ‘input’ value be denoted by the placeholder  then the modified and utilisable value is denoted as 0 , and intermediate values would be ∗ , ∗∗ &c. On very particular occasions, in order to maximise readability and only if unambiguous in meaning, we may use alpha-numeric subscripts to denote intermediate values, especially those of particular note.

在考虑现有值的变体时，我们遵循以下规则：在给定的定义范围内，如果我们假设未修改的“输入”值由占位符表示，则修改后的可用值表示为 0 ，中间值将表示为 ＊ 、 ＊＊ 等。 在非常特殊的情况下，为了最大限度地提高可读性并且仅在含义明确的情况下，我们可以使用字母数字下标来表示中间值，尤其是那些特别值得注意的值。

When considering the use of existing functions, given a function f, the function f ∗ denotes a similar, element-wise version of the function mapping instead between sequences. It is formally defined in section 4.3.

当考虑使用现有函数时，给定函数 f，函数 f * 表示函数映射的类似元素版本，而不是序列之间的映射。 它在 4.3 节中正式定义。

We define a number of useful functions throughout. One of the more common is `, which evaluates to the last item in the given sequence:

我们在整个过程中定义了许多有用的函数。 更常见的一个是`，它计算给定序列中的最后一项：
$$
l(x) ≡ x[||x|| − 1]
$$

## 4 Blocks, State and Transactions 区块、状态和交易

Having introduced the basic concepts behind Ethereum, we will discuss the meaning of a transaction, a block and the state in more detail.

介绍了以太坊背后的基本概念后，我们将更详细地讨论交易、区块和状态的含义。

### 4.1. World State. 世界状态

The world state (state), is a mapping between addresses (160-bit identifiers) and account states (a data structure serialised as RLP, see Appendix B). Though not stored on the blockchain, it is assumed that the implementation will maintain this mapping in a modified Merkle Patricia tree (trie, see Appendix D). The trie requires a simple database backend that maintains a mapping of byte arrays to byte arrays; we name this underlying database the state database. This has a number of benefits; firstly the root node of this structure is cryptographically dependent on all internal data and as such its hash can be used as a secure identity for the entire system state. Secondly, being an immutable data structure, it allows any previous state (whose root hash is known) to be recalled by simply altering the root hash accordingly. Since we store all such root hashes in the blockchain, we are able to trivially revert to old states.

世界状态（state）是地址（160位标识符）和账户状态（序列化为RLP的数据结构，参见附录B）之间的映射。 虽然没有存储在区块链上，但假设实现将在修改后的 Merkle Patricia 树（trie，参见附录 D）中维护此映射。 trie 需要一个简单的数据库后端来维护字节数组到字节数组的映射； 我们将这个底层数据库命名为状态数据库。 这有很多好处； 首先，该结构的根节点在加密上依赖于所有内部数据，因此其散列可以用作整个系统状态的安全身份。 其次，作为一种不可变的数据结构，它允许通过简单地相应地改变根哈希来调用任何先前的状态（其根哈希是已知的）。 由于我们将所有此类根哈希存储在区块链中，因此我们能够轻松恢复到旧状态。













































