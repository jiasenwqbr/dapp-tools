/// 使用了 Anchor 框架，它是为 Solana 区块链开发的高级框架，旨在简化智能合约的编写。
/// 代码定义了一个名为 AddressInfo 的账户结构体，并通过各种属性标签（attributes）来描述该账户的存储和初始化需求。
///
/// 导入模块
/// use anchor_lang::prelude::*;：引入 Anchor 框架的预导入模块，这样可以使用 Anchor 提供的各种类型和宏。
use anchor_lang::prelude::*;
/// 定义账户结构体
/// #[account]：这个属性标签告诉 Anchor 这是一个账户数据结构。Anchor 会生成必要的代码来处理这个账户的数据序列化和反序列化。
/// #[derive(InitSpace)]：这个属性标签为该结构体派生出 InitSpace 特征，Anchor 会自动计算并分配适当的存储空间以保存账户数据。
///
/// 定义字段
/// #[max_len(50)]：这个属性标签指定 name 字段的最大长度为 50 个字符。Anchor 使用这个信息来分配存储空间并验证字符串长度。
/// pub name: String：定义一个公共的 name 字段，类型为 String。
#[account]
#[derive(InitSpace)]
pub struct AddressInfo {
    #[max_len(50)]
    pub name: String,
    pub house_number: u8,

    #[max_len(50)]
    pub street: String,
    #[max_len(50)]
    pub city: String,
}

/*
这个 AddressInfo 结构体代表一个存储在 Solana 区块链上的账户数据。
使用 Anchor 框架，我们可以轻松定义数据结构并指定其存储需求。
以下是每个部分的具体作用：
    导入 Anchor 框架：提供了必要的类型和宏，以便简化智能合约的开发。
    账户属性标签：使用 #[account] 和 #[derive(InitSpace)] 属性标签，告诉 Anchor 这是一个账户数据结构，并且需要初始化存储空间。
    字段定义：定义了四个字段（name, house_number, street, city），每个字段的最大长度属性标签确保在分配和存储时不会超过指定长度。
*/
