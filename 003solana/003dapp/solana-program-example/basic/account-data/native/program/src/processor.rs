///
/// 实现了一个 Solana 智能合约的主要入口函数，用于处理传入的指令。它依赖于 Borsh 进行序列化和反序列化，并处理具体的业务逻辑。
/// 导入模块
///
/// borsh::BorshDeserialize：引入 Borsh 序列化库，用于反序列化数据。
/// solana_program::*：引入 Solana 程序库的各种模块和函数，包括账户信息、程序结果、程序错误、和公钥等。
/// crate::{instructions, state::AddressInfo}：引入当前 crate（包）中的模块和类型，包括 instructions 模块和 AddressInfo 结构体。
///
///
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{instructions, state::AddressInfo};

///
/// 函数定义
/// program_id：程序的公钥，标识该智能合约在 Solana 上的唯一地址。
/// accounts：与智能合约交互的账户数组。
/// instruction_data：指令数据的字节数组。
/// ProgramResult：函数返回类型，表示程序执行的结果（成功或错误）。
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 处理指令数据
    // AddressInfo::try_from_slice(instruction_data)：尝试从指令数据中反序列化出一个 AddressInfo 结构体。如果成功，address_info 将包含反序列化后的数据。
    // if let Ok(address_info)：使用 if let 解构语法匹配反序列化结果。如果成功，执行块内代码。
    // instructions::create::create_address_info(program_id, accounts, address_info)：
    // 调用 instructions::create 模块中的 create_address_info 函数，传递程序 ID、账户数组和反序列化后的地址信息。
    if let Ok(address_info) = AddressInfo::try_from_slice(instruction_data) {
        return instructions::create::create_address_info(program_id, accounts, address_info);
    };
    // 处理错误
    // 如果反序列化失败（即指令数据无效），返回一个 ProgramError::InvalidInstructionData 错误，表示指令数据无效。
    Err(ProgramError::InvalidInstructionData)
}

/*
完整的函数流程
    接收程序 ID、账户数组和指令数据。
    尝试从指令数据中反序列化出 AddressInfo 结构体。
    如果反序列化成功，调用 create_address_info 函数执行具体的业务逻辑。
    如果反序列化失败，返回指令数据无效错误。
    AddressInfo 和 create_address_info 的假设
    为了更好地理解这段代码，我们需要假设 AddressInfo 和 create_address_info 的结构和功能：

    AddressInfo 是一个定义了地址信息的结构体，支持 Borsh 反序列化。
*/
