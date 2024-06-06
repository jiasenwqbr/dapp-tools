#![allow(clippy::result_large_err)]
/// 一个完整的 Solana 智能合约（也称为程序），使用了 Anchor 框架来简化合约开发。它包括模块的声明、入口函数和主要功能的实现。
/// #![allow(clippy::result_large_err)]：这是一条编译器指令，
/// 告诉编译器忽略 clippy（Rust 的 lint 工具）关于返回大错误类型的警告。这通常用于允许较大的错误类型通过编译检查。
///
/// 导入模块
// use anchor_lang::prelude::*;：引入 Anchor 框架的预导入模块，提供必要的宏和类型。
// use instructions::*;：导入 instructions 模块中的所有内容。
// pub mod instructions;：声明一个名为 instructions 的公共模块。
// pub mod state;：声明一个名为 state 的公共模块。
use anchor_lang::prelude::*;
use instructions::*;
pub mod instructions;
pub mod state;
/// 声明程序ID
// declare_id!("hB93vXZbZQ42Zczn2joFoVtmVw3ZG87wLrVDtCkiTYP");：
// 声明该程序的唯一标识符（Program ID）。这个 ID 用于在区块链上唯一标识该程序。
declare_id!("hB93vXZbZQ42Zczn2joFoVtmVw3ZG87wLrVDtCkiTYP");
/// 定义程序模块
// #[program]：这个属性标签告诉 Anchor 这是一个程序模块。
#[program]
pub mod account_data_anchor_program_example {
    use instructions::create;

    use super::*;

    pub fn anchor_program_example(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        city: String,
        street: String,
    ) -> Result<()> {
        create::create_address_info(ctx, name, house_number, street, city)
    }
}

/*
这段代码定义了一个 Solana 智能合约，使用 Anchor 框架来简化合约的开发和管理。以下是每个部分的具体作用：

    编译器指令：允许较大的错误类型通过编译检查。
    模块导入：引入了必要的模块和类型，声明了两个公共模块 instructions 和 state。
    程序 ID 声明：指定该智能合约在区块链上的唯一标识符。
    程序模块定义：定义了一个名为 account_data_anchor_program_example 的程序模块，并包含一个名为 anchor_program_example 的函数。
    函数实现：anchor_program_example 函数调用 create 模块中的 create_address_info 函数，创建并初始化 AddressInfo 账户。
*/
