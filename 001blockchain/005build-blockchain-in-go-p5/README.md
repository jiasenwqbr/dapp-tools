# Building Blockchain in Go. Part 5: Addresses

11 Sep 2017

## Introduction引言

In [the previous article](https://jeiwan.net/posts/building-blockchain-in-go-part-4/), we started implementing transactions. You were also introduced to the impersonal nature of transactions: there are no user accounts, your personal data (e.g., name, passport number or SSN) is not required and not stored anywhere in Bitcoin. But there still must be something that identifies you as the owner of transaction outputs (i.e. the owner of coins locked on these outputs). And this is what Bitcoin addresses are needed for. So far we’ve used arbitrary user defined strings as addresses, and the time has come to implement real addresses, as they’re implemented in Bitcoin.

在上一篇文章中，我们已经初步实现了交易。相信你应该了解了交易中的一些天然属性，这些属性没有丝毫“个人”色彩的存在：在比特币中，没有用户账户，不需要也不会在任何地方存储个人数据（比如姓名，护照号码或者 SSN）。但是，我们总要有某种途径识别出你是交易输出的所有者（也就是说，你拥有在这些输出上锁定的币）。这就是比特币地址（address）需要完成的使命。在上一篇中，我们把一个由用户定义的任意字符串当成是地址，现在我们将要实现一个跟比特币一样的真实地址。

> This part introduces significant code changes, so it makes no sense explaining all of them here. Please refer to [this page](https://github.com/Jeiwan/blockchain_go/compare/part_4...part_5#files_bucket) to see all the changes since the last article.
>
> 本文的代码实现变化很大，请点击 [这里](https://github.com/Jeiwan/blockchain_go/compare/part_4...part_5#files_bucket) 查看所有的代码更改。

## Bitcoin Address 比特币地址

Here’s an example of a Bitcoin address: [1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa](https://blockchain.info/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa). This is the very first Bitcoin address, which allegedly belongs to Satoshi Nakamoto. Bitcoin addresses are public. If you want to send coins to someone, you need to know their address. But addresses (despite being unique) are not something that identifies you as the owner of a “wallet”. In fact, such addresses are a human readable representation of public keys. In Bitcoin, your identity is a pair (or pairs) of private and public keys stored on your computer (or stored in some other place you have access to). Bitcoin relies on a combination of cryptography algorithms to create these keys, and guarantee that no one else in the world can access your coins without getting physical access to your keys. Let’s discuss what these algorithms are.

这就是一个真实的比特币地址：[1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa](https://blockchain.info/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa)。这是史上第一个比特币地址，据说属于中本聪。比特币地址是完全公开的，如果你想要给某个人发送币，只需要知道他的地址就可以了。但是，地址（尽管地址也是独一无二的）并不是用来证明你是一个“钱包”所有者的信物。实际上，所谓的地址，只不过是将公钥表示成人类可读的形式而已，因为原生的公钥人类很难阅读。在比特币中，你的身份（identity）就是一对（或者多对）保存在你的电脑（或者你能够获取到的地方）上的公钥（public key）和私钥（private key）。比特币基于一些加密算法的组合来创建这些密钥，并且保证了在这个世界上没有其他人能够取走你的币，除非拿到你的密钥。下面，让我们来讨论一下这些算法到底是什么。

## Public-key Cryptography 公钥加密

Public-key cryptography algorithms use pairs of keys: public keys and private keys. Public keys are not sensitive and can be disclosed to anyone. In contrast, private keys shouldn’t be disclosed: no one but the owner should have access to them because it’s private keys that serve as the identifier of the owner. You are your private keys (in the world of cryptocurrencies, of course).

公钥加密（public-key cryptography）算法使用的是成对的密钥：公钥和私钥。公钥并不是敏感信息，可以告诉其他人。但是，私钥绝对不能告诉其他人：只有所有者（owner）才能知道私钥，能够识别，鉴定和证明所有者身份的就是私钥。在加密货币的世界中，你的私钥代表的就是你，私钥就是一切。

In essence, a Bitcoin wallet is just a pair of such keys. When you install a wallet application or use a Bitcoin client to generate a new address, a pair of keys is generated for you. The one who controls the private key controls all the coins sent to this key in Bitcoin.

本质上，比特币钱包也只不过是这样的密钥对而已。当你安装一个钱包应用，或是使用一个比特币客户端来生成一个新地址时，它就会为你生成一对密钥。在比特币中，谁拥有了私钥，谁就可以控制所有发送到这个公钥的币。

Private and public keys are just random sequences of bytes, thus they cannot be printed on the screen and read by a human. That’s why Bitcoin uses an algorithm to convert public keys into a human readable string.

私钥和公钥只不过是随机的字节序列，因此它们无法在屏幕上打印，人类也无法通过肉眼去读取。这就是为什么比特币使用了一个转换算法，将公钥转化为一个人类可读的字符串（也就是我们看到的地址）。

> If you’ve ever used a Bitcoin wallet application, it’s likely that a mnemonic pass phrase was generated for you. Such phrases are used instead of private keys and can be used to generate them. This mechanism is implemented in [BIP-039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki).
>
> 如果你用过比特币钱包应用，很可能它会为你生成一个助记符。这样的助记符可以用来替代私钥，并且可以被用于生成私钥。[BIP-039](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) 已经实现了这个机制。

Ok, we now know what identifies users in Bitcoin. But how does Bitcoin check the ownership of transaction outputs (and coins stored on them)?

好了，现在我们已经知道了在比特币中证明用户身份的是私钥。那么，比特币如何检查交易输出（和存储在里面的币）的所有权呢？

## Digital Signatures 数字签名

In mathematics and cryptography, there’s a concept of digital signature – algorithms that guarantee:

在数学和密码学中，有一个数字签名（digital signature）的概念，算法可以保证：

1. that data wasn’t modified while being transferred from a sender to a recipient;

   当数据从发送方传送到接收方时，数据不会被修改；

2. that data was created by a certain sender;

   数据由某一确定的发送方创建；

3. that the sender cannot deny sending the data.

   发送方无法否认发送过数据这一事实。

By applying a signing algorithm to data (i.e., signing the data), one gets a signature, which can later be verified. Digital signing happens with the usage of a private key, and verification requires a public key.通过在数据上应用签名算法（也就是对数据进行签名），你就可以得到一个签名，这个签名晚些时候会被验证。生成数字签名需要一个私钥，而验证签名需要一个公钥。签名有点类似于印章，比方说我做了一幅画，完了用印章一盖，就说明了这幅画是我的作品。给数据生成签名，就是给数据盖了章。

In order to sign data we need the following things:

为了对数据进行签名，我们需要下面两样东西：

1. data to sign;

   要签名的数据

2. private key.

   私钥

The operation of signing produces a signature, which is stored in transaction inputs. In order to verify a signature, the following is required:

应用签名算法生成一个签名，并且这个签名会被存储到交易输入中。为了对一个签名进行验证，我们需要做三样东西:

1. data that was signed;

   被签名的数据

2. the signature;

   签名

3. public key.

   公钥

In simple terms, the verification process can be described as: check that this signature was obtained from this data with a private key used to generate the public key.

简单来说，验证过程可以被描述为：检查签名是由被签名数据加上私钥得来，并且公钥恰好是由该私钥生成。

> Digital signatures are not encryption, you cannot reconstruct the data from a signature. This is similar to hashing: you run data through a hashing algorithm and get a unique representation of the data. The difference between signatures and hashes is key pairs: they make signature verification possible.
> But key pairs can also be used to encrypt data: a private key is used to encrypt, and a public key is used to decrypt the data. Bitcoin doesn’t use encryption algorithms though.
>
> 数据签名并不是加密，你无法从一个签名重新构造出数据。这有点像哈希：你在数据上运行一个哈希算法，然后得到一个该数据的唯一表示。签名与哈希的区别在于密钥对：有了密钥对，才有签名验证。但是密钥对也可以被用于加密数据：私钥用于加密，公钥用于解密数据。不过比特币并不使用加密算法。

Every transaction input in Bitcoin is signed by the one who created the transaction. Every transaction in Bitcoin must be verified before being put in a block. Verification means (besides other procedures):

在比特币中，每一笔交易输入都会由创建交易的人签名。在被放入到一个块之前，必须要对每一笔交易进行验证。除了一些其他步骤，验证意味着：

1. Checking that inputs have permission to use outputs from previous transactions.

   检查交易输入有权使用来自之前交易的输出

2. Checking that the transaction signature is correct.

   检查交易签名是正确的

Schematically, the process of signing data and verifying signature looks likes this:

如图，对数据进行签名和对签名进行验证的过程大致如下：

![Digital Signatures](https://jeiwan.net/images/signing-scheme.png)

Let’s now review the full lifecycle of a transaction:

现在来回顾一个交易完整的生命周期：

1. In the beginning, there’s the genesis block that contains a coinbase transaction. There are no real inputs in coinbase transactions, so signing is not necessary. The output of the coinbase transaction contains a hashed public key (`RIPEMD16(SHA256(PubKey))` algorithms are used).

   起初，创世块里面包含了一个 coinbase 交易。在 coinbase 交易中，没有输入，所以也就不需要签名。coinbase 交易的输出包含了一个哈希过的公钥（使用的是 **RIPEMD16(SHA256(PubKey))** 算法）

2. When one sends coins, a transaction is created. Inputs of the transaction will reference outputs from previous transaction(s). Every input will store a public key (not hashed) and a signature of the whole transaction.

   当一个人发送币时，就会创建一笔交易。这笔交易的输入会引用之前交易的输出。每个输入会存储一个公钥（没有被哈希）和整个交易的一个签名。

3. Other nodes in the Bitcoin network that receive the transaction will verify it. Besides other things, they will check that: the hash of the public key in an input matches the hash of the referenced output (this ensures that the sender spends only coins belonging to them); the signature is correct (this ensures that the transaction is created by the real owner of the coins).比特币网络中接收到交易的其他节点会对该交易进行验证。除了一些其他事情，他们还会检查：在一个输入中，公钥哈希与所引用的输出哈希相匹配（这保证了发送方只能花费属于自己的币）；签名是正确的（这保证了交易是由币的实际拥有者所创建）。

4. When a miner node is ready to mine a new block, it’ll put the transaction in a block and start mining it.

   当一个矿工准备挖一个新块时，他会将交易放到块中，然后开始挖矿。

5. When the blocked is mined, every other node in the network receives a message saying the block is mined and adds the block to the blockchain.

   当新块被挖出来以后，网络中的所有其他节点会接收到一条消息，告诉其他人这个块已经被挖出并被加入到区块链。

6. After a block is added to the blockchain, the transaction is completed, its outputs can be referenced in new transactions.

   当一个块被加入到区块链以后，交易就算完成，它的输出就可以在新的交易中被引用。

## Elliptic Curve Cryptography 椭圆曲线加密

As described above, public and private keys are sequences of random bytes. Since it’s private keys that are used to identify owners of coins, there’s a required condition: the randomness algorithm must produce truly random bytes. We don’t want to accidentally generate a private key that’s owned by someone else.

正如之前提到的，公钥和私钥是随机的字节序列。私钥能够用于证明持币人的身份，需要有一个条件：随机算法必须生成真正随机的字节。因为没有人会想要生成一个私钥，而这个私钥意外地也被别人所有。

Bitcoin uses elliptic curves to generate private keys. Elliptic curves is a complex mathematical concept, which we’re not going to explain in details here (if you’re curious, check out [this gentle introduction to elliptic curves](http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/) WARNING: Math formulas!). What we need to know is that these curves can be used to generate really big and random numbers. The curve used by Bitcoin can randomly pick a number between 0 and 2²⁵⁶ (which is approximately 10⁷⁷, when there are between 10⁷⁸ and 10⁸² atoms in the visible universe). Such a huge upper limit means that it’s almost impossible to generate the same private key twice.

比特币使用椭圆曲线来产生私钥。椭圆曲线是一个复杂的数学概念，我们并不打算在这里作太多解释（如果你真的十分好奇，可以查看[这篇文章](http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)，注意：有很多数学公式！）我们只要知道这些曲线可以生成非常大的随机数就够了。在比特币中使用的曲线可以随机选取在 0 与 $2 ^ {256}$（大概是 $10^{77}$, 而整个可见的宇宙中，原子数在 $10^{78}$到 $10^{82}$之间） 的一个数。有如此高的一个上限，意味着几乎不可能发生有两次生成同一个私钥的事情。

Also, Bitcoin uses (and we will) ECDSA (Elliptic Curve Digital Signature Algorithm) algorithm to sign transactions.

比特币使用的是 ECDSA（Elliptic Curve Digital Signature Algorithm）算法来对交易进行签名，我们也会使用该算法。

## Base58

Now let’s get back to the above mentioned Bitcoin address: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa. Now we know that this is a human-readable representation of a public key. And if we decode it, here’s what the public key looks like (as a sequence of bytes written in the hexadecimal system):

回到上面提到的比特币地址：1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa 。现在，我们已经知道了这是公钥用人类可读的形式表示而已。如果我们对它进行解码，就会看到公钥的本来面目（16 进制表示的字节）：

```shell
0062E907B15CBF27D5425399EBF6F0FB50EBB88F18C29B7D93
```

Bitcoin uses the Base58 algorithm to convert public keys into human readable format. The algorithm is very similar to famous Base64, but it uses shorter alphabet: some letters were removed from the alphabet to avoid some attacks that use letters similarity. Thus, there are no these symbols: 0 (zero), O (capital o), I (capital i), l (lowercase L), because they look similar. Also, there are no + and / symbols.

比特币使用 Base58 算法将公钥转换成人类可读的形式。这个算法跟著名的 Base64 很类似，区别在于它使用了更短的字母表：为了避免一些利用字母相似性的攻击，从字母表中移除了一些字母。也就是，没有这些符号：0(零)，O(大写的 o)，I(大写的i)，l(小写的 L)，因为这几个字母看着很像。另外，也没有 + 和 / 符号。

Let’s schematically visualize the process of getting an address from a public key:

下图是从一个公钥获得一个地址的过程：

![Address Generation](https://jeiwan.net/images/address-generation-scheme.png)

Thus, the above mentioned decoded public key consists of three parts:

因此，上面提到的公钥解码后包含三个部分：

```shell
Version  Public key hash                           Checksum
00       62E907B15CBF27D5425399EBF6F0FB50EBB88F18  C29B7D93
```

Since hashing functions are one way (i.e., they cannot be reversed), it’s not possible to extract the public key from the hash. But we can check if a public key was used to get the hash by running it thought the save hash functions and comparing the hashes.

由于哈希函数是单向的（也就说无法逆转回去），所以不可能从一个哈希中提取公钥。不过通过执行哈希函数并进行哈希比较，我们可以检查一个公钥是否被用于哈希的生成。

Ok, now that we have all the pieces, let’s write some code. Some of the concepts should be more clear when written in code.

好了，所有细节都已就绪，来写代码吧。很多概念只有当写代码的时候，才能理解地更透彻。

## Implementing Addresses

We’ll start with the `Wallet` structure:

我们先从钱包 `Wallet` 结构开始：

```go
type Wallet struct {
	PrivateKey ecdsa.PrivateKey
	PublicKey  []byte
}

type Wallets struct {
	Wallets map[string]*Wallet
}

func NewWallet() *Wallet {
	private, public := newKeyPair()
	wallet := Wallet{private, public}

	return &wallet
}

func newKeyPair() (ecdsa.PrivateKey, []byte) {
	curve := elliptic.P256()
	private, err := ecdsa.GenerateKey(curve, rand.Reader)
	pubKey := append(private.PublicKey.X.Bytes(), private.PublicKey.Y.Bytes()...)

	return *private, pubKey
}
```

A wallet is nothing but a key pair. We’ll also need the `Wallets` type to keep a collection of wallets, save them to a file, and load them from it. In the construction function of `Wallet` a new key pair is generated. The `newKeyPair` function is straightforward: ECDSA is based on elliptic curves, so we need one. Next, a private key is generated using the curve, and a public key is generated from the private key. One thing to notice: in elliptic curve based algorithms, public keys are points on a curve. Thus, a public key is a combination of X, Y coordinates. In Bitcoin, these coordinates are concatenated and form a public key.

一个钱包只有一个密钥对而已。我们需要 `Wallets` 类型来保存多个钱包的组合，将它们保存到文件中，或者从文件中进行加载。`Wallet` 的构造函数会生成一个新的密钥对。`newKeyPair` 函数非常直观：ECDSA 基于椭圆曲线，所以我们需要一个椭圆曲线。接下来，使用椭圆生成一个私钥，然后再从私钥生成一个公钥。有一点需要注意：在基于椭圆曲线的算法中，公钥是曲线上的点。因此，公钥是 X，Y 坐标的组合。在比特币中，这些坐标会被连接起来，然后形成一个公钥。

Now, let’s generate an address:

现在，来生成一个地址：

```go
func (w Wallet) GetAddress() []byte {
	pubKeyHash := HashPubKey(w.PublicKey)

	versionedPayload := append([]byte{version}, pubKeyHash...)
	checksum := checksum(versionedPayload)

	fullPayload := append(versionedPayload, checksum...)
	address := Base58Encode(fullPayload)

	return address
}

func HashPubKey(pubKey []byte) []byte {
	publicSHA256 := sha256.Sum256(pubKey)

	RIPEMD160Hasher := ripemd160.New()
	_, err := RIPEMD160Hasher.Write(publicSHA256[:])
	publicRIPEMD160 := RIPEMD160Hasher.Sum(nil)

	return publicRIPEMD160
}

func checksum(payload []byte) []byte {
	firstSHA := sha256.Sum256(payload)
	secondSHA := sha256.Sum256(firstSHA[:])

	return secondSHA[:addressChecksumLen]
}
```

Here are the steps to convert a public key into a Base58 address:

将一个公钥转换成一个 Base58 地址需要以下步骤：

1. Take the public key and hash it twice with `RIPEMD160(SHA256(PubKey))` hashing algorithms.

   使用 `RIPEMD160(SHA256(PubKey))` 哈希算法，取公钥并对其哈希两次

2. Prepend the version of the address generation algorithm to the hash.

   给哈希加上地址生成算法版本的前缀

3. Calculate the checksum by hashing the result of step 2 with `SHA256(SHA256(payload))`. The checksum is the first four bytes of the resulted hash.

   对于第二步生成的结果，使用 `SHA256(SHA256(payload))` 再哈希，计算校验和。校验和是结果哈希的前四个字节。

4. Append the checksum to the `version+PubKeyHash` combination.

   将校验和附加到 `version+PubKeyHash` 的组合中。

5. Encode the `version+PubKeyHash+checksum` combination with Base58.

   使用 Base58 对 `version+PubKeyHash+checksum` 组合进行编码。

As a result, you’ll get a **real Bitcoin address**, you can even check its balance on [blockchain.info](https://blockchain.info/). But I can assure you that the balance is 0 no matter how many times you generate a new address and check its balance. This is why choosing proper public-key cryptography algorithm is so crucial: considering private keys are random numbers, the chance of generating the same number must be as low as possible. Ideally, it must be as low as “never”.

至此，就可以得到一个**真实的比特币地址**，你甚至可以在 [blockchain.info](https://blockchain.info/) 查看它的余额。不过我可以负责任地说，无论生成一个新的地址多少次，检查它的余额都是 0。这就是为什么选择一个合适的公钥加密算法是如此重要：考虑到私钥是随机数，生成同一个数字的概率必须是尽可能地低。理想情况下，必须是低到“永远”不会重复。

Also, pay attention that you don’t need to connect to a Bitcoin node to get an address. The address generation algorithm utilizes a combination of open algorithms that are implemented in many programming languages and libraries.

Now we need to modify inputs and outputs for them to use addresses:

```go
type TXInput struct {
	Txid      []byte
	Vout      int
	Signature []byte
	PubKey    []byte
}

func (in *TXInput) UsesKey(pubKeyHash []byte) bool {
	lockingHash := HashPubKey(in.PubKey)

	return bytes.Compare(lockingHash, pubKeyHash) == 0
}

type TXOutput struct {
	Value      int
	PubKeyHash []byte
}

func (out *TXOutput) Lock(address []byte) {
	pubKeyHash := Base58Decode(address)
	pubKeyHash = pubKeyHash[1 : len(pubKeyHash)-4]
	out.PubKeyHash = pubKeyHash
}

func (out *TXOutput) IsLockedWithKey(pubKeyHash []byte) bool {
	return bytes.Compare(out.PubKeyHash, pubKeyHash) == 0
}
```

Notice, that we’re no longer using `ScriptPubKey` and `ScriptSig` fields, because we’re not going to implement a scripting language. Instead, `ScriptSig` is split into `Signature` and `PubKey` fields, and `ScriptPubKey` is renamed to `PubKeyHash`. We’ll implement the same outputs locking/unlocking and inputs signing logics as in Bitcoin, but we’ll do this in methods instead.

The `UsesKey` method checks that an input uses a specific key to unlock an output. Notice that inputs store raw public keys (i.e., not hashed), but the function takes a hashed one. `IsLockedWithKey` checks if provided public key hash was used to lock the output. This is a complementary function to `UsesKey`, and they’re both used in `FindUnspentTransactions` to build connections between transactions.

`Lock` simply locks an output. When we send coins to someone, we know only their address, thus the function takes an address as the only argument. The address is then decoded and the public key hash is extracted from it and saved in the `PubKeyHash` field.

Now, let’s check that everything works correctly:

```shell
$ blockchain_go createwallet
Your new address: 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt

$ blockchain_go createwallet
Your new address: 15pUhCbtrGh3JUx5iHnXjfpyHyTgawvG5h

$ blockchain_go createwallet
Your new address: 1Lhqun1E9zZZhodiTqxfPQBcwr1CVDV2sy

$ blockchain_go createblockchain -address 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt
0000005420fbfdafa00c093f56e033903ba43599fa7cd9df40458e373eee724d

Done!

$ blockchain_go getbalance -address 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt
Balance of '13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt': 10

$ blockchain_go send -from 15pUhCbtrGh3JUx5iHnXjfpyHyTgawvG5h -to 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt -amount 5
2017/09/12 13:08:56 ERROR: Not enough funds

$ blockchain_go send -from 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt -to 15pUhCbtrGh3JUx5iHnXjfpyHyTgawvG5h -amount 6
00000019afa909094193f64ca06e9039849709f5948fbac56cae7b1b8f0ff162

Success!

$ blockchain_go getbalance -address 13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt
Balance of '13Uu7B1vDP4ViXqHFsWtbraM3EfQ3UkWXt': 4

$ blockchain_go getbalance -address 15pUhCbtrGh3JUx5iHnXjfpyHyTgawvG5h
Balance of '15pUhCbtrGh3JUx5iHnXjfpyHyTgawvG5h': 6

$ blockchain_go getbalance -address 1Lhqun1E9zZZhodiTqxfPQBcwr1CVDV2sy
Balance of '1Lhqun1E9zZZhodiTqxfPQBcwr1CVDV2sy': 0
```

Nice! Now let’s implement transaction signatures.

## Implementing Signatures

Transactions must be signed because this is the only way in Bitcoin to guarantee that one cannot spend coins belonging to someone else. If a signature is invalid, the transaction is considered invalid too and, thus, cannot be added to the blockchain.

We have all the pieces to implement transactions signing, except one thing: data to sign. What parts of a transaction are actually signed? Or a transaction is signed as a whole? Choosing data to sign is quite important. The thing is that data to be signed must contain information that identifies the data in a unique way. For example, it makes no sense signing just output values because such signature won’t consider the sender and the recipient.

Considering that transactions unlock previous outputs, redistribute their values, and lock new outputs, the following data must be signed:

1. Public key hashes stored in unlocked outputs. This identifies “sender” of a transaction.
2. Public key hashes stored in new, locked, outputs. This identifies “recipient” of a transaction.
3. Values of new outputs.

> In Bitcoin, locking/unlocking logic is stored in scripts, which are stored in `ScriptSig` and `ScriptPubKey` fields of inputs and outputs, respectively. Since Bitcoins allows different types of such scripts, it signs the whole content of `ScriptPubKey`.

As you can see, we don’t need to sign the public keys stored in inputs. Because of this, in Bitcoin, it’s not a transaction that’s signed, but its trimmed copy with inputs storing `ScriptPubKey` from referenced outputs.

> A detailed process of getting a trimmed transaction copy is described [here](https://en.bitcoin.it/wiki/File:Bitcoin_OpCheckSig_InDetail.png). It’s likely to be outdated, but I didn’t manage to find a more reliable source of information.

Ok, it looks complicated, so let’s start coding. We’ll start with the `Sign` method:

```go
func (tx *Transaction) Sign(privKey ecdsa.PrivateKey, prevTXs map[string]Transaction) {
	if tx.IsCoinbase() {
		return
	}

	txCopy := tx.TrimmedCopy()

	for inID, vin := range txCopy.Vin {
		prevTx := prevTXs[hex.EncodeToString(vin.Txid)]
		txCopy.Vin[inID].Signature = nil
		txCopy.Vin[inID].PubKey = prevTx.Vout[vin.Vout].PubKeyHash
		txCopy.ID = txCopy.Hash()
		txCopy.Vin[inID].PubKey = nil

		r, s, err := ecdsa.Sign(rand.Reader, &privKey, txCopy.ID)
		signature := append(r.Bytes(), s.Bytes()...)

		tx.Vin[inID].Signature = signature
	}
}
```

The method takes a private key and a map of previous transactions. As mentioned above, in order to sign a transaction, we need to access the outputs referenced in the inputs of the transaction, thus we need the transactions that store these outputs.

Let’s review this method step by step:

```go
if tx.IsCoinbase() {
	return
}
```

Coinbase transactions are not signed because there are no real inputs in them.

```go
txCopy := tx.TrimmedCopy()
```

A trimmed copy will be signed, not a full transaction:

```go
func (tx *Transaction) TrimmedCopy() Transaction {
	var inputs []TXInput
	var outputs []TXOutput

	for _, vin := range tx.Vin {
		inputs = append(inputs, TXInput{vin.Txid, vin.Vout, nil, nil})
	}

	for _, vout := range tx.Vout {
		outputs = append(outputs, TXOutput{vout.Value, vout.PubKeyHash})
	}

	txCopy := Transaction{tx.ID, inputs, outputs}

	return txCopy
}
```

The copy will include all the inputs and outputs, but `TXInput.Signature` and `TXInput.PubKey` are set to nil.

Next, we iterate over each input in the copy:

```go
for inID, vin := range txCopy.Vin {
	prevTx := prevTXs[hex.EncodeToString(vin.Txid)]
	txCopy.Vin[inID].Signature = nil
	txCopy.Vin[inID].PubKey = prevTx.Vout[vin.Vout].PubKeyHash
```

In each input, `Signature` is set to `nil` (just a double-check) and `PubKey` is set to the `PubKeyHash` of the referenced output. At this moment, all transactions but the current one are “empty”, i.e. their `Signature` and `PubKey` fields are set to nil. Thus, **inputs are signed separately**, although this is not necessary for our application, but Bitcoin allows transactions to contain inputs referencing different addresses.

```go
	txCopy.ID = txCopy.Hash()
	txCopy.Vin[inID].PubKey = nil
```

The `Hash` method serializes the transaction and hashes it with the SHA-256 algorithm. The resulted hash is the data we’re going to sign. After getting the hash we should reset the `PubKey` field, so it doesn’t affect further iterations.

Now, the central piece:

```go
	r, s, err := ecdsa.Sign(rand.Reader, &privKey, txCopy.ID)
	signature := append(r.Bytes(), s.Bytes()...)

	tx.Vin[inID].Signature = signature
```

We sign `txCopy.ID` with `privKey`. An ECDSA signature is a pair of numbers, which we concatenate and store in the input’s `Signature` field.

Now, the verification function:

```go
func (tx *Transaction) Verify(prevTXs map[string]Transaction) bool {
	txCopy := tx.TrimmedCopy()
	curve := elliptic.P256()

	for inID, vin := range tx.Vin {
		prevTx := prevTXs[hex.EncodeToString(vin.Txid)]
		txCopy.Vin[inID].Signature = nil
		txCopy.Vin[inID].PubKey = prevTx.Vout[vin.Vout].PubKeyHash
		txCopy.ID = txCopy.Hash()
		txCopy.Vin[inID].PubKey = nil

		r := big.Int{}
		s := big.Int{}
		sigLen := len(vin.Signature)
		r.SetBytes(vin.Signature[:(sigLen / 2)])
		s.SetBytes(vin.Signature[(sigLen / 2):])

		x := big.Int{}
		y := big.Int{}
		keyLen := len(vin.PubKey)
		x.SetBytes(vin.PubKey[:(keyLen / 2)])
		y.SetBytes(vin.PubKey[(keyLen / 2):])

		rawPubKey := ecdsa.PublicKey{curve, &x, &y}
		if ecdsa.Verify(&rawPubKey, txCopy.ID, &r, &s) == false {
			return false
		}
	}

	return true
}
```

The method is quite straightforward. First, we need the same transaction copy:

```go
txCopy := tx.TrimmedCopy()
```

Next, we’ll need the same curve that is used to generate key pairs:

```go
curve := elliptic.P256()
```

Next, we check signature in each input:

```go
for inID, vin := range tx.Vin {
	prevTx := prevTXs[hex.EncodeToString(vin.Txid)]
	txCopy.Vin[inID].Signature = nil
	txCopy.Vin[inID].PubKey = prevTx.Vout[vin.Vout].PubKeyHash
	txCopy.ID = txCopy.Hash()
	txCopy.Vin[inID].PubKey = nil
```

This piece is identical to the one in the `Sign` method, because during verification we need the same data what was signed.

```go
	r := big.Int{}
	s := big.Int{}
	sigLen := len(vin.Signature)
	r.SetBytes(vin.Signature[:(sigLen / 2)])
	s.SetBytes(vin.Signature[(sigLen / 2):])

	x := big.Int{}
	y := big.Int{}
	keyLen := len(vin.PubKey)
	x.SetBytes(vin.PubKey[:(keyLen / 2)])
	y.SetBytes(vin.PubKey[(keyLen / 2):])
```

Here we unpack values stored in `TXInput.Signature` and `TXInput.PubKey`, since a signature is a pair of numbers and a public key is a pair of coordinates. We concatenated them earlier for storing, and now we need to unpack them to use in `crypto/ecdsa` functions.

```go
	rawPubKey := ecdsa.PublicKey{curve, &x, &y}
	if ecdsa.Verify(&rawPubKey, txCopy.ID, &r, &s) == false {
		return false
	}
}

return true
```

Here it is: we create an `ecdsa.PublicKey` using the public key extracted from the input and execute `ecdsa.Verify` passing the signature extracted from the input. If all inputs are verified, return true; if at least one input fails verification, return false.

Now, we need a function to obtain previous transactions. Since this requires interaction with the blockchain, we’ll make it a method of `Blockchain`:

```go
func (bc *Blockchain) FindTransaction(ID []byte) (Transaction, error) {
	bci := bc.Iterator()

	for {
		block := bci.Next()

		for _, tx := range block.Transactions {
			if bytes.Compare(tx.ID, ID) == 0 {
				return *tx, nil
			}
		}

		if len(block.PrevBlockHash) == 0 {
			break
		}
	}

	return Transaction{}, errors.New("Transaction is not found")
}

func (bc *Blockchain) SignTransaction(tx *Transaction, privKey ecdsa.PrivateKey) {
	prevTXs := make(map[string]Transaction)

	for _, vin := range tx.Vin {
		prevTX, err := bc.FindTransaction(vin.Txid)
		prevTXs[hex.EncodeToString(prevTX.ID)] = prevTX
	}

	tx.Sign(privKey, prevTXs)
}

func (bc *Blockchain) VerifyTransaction(tx *Transaction) bool {
	prevTXs := make(map[string]Transaction)

	for _, vin := range tx.Vin {
		prevTX, err := bc.FindTransaction(vin.Txid)
		prevTXs[hex.EncodeToString(prevTX.ID)] = prevTX
	}

	return tx.Verify(prevTXs)
}
```

These functions are simple: `FindTransaction` finds a transaction by ID (this requires iterating over all the blocks in the blockchain); `SignTransaction` takes a transaction, finds transactions it references, and signs it; `VerifyTransaction` does the same, but verifies the transaction instead.

Now, we need to actually sign and verify transactions. Signing happens in the `NewUTXOTransaction`:

```go
func NewUTXOTransaction(from, to string, amount int, bc *Blockchain) *Transaction {
	...

	tx := Transaction{nil, inputs, outputs}
	tx.ID = tx.Hash()
	bc.SignTransaction(&tx, wallet.PrivateKey)

	return &tx
}
```

Verification happens before a transaction is put into a block:

```go
func (bc *Blockchain) MineBlock(transactions []*Transaction) {
	var lastHash []byte

	for _, tx := range transactions {
		if bc.VerifyTransaction(tx) != true {
			log.Panic("ERROR: Invalid transaction")
		}
	}
	...
}
```

And that’s it! Let’s check everything one more time:

```shell
$ blockchain_go createwallet
Your new address: 1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR

$ blockchain_go createwallet
Your new address: 1NE86r4Esjf53EL7fR86CsfTZpNN42Sfab

$ blockchain_go createblockchain -address 1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR
000000122348da06c19e5c513710340f4c307d884385da948a205655c6a9d008

Done!

$ blockchain_go send -from 1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR -to 1NE86r4Esjf53EL7fR86CsfTZpNN42Sfab -amount 6
0000000f3dbb0ab6d56c4e4b9f7479afe8d5a5dad4d2a8823345a1a16cf3347b

Success!

$ blockchain_go getbalance -address 1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR
Balance of '1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR': 4

$ blockchain_go getbalance -address 1NE86r4Esjf53EL7fR86CsfTZpNN42Sfab
Balance of '1NE86r4Esjf53EL7fR86CsfTZpNN42Sfab': 6
```

Nothing is broken. Awesome!

Let’s also comment out the `bc.SignTransaction(&tx, wallet.PrivateKey)` call in `NewUTXOTransaction` to ensure that unsigned transactions cannot be mined:

```go
func NewUTXOTransaction(from, to string, amount int, bc *Blockchain) *Transaction {
   ...
	tx := Transaction{nil, inputs, outputs}
	tx.ID = tx.Hash()
	// bc.SignTransaction(&tx, wallet.PrivateKey)

	return &tx
}
$ go install
$ blockchain_go send -from 1AmVdDvvQ977oVCpUqz7zAPUEiXKrX5avR -to 1NE86r4Esjf53EL7fR86CsfTZpNN42Sfab -amount 1
2017/09/12 16:28:15 ERROR: Invalid transaction
```

## Conclusion

It’s really awesome that we’ve got so far and implemented so many key features of Bitcoin! We’ve implemented almost everything outside networking, and in the next part, we’ll finish transactions.

Links:

1. [Full source codes](https://github.com/Jeiwan/blockchain_go/tree/part_5)
2. [Public-key cryptography](https://en.wikipedia.org/wiki/Public-key_cryptography)
3. [Digital signatures](https://en.wikipedia.org/wiki/Digital_signature)
4. [Elliptic curve](https://en.wikipedia.org/wiki/Elliptic_curve)
5. [Elliptic curve cryptography](https://en.wikipedia.org/wiki/Elliptic_curve_cryptography)
6. [ECDSA](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
7. [Technical background of Bitcoin addresses](https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses)
8. [Address](https://en.bitcoin.it/wiki/Address)
9. [Base58](https://en.bitcoin.it/wiki/Base58Check_encoding)
10. [A gentle introduction to elliptic curve cryptography](http://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
