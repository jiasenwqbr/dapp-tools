# Solana trainning

## Blockchain Basics

Welcome to the full end-to-end Solana Blocakchain and Smart Contact Bootcamp.

欢迎来到完整的端到端Solana区块链智能合约训练营。

We're gonna to through everything you need to know from a technical to get started building on the Solana blockchain and write your own full stack decentralized applaications.

我们将介绍你需要了解的所有内容，从技术角度触发，帮助你开始构建在Solana区块链上，并编写你自己完整的去中心化应用程序。

Whether or not you have been in the cryptocurrency space for a while or just getting started,we'll be taking you through your journey to learn how to become an expert Solana engineer.

无论你是否在加密货币领域待过一段时间还是刚刚开始，我们将带你了解如何成为一名专家的Solana工程师。

At the time of recording,Solana developers are in high demand.

在录制时，Solana开发者的需求量很大。

We're seeing a ton of jobs getting posted within the Solana Ecosystem.

我们看到很多职位正在发布在Solana生态系统内。

Over the last three years,Solana developers participating in global hackathons hava created companies that received over \$600 million in funding.And on average,Blockchain Developers right now are making more than \$170,000 a year.

在过去的三年中，参与的Solana开发者在全球黑客马拉松中创建了公司并获得了超过6亿美元的资金。目前，区块链开发者的平均年薪超过了17万美元。

While we cannot guarantee you a job,we're gonna try our best to provide you with the best skills you need to become a Solana developer.

虽然我们无法保证你获得工作，我们将尽力提供给你成为一名Solana开发者所需的技能。

Wether or not you hava any blockchain experience,or hava been building for while,if you are interested in blockchain,this is the course for you.

无论你是否有区块链经验，还是已经构建了一段时间，如果你对区块链感兴趣，这就是适合你的课程。

We'll be starting with blockchain basics,then going into full end-to-end projects,and finally finishing up with what it takes to get your new appliaction to production.

我们将从区块链基础知识开始，然后进入完整的端到端的项目，最后启动新的应用所需的将你的新应用投入生产。

If you hava already built some smart contracts or familiar with blockchain,feel free to jump around the bootcamp to find which projects best fit for your needs.

如果你已经构建了一些智能合约或熟悉区块链，请随意在训练营中跳跃找出适合你需求的项目。

You'll be able to find all the information you need to know about each project in the description,as well as in the GitHub respository link below.

你将能够找到你需要了解的所有信息在每个项目的描述中，以及下面的GitHub存储库链接中。

You'll find the entire bootcamp in this GitHub respository,including any possible resources you may need,such as links to documentation or further reading on topics.

你将在此GitHub库中找到整个训练营，包括你可能需要的任何资源，例如文档或主题进一步阅读的链接。

Throughout this bootcamp,we'll be learning how to build on a blockchain and be using both Rust and TypeScript to accomplish this.

在整个训练营中，我们将学习如何在区块链上构建，并使用Rust和TypeScript来实现这一点。

Don't worry you don't know these languages as we'll be teaching you enough to get started  thoughout each lesson.

如果你不懂这些语言，请不要担心，因为我们会教你足够的知识以便开始在每节课中。

As long as you have some software development background,you shoud do well.

只要你有一些软件开发背景，你应该会表现出色。

Even the best developers don't  get it right the frist time.

即使是最好的开发者也不会第一次就作对。

If you do happen to run into issues at any point in this bootcamp,there's a Solana specific stack exchange where you can get help.

如果在任何阶段遇到问题，在这个训练营中，你可以访问一个Solana专属的问答社区，你可以在这里寻求帮助。

The Solana Fundation developer relatins team and other developers from the ecosystem will be answering your question there.

Solana基金会的开发关系团队以及生态系统中的其他开发者将会回答你的问题。

Just make sure you search for your question first and provide enough information about your error you've received while buiding each project.

请确保首先搜索你的问题，并提供足够的信息关于你收到的错误，在构建每个项目时。

We will also post the full solution at eht end of each step and in the notes below.

我还将在每个步骤后发布完整的解决方案以及以下笔记中。

It is incredibly important that you pace yourself while going through this Bootcamp.

你一定要注意节奏在参加这个训练营时。 

It is a marathon,not a aprint.

这是一场马拉松，而不是短跑。

Take breaks ao you can have some time to digest what you have just learned.Go back different project as a refresher,take the course at your own space.My name is Jacob Creech,and I'll be working with you of some of these projects thoughout this bootcamp.

休息一下，这样才能有时间消化你刚刚学到的内容。回顾不同的项目作为复习，以你自己的节奏上课。我的名字叫Jacob Creech,我将和你一起工作在整个训练营中参与其中的一些项目。

I'm Brianna Miglicccio,and I'll be leading you through how build smart contract,and Decentrolized applications.

我是Brianna Migliccio,我将引导你了解如何构建智能合约和去中心化的应用。

And I'm Mike MacCana.I'll be working with you thoughout the bootcamp on key blockchain concepts.

我是Mike MacCana。我将在整个训练营中与你一起讨论区块链的核心概念。

Before you start building,you shoud understand the purpose of blockchain and the problems that they solve.Fully understand the value blockchain provides will help you leverage it to solve the right problems.So,let's get started.

在你开始构建之前，你应该了解区块链的目的及其解决的问题。充分理解区块链带来的价值将帮助你充分利用它解决正确的问题。那么让我们开始吧。

So what is blockchain?Why is it important and what can you do with it?

那么什么是区块链呢？他为什么重要，你可以利用它做什么？

As its core,a blockchain is a distributed database with them special characteristics.Unlike most databases,blockchains are unique in that they have no central authority,are fully open for anyone to use,and all data is complately transparent.Everyone can make payments on the blockchain while no one entity is capable of stopping or reversing those payments.This is what we call censorship resistant.

从本质上讲，区块链是一种分布式数据库具有一些特殊特性。与大多数数据库不同，区块链具有独特性因为它们没有中央权威机构，对任何人完全开放使用，所有的数据都是完全透明的。每个人都可以在区块链上进行支付而没有任何实体能够停止或撤销这些支付。这就是我们所说的抗审查。

The transactions are irreversible and the blockchain itself is a recording of those transactions.That's why blockchains are often referred to as a ledger.Unlike databases which are typically stored in a central location,data on blockchains are stored on mutiple computers across the world.Essentially,there's no single copy of the blockchain that can be lost or tempered with.

交易是不可逆的，区块链本身是这些交易记录。这就是区块链常被称为账本的原因。与通常存储的数据库不同在一个中心位置，区块链上的数据被存储在全球多个计算机上。本质上，区块链没有单一的副本可以丢失或被篡改。

Instead,the data is stored in mutiple copies and verified by checking the blocks or bundles of transactions.This is called decentralization.

相反，数据以多个副本存储并通过检查区块或交易捆绑进行验证。这被称为去中心化。

What this means in practice is that blockchain allows people to transact with each other directly.

这在实践中的意义是区块链允许人们直接进行交易。

Think about the last time you paid for something,or swapped currencies or just stored your money somewhere.You might have used a credit card,or a money transfer service or a bank.To facilitate that,there's typically a service fee involved.The seller pays a couple of percent on every credit card transaction.You may pay a little bit more for an international transfer.Annual bank probably has a monthly fee just hold your money.

想想上次支付某物的时间，或者兑换货币，或者只是把钱存放在某处。你可能使用了信用卡，或者是汇款服务或银行。为了促进这一点，通常会涉及服务费。卖方在每笔信用卡交易中支付几个百分点在每笔信用卡交易中。国际转账你需要多支付一点。年费银行可能会有月费仅仅为了保管你的钱。

Blockchain allows people to transact with each other directly without needing somebody in between.Now,to understand blockchain,you need start with Bitcoin.Back in 2008,Bitcoin was created by the pseudonym,Satoshi Nakamoto as a way to make digital peer-to-peer payments.The white paper for Bitcoin outlined a way to make those payments without having a trusted third party such as a bank or other financial institution.Blockchain uses digital signatures to let people transact without having to fully trust each other.

区块链允许人们直接进行交易而不需要中间人。现在，要理解区块链，你需要从比特币开始。在2008年，比特币被创建由化名中本聪创建，作为一种进行数字点对点支付的方式。比特币的白皮书概述了一种方法进行这些支付，而不需可信的第三方如银行或其他金融机构。区块链使用数字签名让人们进行交易而不必完全信任彼此。

Digital signatures give us confidence that the money sent is in the correct currency,that the amount is correct and the amount was actually sent.

数字签名让我们确信发送的资金事宜正确的货币发送，金额是正确的并且金额确实被发送。

Bitcoin was created more than 16 years ago,and these days is less used for day-to-day transactions.People think of it more as digital gold.However,plenty of new innovations  have built since then。Modern blockchains like the one that we'll be building on,Solana support fast payments with lower fees than both Bitcoin and traditional finance.

比特币在16年前创建，它在日常交易中的使用减少。人们更把它视为数字黄金。然而，自那以来，许多创新被构建出来。现在的区块链，如我们将要构建的区块链，Solana支持快速支付，费用低于比特币和传统金融。

Not only that,but new creations like Smart Contracts have enabled a whole new set of capabilities previously not available to blockchains.Smart Contracts are computer programs that run on blockchain.Instead of having a cloud provider run your program,you run the program in the blockchain.Smart Contracts are a little bit like APIs in the traditional tech world，except rather than  being invoked by requests,they are invoked by instructions.And rather than sending responses back,they write their changes out to the blockchain where anyone can read.

不仅如此，还有像智能合约这样的新创作使得一套新的能力得以实现以前区块链无法实现。智能合约是运行在区块链上的程序在区块链上运行。而不是让云服务商运行你的程序，你在区块链上运行程序。智能合约有点像传统技术世界中的API，在传统的技术世界中，只是它们不是通过请求调用的，而是通过指令调用。而不是发送响应过来，他们将更改写入区块链，任何人都可以读取。

Smart Contract extend blockchains beyond simple payments to allow complex transactions that could lend money without a bank,allow people to exchange items directly with each other at a price they both agree on,or even create a lottery without a lottery ticket issuer.

智能合约将扩展到简单支付之外以允许复杂的交易，可以在没有银行的情况下借钱,允许人们交换物品以他们双方同意的价格，甚至可以创建一个没有lottery发行者的抽奖。

We'll going to build these in this bootcamp,so you can see how powerful Smart Contracts can be for youself.

我们将在这个训练营中构建这些，以便你可以看到智能合约对你来说有多强大。

Blockchain would not be possible without cryptography.Everyone that uses a blockchain has something called a key pair.A key pair consists of two keys,a private key and a public key.The public key is shown to anyone.It is used as an address people can use to transact whit you.The private key you must keep secret.The privarte key is uesed to sign transactions,which proves that you made them.For example,wanna spend some of your tokens?You need to sign the transaction using your private key.Anyone can use your public key to verify that you as the holder of the matching private key made that transaction.Key pairs are pretty common in cryptography,even outside blockchain.Let's use an example of passports.Your electronic passport is signed by your government using government's private key.When you pass the through the passport gates of the airport,they use your government's public key to prove your passpot is really signed by your goverment.If the passport signature is correct,it must have been signed with your government's private key.If I make my own passwort,it won't be singed by my government's private key,so I won't be able to get through the gates.

没有密码学，区块链是不可能的。每个使用区块链的人都有一个叫做密钥对的东西。密钥对由两个秘钥组成，一个公钥一个私钥。公钥是向任何人展示的。它用作人们可以用来与你交易的地址。私钥必须保密。私钥用于签署交易，这证明你进行的交易。例如，想花一些你的token?你需要使用私钥签署交易。其他任何人都可以使用你的公钥来验证你作为持有者拥有匹配私钥的人进行了交易。密钥对在秘密学中非常常见，即使在区块链之外。让我们以护照作为例子。你的电子护照由你的政府使用其私钥签署的。当你通过机场的护照闸口时，他们使用你政府的公钥来证明你的护照是由你政府签署的。如果护照签名是正确的，它必须是用你政府的私钥签署的。如果我自己制作护照，它不会签署有我政府的私钥签署，所以我无法通过闸口。

In solana,everyone has a public key that uses their address.other people can use your address to transact with you.The most basic solana wallet is just an address with a small balance of SOL.Users can pay their transaction fees and intract with smart contracts using SOL.

在Solana中，每个人都有一个公钥使用他们的地址。其他人可以用你的地址与你交易。最基本的Solana钱包只是一个地址其余额为少量的SOL.用户可以使用SOL支付交易费用并与智能合约进行交互。

Here's Alice's wallet,here's Bob's wallet.If Alice wanted to send Bob some SOL,She wound use her private key to sign a transaction that uses the system program's transfer function to send some SOL to Bob and uses the memo programs memo function to write a note to Bob about the transaction.If both these steps called instructions are completed successfully,the transaction completes successfully.And Bob receives the tokens in his address.If something goes wrong,that's okey.The transaction fails and nothing changes,Alice still have her SOL.

这是爱丽丝的钱包，这是鲍勃的钱包。如果爱丽丝想发送给鲍勃一些SOL，它将使用她的私钥来签署一笔交易，该交易将使用系统程序的转账功能将一些SOL发送给鲍勃。并使用程序的备忘录功能写一条说明告诉鲍勃关于交易的详情。如果这两个被称为指令的步骤都成功完成了，该交易将成功完成。鲍勃将在他的地址中收到token。如果出现问题，那也没关系。交易会失败，而且不会改变任何事情，爱丽丝仍然会拥有她的SOL。

Blockchain technology allows for new applications to be built in this digital age.We can create systems where people transact with each other directly make digital assets that can be sold or used outside of a game,allow art to be sold and ensure artists get paid each time a piece of artwork In transfered.

区块链技术使得在这个数字时代可以创新应用。我们可以创建一个系统，让人们直接交易，彼此交易并制作数字资产，这些资产可以在游戏外进行出售或使用，允许艺术品被出售，并确保每次作品转让时艺术家都能获得报酬。

The projects we'll be building throughout this course will use solana blockchain to create unique applications that could only be built with smart contracts.Let's get started with your first application.

在本课程我们将构建的项目将使用Solana区块链创建独特的应用程序,这些应用程序只能通过智能合约构建。让我们开始你的第一个应用吧。

## Project 1:Favorites Program

The first smart contract we make will be simple.It's gonna save our favorite thing to the blockchain.We're gonna learn how we can save,update,and retrieve information form the blockchian and how signing in used to control access in our smart contract.

我们构建的第一个智能合约将会很简单。它将把我们最喜欢的东西保存到区块链上。我们将学习如何保存、更新，并从区块链检索信息，以及如何使用签名来控制我们智能合约中的访问。

In Solana,your smart contracts or programs can store additional data in what's called Program Derived Addresses.PDA address are not made from a public key,instead they are made from seeds,which can be whatever you the programmer want.Wanna store some config for your smart contracts?You can make a PDA from the seed config as a string.When you need to find your program config,look for the PDA made from that seed and you'll find your config.

在Solana中，你的智能合约或程序可以存储额外的数据在所谓的程序派生地址（PDA）中。PDA地址不是由公钥创建的，而是由种子创建的，可以是你作为程序员所希望的任何东西。想为你的智能合约存储一些配置吗？你可以使用种子配置作为字符串创建PDA。当你需要找到程序配置时，查找由该种子创建的PDA你就能找到你的配置。

Solana PDAs are a type of key value store.If you haven't used a key value store before,you can think of a PDA as role of data,but the seeds being the primary key used to find the data.So let's build a simple app that saves someone's favorite thanks to the blockchain and uses digital signatures to ensure only the wallet holder can update their own favorites.We'll use Solana Playground,and Anchor,most popular framework for making Solana programs.

Solana的PDA是一种键值存储。如果你之前没有使用过键值存储，你可以将PDA视为数据的一组，但种子是用于查找数据的主键。那么让我们构建一个简单的应用程序，将某人的最爱保存早区块链上并使用数字签名确保只有钱包持有者可以更新他们自己的爱好。我们将使用Solana Playground和Anchor，这是创建Solana程序最受欢迎的框架。

So we're gonna be using a tool called Solana Playground,which lets us make Solana programs or smart contracts driectly in our web browser without installing anything on our local machine.So open beta.solpg.io,click create a new project.Give your project a name.I'm gonna call this project favorites because it's gonna svae our favorite things to the blockchain and we'll pick the Anchor option

因此我们将使用一个名为Solana Playground的工具，它允许我们在网页中直接创建Solana程序，而无需在本地计算机上安装任何东西。请访问beta.solpg.io,点击创建新项目。给你的项目命名。我将把这个项目成为favorites，因为他将我们最喜欢的东西保存到区块链上，我们将选择Anchor选项

```rust
use anchor_lang::prelude::*;
// Our program's address!
// This matches the key in the target/deploy directory
declare_id!("4bG5JRotXdYMfb3WY43F6n4fKa6JwLk66aKjD99rjtNd");

// Anchor programs always use 8 bits for the discriminator
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// Our Solana program!
#[program]
pub mod favorites {
    use super::*;

    // Our instruction handler! It sets the user's favorite number and color
    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();
        msg!("Greetings from {}", context.program_id);
        msg!("User {user_public_key}'s favorite number is {number}, favorite color is: {color}",);

        msg!("User's hobbies are: {:?}", hobbies);

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });
        Ok(())
    }

    // We can also add a get_favorites instruction handler to return the user's favorite number and color
}

// What we will put inside the Favorites PDA
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}
// When people call the set_favorites instruction, they will need to provide the accounts that will be modifed. This keeps Solana fast!
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds=[b"favorites", user.key().as_ref()],
    bump)]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

```















































































