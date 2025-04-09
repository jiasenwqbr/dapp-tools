# legacy-bootcamp-temp

## Getting Started

### Prerequisites

- Node v18.18.0 or higher

- Rust v1.77.2 or higher
- Anchor CLI 0.30.1 or higher
- Solana CLI 1.18.17 or higher

### Installation

#### Clone the repo

```shell
git clone <repo-url>
cd <repo-name>
```

#### Install Dependencies

```shell
pnpm install
```

#### Start the web app

```
pnpm dev
```

## Apps

### anchor

This is a Solana program written in Rust using the Anchor framework.

#### Commands

You can use any normal anchor commands. Either move to the `anchor` directory and run the `anchor` command or prefix the
command with `pnpm`, eg: `pnpm anchor`.

#### Sync the program id:

Running this command will create a new keypair in the `anchor/target/deploy` directory and save the address to the
Anchor config file and update the `declare_id!` macro in the `./src/lib.rs` file of the program.

You will manually need to update the constant in `anchor/lib/counter-exports.ts` to match the new program id.

```shell
pnpm anchor keys sync
```

#### Build the program:

```shell
pnpm anchor-build
```

#### Start the test validator with the program deployed:

```shell
pnpm anchor-localnet
```

#### Run the tests

```shell
pnpm anchor-test
```

#### Deploy to Devnet

```shell
pnpm anchor deploy --provider.cluster devnet
```

### web

This is a React app that uses the Anchor generated client to interact with the Solana program.

#### Commands

Start the web app

```shell
pnpm dev
```

Build the web app

```shell
pnpm build
```



## 详解以下代码：
```rust
use anchor_lang::prelude::*;

declare_id!("5s3PtT8kLYCv1WEp6dSh3T7EuF35Z6jSu5Cvx4hWG79H");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>, 
                            _poll_id: u64, 
                            start_time: u64, 
                            end_time: u64,
                            name: String,
                            description: String) -> Result<()> {
        ctx.accounts.poll_account.poll_name = name;
        ctx.accounts.poll_account.poll_description = description;
        ctx.accounts.poll_account.poll_voting_start = start_time;
        ctx.accounts.poll_account.poll_voting_end = end_time;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitializeCandidate>, 
                                _poll_id: u64, 
                                candidate: String) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = candidate;
        ctx.accounts.poll_account.poll_option_index += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        let candidate_account = &mut ctx.accounts.candidate_account;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time > (ctx.accounts.poll_account.poll_voting_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.poll_voting_start as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate_account.candidate_votes += 1;

        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount{
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}

```


这段代码是一个使用 Anchor 框架编写的 Solana 上的投票系统智能合约，它定义了三个主要操作：创建投票、添加候选人、投票。下面我们逐步详细解释每个部分。

顶部定一部分
```rust
use anchor_lang::prelude::*;

declare_id!("5s3PtT8kLYCv1WEp6dSh3T7EuF35Z6jSu5Cvx4hWG79H");

```
引入 Anchor 框架的核心预导入模块。

declare_id! 宏是定义当前程序（合约）的唯一 Program ID，在部署后必须与部署地址一致。

 #[program] 模块：定义核心指令逻辑
✅ initialize_poll

```rust
pub fn initialize_poll(...) -> Result<()> { ... }

```

用途：初始化一个投票（Poll）。

传入参数：

poll_id: 用于种子生成 PDA

start_time, end_time: 投票起止时间

name, description: 投票标题和描述

操作：设置 poll_account 里对应的字段。


✅ initialize_candidate

```rust
pub fn initialize_candidate(...) -> Result<()> { ... }

```
用途：为某个投票添加候选人。

逻辑：

新建一个 candidate_account（PDA）

设置候选人名字

poll_account.poll_option_index += 1 表示候选人数量增加


✅ vote

```rust
pub fn vote(...) -> Result<()> { ... }

```
用途：投票操作。

逻辑：

读取系统时间 Clock::get()?.unix_timestamp

校验当前时间是否在合法的投票时间段内

如果在时间范围内，对候选人计数 candidate_votes += 1


📦 #[derive(Accounts)]：账户校验结构体
✅ InitializePoll
要求：

调用者是 signer（付费者）

初始化 poll_account（通过 seed + bump）

使用 poll_id 作为种子的一部分


✅ InitializeCandidate
要求：

依赖于已经存在的 poll_account

初始化 candidate_account，用 poll_id + candidate name 作为 PDA 的种子

✅ Vote
需要访问两个可变账户：

poll_account（验证投票时间）

candidate_account（增加投票数）

🗃 账户结构定义
✅ PollAccount

```rust
#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    pub poll_name: String,         // 最多 32 字符
    pub poll_description: String,  // 最多 280 字符
    pub poll_voting_start: u64,    // 开始时间（Unix timestamp）
    pub poll_voting_end: u64,      // 结束时间
    pub poll_option_index: u64,    // 候选人数量计数器
}

```
注：#[max_len(n)] 与 InitSpace 会自动为 Anchor 计算存储空间

✅ CandidateAccount

```rust
#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    pub candidate_name: String,  // 最多 32 字符
    pub candidate_votes: u64,    // 投票计数
}

```

⚠️ 错误定义
```rust
#[error_code]
pub enum ErrorCode {
    VotingNotStarted,
    VotingEnded,
}


```
用自定义错误提高可读性并简化错误返回。


📊 程序流程图示意

```lua

initialize_poll --> 创建投票账户（包含名称、描述、时间等）
   |
   └──> initialize_candidate --> 添加候选人（关联到投票）
         |
         └──> vote --> 通过时间窗口校验后增加票数

```
✅ 总结

操作	   功能	  输入	  输出
initialize_poll	创建投票	poll_id, 时间, 名称等	PollAccount PDA

initialize_candidate	添加候选人	poll_id, candidate	CandidateAccount PDA

vote	给某候选人投票	poll_id, candidate	增加计数

如果你想继续深入：

支持一个人只能投一次（加 voter list）

添加得票排行

发 NFT 奖励获胜者

我都可以帮你拓展，要不要来一个升级版本？

