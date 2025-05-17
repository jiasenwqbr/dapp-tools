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
