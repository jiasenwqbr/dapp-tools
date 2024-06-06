/// 使用 Anchor 框架定义了一个 Solana 智能合约中的账户创建过程。
/// 具体来说，它定义了一个结构体 CreateAddressInfo 及其初始化过程，
/// 以及一个函数 create_address_info 来创建并初始化一个 AddressInfo 账户。
///
/// 导入模块
/// use crate::state::AddressInfo;：引入在 state 模块中定义的 AddressInfo 结构体。
/// use anchor_lang::prelude::*;：引入 Anchor 框架的预导入模块，提供必要的宏和类型。
use crate::state::AddressInfo;
use anchor_lang::prelude::*;
/// 定义账户结构体
/// #[derive(Accounts)]：这个属性标签告诉 Anchor 这个结构体用于账户验证和初始化。
/// pub struct CreateAddressInfo<'info>：定义一个名为 CreateAddressInfo 的结构体，
/// 生命周期参数 'info 表示该结构体中的引用将持续多久。

/// 字段解释：
/// payer 字段
/// #[account(mut)]：表示该账户可以被修改。
/// payer: Signer<'info>：定义一个名为 payer 的字段，类型为 Signer<'info>，表示这是一个签名者账户。
///
/// address_info 字段
/// #[account(init, payer = payer, space = 8 + AddressInfo::INIT_SPACE)]：
/// 表示这是一个初始化账户，payer 指定了支付账户初始化费用的账户，space 指定了账户需要的字节空间。
/// address_info: Account<'info, AddressInfo>：
/// 定义一个名为 address_info 的字段，类型为 Account<'info, AddressInfo>，表示这是一个 AddressInfo 类型的账户。
///
/// system_program 字段
/// system_program: Program<'info, System>：
/// 定义一个名为 system_program 的字段，类型为 Program<'info, System>，表示这是一个系统程序账户，用于创建和管理账户。
#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + AddressInfo::INIT_SPACE,
    )]
    address_info: Account<'info, AddressInfo>,
    system_program: Program<'info, System>,
}
/// 初始化函数
/// pub fn create_address_info(...) -> Result<()>：定义一个公开的函数 create_address_info，返回类型为 Result<()>。
/// ctx: Context<CreateAddressInfo>：函数参数 ctx 是一个 Context 对象，封装了账户信息和上下文。
/// name: String, house_number: u8, street: String, city: String：函数参数，这些是需要存储在 AddressInfo 账户中的信息。
///
/// 函数体解释
/// 设置账户信息
/// 使用解引用操作 *，将 ctx.accounts.address_info 设置为一个新的 AddressInfo 结构体实例。
/// 通过传入的参数 name、house_number、street 和 city 来初始化 AddressInfo 结构体。
///
/// 返回结果
/// 函数返回Ok(()),表示成功完成操作
pub fn create_address_info(
    ctx: Context<CreateAddressInfo>,
    name: String,
    house_number: u8,
    street: String,
    city: String,
) -> Result<()> {
    *ctx.accounts.address_info = AddressInfo {
        name,
        house_number,
        street,
        city,
    };
    Ok(())
}

/*
总结
这段代码定义了一个 Solana 智能合约中的账户创建过程，使用 Anchor 框架来简化和管理账户的初始化和验证。具体步骤包括：

    定义一个 CreateAddressInfo 结构体，包含需要的账户信息和初始化参数。
    使用 #[account] 属性标签来指定账户的行为（如可变、初始化、支付账户、空间大小等）。
    定义一个 create_address_info 函数，通过 Context 对象访问账户信息，并初始化 AddressInfo 账户。
*/
