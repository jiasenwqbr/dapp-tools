/// 实现了一个 Solana 程序的入口点，并定义了模块结构。它主要用来处理传入的指令，并将其分发给适当的处理函数。
///
/// 导入和模块声明
use processor::process_instruction;
use solana_program::entrypoint;

pub mod instructions;
pub mod processor;
pub mod state;

/// 定义入口点
/// 使用 Solana 程序库中的 entrypoint 宏，将 process_instruction 函数指定为程序的入口点。
/// 这个宏展开后会生成适当的代码，使 Solana 运行时能够调用这个函数来处理传入的指令。
entrypoint!(process_instruction);
