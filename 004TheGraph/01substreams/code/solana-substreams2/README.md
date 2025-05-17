# solana-dex-with-sol-events
Stream Solana Dex events and SOL Transfer events with substreams


## Attension 

To meet my new Solana data requirements, I have expanded it and added some features. It now includes Sol transfer events and pumpfun events.

**This is due to my specific needs and it is not universal. Please be cautious when using it.**

## Usage

- First step into the project raydium_amm folder
```bash
cd raydium_amm
```

- Second step, run the substreams(After You install substreams scripts and config the system env)

```bash
substreams run substreams.yaml raydium_amm_events -e mainnet.sol.streamingfast.io:443 -s {your_start_block_number} -t +1
```

If you see no output, please check that you have set a starting block, e.g. `substreams run substreams.yaml raydium_amm_events -e mainnet.sol.streamingfast.io:443 -s 325766951 -t +1`.

## Suported Events 

### Basic

- InitializeEvent: Raydium AMM initialization event
- DepositEvent：Raydium AMM deposit event
- WithdrawEvent:  Raydium AMM withdraw event
- WithdrawPnlEvent: Raydium AMM withdraw PNL event
- SwapEvent: Raydium AMM swap event

### Features

- TransferEvent: SOL Transfer event filter the lamports at least 100000, and filter the dex(Raydium\ Pumpfun \ Jupiter) 
- TransferWithSeedEvent: SOL Transfer with Seed event filter the lamports at least 100000, and filter the dex(Raydium\ Pumpfun \ Jupiter) 
- PumpfunSwapEvent: Pumpfun swap event（buy or sell）
- PumpfunWithdrawEvent: Pumpfun withdraw event
- PumpfunCreateEvent: Pumpfun create event
    
For more information, refer to the [protobuf specification](raydium_amm/proto/raydium_amm.proto).


## data structure

### Pumpfun 协议事件数据结构详解
#### 1. 整体结构概述
这是一个用于记录 Pumpfun 协议事件的嵌套数据结构，采用 Protocol Buffers (protobuf) 格式，包含以下主要组成部分：

区块级事件容器 (PumpfunBlockEvents)

交易级事件容器 (PumpfunTransactionEvents)

通用事件包装器 (PumpfunEvent)

具体事件类型 (5种不同事件)

#### 2. 核心数据结构解析
##### 2.1 PumpfunBlockEvents (区块事件)
```rust
pub struct PumpfunBlockEvents {
    pub transactions: Vec<PumpfunTransactionEvents>, // 区块内所有交易事件
}
```

功能：承载一个区块内所有 Pumpfun 相关交易事件

字段：

transactions: 交易事件数组，每个元素代表一笔交易

##### 2.2 PumpfunTransactionEvents (交易事件)
```rust
pub struct PumpfunTransactionEvents {
    pub signature: String,           // 交易签名
    pub events: Vec<PumpfunEvent>,   // 交易内的事件列表
}
```
- 功能：记录单笔交易中的 Pumpfun 事件

- 字段：

signature: 交易唯一标识

events: 该交易触发的 Pumpfun 事件数组

##### 2.3 PumpfunEvent (通用事件包装器)
```rust
pub struct PumpfunEvent {
    pub event: Option<pumpfun_event::Event>, // 具体事件类型
}

pub mod pumpfun_event {
    pub enum Event {
        Initialize(InitializeEvent),     // 初始化事件
        SetParams(SetParamsEvent),       // 参数设置事件
        PumpfunSwap(PumpfunSwapEvent),    // 代币交换事件
        PumpfunWithdraw(PumpfunWithdrawEvent), // 提取事件
        PumpfunCreate(PumpfunCreateEvent), // 创建事件
    }
}
```
- 功能：使用 oneof 结构封装具体事件类型

- 设计特点：

采用 Rust 的 Option 和 Oneof 实现 protobuf 的 oneof 特性

通过 tag 编号区分不同事件类型

#### 3. 具体事件类型详解
##### 3.1 PumpfunCreateEvent (代币创建事件)
```rust
pub struct PumpfunCreateEvent {
    pub user: String,                      // 创建者地址
    pub name: String,                      // 代币名称
    pub symbol: String,                    // 代币符号
    pub uri: String,                       // 元数据URI
    pub mint: String,                      // 代币mint地址
    pub bonding_curve: String,             // 绑定曲线地址
    pub associated_bonding_curve: String,  // 关联绑定曲线
    pub metadata: String,                  // 元数据地址
}
```
- 触发时机：用户创建新代币时
- 关键字段：
bonding_curve: 代币的价格曲线配置
metadata: 代币的链上元数据
##### 3.2 InitializeEvent (初始化事件)
```rust
pub struct InitializeEvent {
    pub user: String,  // 初始化用户地址
}
```
- 触发时机：用户首次与协议交互时
- 简化设计：仅记录用户地址

##### 3.3 SetParamsEvent (参数设置事件)
```rust
pub struct SetParamsEvent {
    pub user: String,                     // 设置者地址
    pub fee_recipient: String,            // 手续费接收地址
    pub initial_virtual_token_reserves: u64, // 初始虚拟代币储备
    pub initial_virtual_sol_reserves: u64,  // 初始虚拟SOL储备
    pub initial_real_token_reserves: u64,   // 初始真实代币储备
    pub token_total_supply: u64,           // 代币总供应量
    pub fee_basis_points: u64,             // 手续费率(基点)
}
```

- 触发时机：设置代币交易参数时
- 经济参数：
  - 虚拟/真实储备金机制
  - 手续费配置

##### 3.4 PumpfunSwapEvent (代币交换事件)
```rust
pub struct PumpfunSwapEvent {
    pub user: String,                     // 交易用户
    pub mint: String,                     // 代币mint地址
    pub bonding_curve: String,            // 绑定曲线地址
    pub sol_amount: Option<u64>,          // SOL交易量
    pub token_amount: u64,                // 代币交易量
    pub direction: String,                // 交易方向(buy/sell)
    pub virtual_sol_reserves: Option<u64>, // 虚拟SOL储备
    pub virtual_token_reserves: Option<u64>, // 虚拟代币储备
    pub real_sol_reserves: Option<u64>,    // 真实SOL储备
    pub real_token_reserves: Option<u64>,  // 真实代币储备
    pub user_token_pre_balance: Option<u64>, // 用户交易前余额
}
```
- 触发时机：用户进行代币买卖时

- 核心字段：

    - direction: 区分买入/卖出

    - 多层级储备金数据

- 设计特点：

    - 使用 Option 表示可选字段

    - 完整记录交易前后状态

##### 3.5 PumpfunWithdrawEvent (提取事件)

```rust
pub struct PumpfunWithdrawEvent {
    pub mint: String,  // 代币mint地址
}
```
- 触发时机：从资金池提取流动性时

- 简化设计：仅记录相关代币

#### 4. 技术实现细节
- 序列化方案：

    - 使用 prost 库实现 protobuf 序列化

    - 消息字段采用 tag 编号而非字段名

- 内存管理：

    - 字符串使用 prost::alloc::string::String

    - 数值类型直接使用原生类型(u64等)

- 枚举处理：

    - oneof 结构转换为 Rust 枚举

    - 每个变体对应一种事件类型

- 可选字段：

    - 使用 Option 包装 protobuf 的 optional 字段

    - 如 sol_amount 在卖出交易时可能为 None

#### 5. 典型使用场景
```rust
// 示例：处理区块事件
fn process_block_events(events: PumpfunBlockEvents) {
    for tx in events.transactions {
        println!("Processing TX: {}", tx.signature);
        
        for event in tx.events {
            match event.event {
                Some(pumpfun_event::Event::PumpfunSwap(swap)) => {
                    println!("Swap event: {} {} tokens", 
                        swap.direction, swap.token_amount);
                },
                Some(pumpfun_event::Event::PumpfunCreate(create)) => {
                    println!("New token created: {}", create.symbol);
                },
                _ => {} // 其他事件类型
            }
        }
    }
}
```
### Raydium AMM 事件数据结构
#### 1. 整体架构
该数据结构是用于记录 Raydium 自动做市商(AMM)协议事件的 Protocol Buffers 定义，采用分层设计：

- 区块层 (RaydiumAmmBlockEvents)：包含区块内所有交易事件

- 交易层 (RaydiumAmmTransactionEvents)：单笔交易中的事件集合

- 事件层 (RaydiumAmmEvent)：具体事件类型的容器

- 具体事件 (10种不同类型的事件)

```
RaydiumAmmBlockEvents (区块层)
└── RaydiumAmmTransactionEvents (交易层)
    └── RaydiumAmmEvent (事件层)
        ├── InitializeEvent
        ├── DepositEvent
        ├── WithdrawEvent
        ├── ...
        └── PumpfunCreateEvent
```

#### 2. 核心结构解析
##### 2.1 区块层 (RaydiumAmmBlockEvents)
```rust
pub struct RaydiumAmmBlockEvents {
    pub transactions: Vec<RaydiumAmmTransactionEvents>, // 交易事件数组
}
```
- 功能：按区块组织交易数据

- 设计特点：

    - 使用Vec存储交易，保持原始顺序

    - 对应protobuf的repeated字段

##### 交易层 (RaydiumAmmTransactionEvents)
```rust
pub struct RaydiumAmmTransactionEvents {
    pub signature: String,       // 交易签名(Base58)
    pub events: Vec<RaydiumAmmEvent>, // 事件列表
    pub block_time: String,      // ISO8601时间格式
    pub transaction_index: String // 区块内索引
}
```
- 功能：记录单笔交易触发的所有AMM事件

- 关键字段：

    - signature：交易唯一标识符

    - block_time：ISO格式时间戳(如"2023-05-01T12:34:56Z")

##### 2.3  事件层 (RaydiumAmmEvent)
```rust
pub enum Event {
    Initialize(InitializeEvent),     // 池初始化
    Deposit(DepositEvent),          // 流动性存入
    Withdraw(WithdrawEvent),        // 流动性提取
    WithdrawPnl(WithdrawPnlEvent),  // 收益提取
    Swap(SwapEvent),                // 代币交换
    // ...其他6种事件类型...
}
```
- 内存布局
```
graph LR
  A[RaydiumAmmEvent] --> B[InitializeEvent]
  A --> C[DepositEvent]
  A --> D[...]
```
- 设计优势：使用oneof节省内存，同一时间只有一种事件类型被激活
#### 3. 关键事件类型分析
##### 3.1 InitializeEvent (流动性池初始化)
```rust
pub struct InitializeEvent {
    pub amm: String,             // AMM合约地址
    pub pc_init_amount: u64,     // 初始报价币数量(如USDC)
    pub coin_init_amount: u64,   // 初始基础币数量(如SOL)
    pub nonce: u32,              // 派生账户的非ce值
    pub market: Option<String>,  // 关联Serum市场
    // ...其他代币合约地址...
}
```
- 业务逻辑：创建新的交易对池

- 关键参数：

    - nonce：用于PDAs(Program Derived Addresses)派生

    - 初始比例决定开盘价格

##### 3.2 SwapEvent (代币交换)
```rust
pub struct SwapEvent {
    pub mint_in: String,         // 输入代币类型
    pub amount_in: u64,          // 输入数量
    pub amount_out: u64,         // 输出数量
    pub direction: String,       // "buy"或"sell"
    pub pool_pc_amount: Option<u64>, // 池报价币储备
    // ...其他池状态字段...
}
```
- 价格计算
```rust
let price = match swap.direction.as_str() {
    "buy" => swap.amount_in as f64 / swap.amount_out as f64,
    _ => swap.amount_out as f64 / swap.amount_in as f64
};
```
- 滑点监控
```rust
pub struct DepositEvent {
    pub pc_amount: u64,          // 存入的报价币
    pub coin_amount: u64,        // 存入的基础币
    pub lp_amount: u64,          // 获得的LP代币
    pub pool_pc_amount: Option<u64>, // 池状态
    // ...
}
```

- 经济模型：

    - 存款：(pcAmount, coinAmount) → lpAmount

    - 取款：lpAmount → (pcAmount, coinAmount)

#### 4. 特殊结构设计
##### 4.1 账户余额追踪
```rust
pub struct AccountBalance {
    pub pre_balance: u64,    // 操作前余额
    pub post_balance: u64    // 操作后余额
}
```
- 审计功能：完整记录状态变化

- 典型应用：在TransferEvent中记录SOL转账前后余额

##### 4.2 可选字段设计
```rust
pub pool_pc_amount: Option<u64> 
```
设计考量：

- 平衡数据完整性和存储效率

- 非关键字段使用Option减少存储开销









