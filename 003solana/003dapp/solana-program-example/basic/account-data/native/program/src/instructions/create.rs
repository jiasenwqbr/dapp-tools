/// 导入模块
/// borsh::BorshSerialize：引入 Borsh 序列化库，用于将数据结构序列化为字节数组。
/// solana_program::*：引入 Solana 程序库的各种模块和函数，包括账户信息、程序结果、系统指令等。
/// crate::state::AddressInfo：引入自定义的 AddressInfo 结构体（假定在 state 模块中定义），用于存储特定的地址信息。
///
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::state::AddressInfo;

/// 函数定义
/// program_id：程序的公钥，标识该智能合约在 Solana 上的唯一地址。
/// accounts：与智能合约交互的账户数组。
/// address_info：要存储的地址信息结构体。
/// ProgramResult：函数返回类型，表示程序执行的结果。
///
pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {
    // 账户迭代器
    // 创建一个可变的账户迭代器 accounts_iter。
    // 获取 address_info_account、payer 和 system_program，分别表示存储地址信息的账户、支付创建账户费用的账户、系统程序账户。
    let accounts_iter = &mut accounts.iter();
    let address_info_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    // 计算所需的 lamports
    // account_span：序列化后的 address_info 字节长度。
    // lamports_required：创建账户所需的最小 lamports（Solana 中的基本货币单位），由 Rent 计算得出。
    let account_span = (address_info.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);
    // 调用系统指令创建账户
    // system_instruction::create_account：创建账户的系统指令。
    // payer.key：支付者的公钥。
    // address_info_account.key：新账户的公钥。
    // lamports_required：所需的 lamports。
    // account_span as u64：账户数据的字节长度。
    // program_id：程序 ID，标识该账户由哪个程序管理。
    // invoke：调用系统指令，传递支付者、地址信息账户和系统程序。
    invoke(
        &system_instruction::create_account(
            payer.key,
            address_info_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(),
            address_info_account.clone(),
            system_program.clone(),
        ],
    )?;

    // 序列化并存储数据
    // address_info.serialize：将 address_info 序列化并存储到 address_info_account 的数据字段中。
    // 返回 Ok(()) 表示程序执行成功。

    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;

    Ok(())
}

/*
总结
    这段代码的主要功能是：
    接受程序 ID、公钥数组和地址信息结构体作为输入。
    迭代账户数组，获取相应的账户信息。
    计算存储地址信息所需的 lamports。
    调用系统指令创建账户并支付所需的 lamports。
    将地址信息序列化并存储在新创建的账户中。
    通过这种方式，可以在 Solana 区块链上创建一个存储自定义地址信息的账户，并确保其存储要求得到满足。
*/
