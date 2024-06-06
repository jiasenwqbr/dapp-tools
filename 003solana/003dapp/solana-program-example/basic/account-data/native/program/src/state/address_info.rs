/// 定义了一个名为 AddressInfo 的 Rust 结构体，
/// 并实现了 Borsh 序列化和反序列化。这是一个常见的模式，用于在区块链应用中进行数据的存储和传输。
///
/// 导入模块
/// borsh::{BorshDeserialize, BorshSerialize}：引入 Borsh 序列化和反序列化功能。
/// Borsh 是一种高效的二进制序列化格式，常用于区块链应用中。
use borsh::{BorshDeserialize, BorshSerialize};
/// #[derive(BorshDeserialize, BorshSerialize, Debug)]：
/// BorshDeserialize：为结构体实现 Borsh 反序列化，允许从字节数组恢复结构体实例。
/// BorshSerialize：为结构体实现 Borsh 序列化，允许将结构体实例转换为字节数组。
/// Debug：为结构体实现 Debug 特征，允许使用 {:?} 打印结构体的调试信息。
///
/// 定义一个公开的结构体 AddressInfo，包含四个字段：
/// name: String：表示名称（例如姓名）。
/// house_number: u8：表示房屋号。
/// street: String：表示街道。
/// city: String：表示城市。
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub house_number: u8,
    pub street: String,
    pub city: String,
}
///
/// impl AddressInfo：为 AddressInfo 结构体实现方法。
/// pub fn new(name: String, house_number: u8, street: String, city: String) -> Self：定义一个公开的关联函数 new，用于创建 AddressInfo 的新实例。
/// name: String：新实例的 name 字段。
/// house_number: u8：新实例的 house_number 字段。
/// street: String：新实例的 street 字段。
/// city: String：新实例的 city 字段。
/// Self：返回类型为 AddressInfo。
/// 函数体中，通过结构体初始化语法 { name, house_number, street, city } 创建并返回一个新的 AddressInfo 实例。

impl AddressInfo {
    pub fn new(name: String, house_number: u8, street: String, city: String) -> Self {
        AddressInfo {
            name,
            house_number,
            street,
            city,
        }
    }
}

/*
这段代码定义了一个 AddressInfo 结构体，用于存储地址信息，包括名称、房屋号、街道和城市。
通过 #[derive(BorshDeserialize, BorshSerialize, Debug)]，为这个结构体自动生成了 Borsh 序列化和反序列化的方法，以及调试打印的方法。
结构体的 new 方法提供了一种方便的方式来创建 AddressInfo 实例。
这种设计非常适合在区块链应用中使用，因为它简洁、高效，且易于通过网络传输或存储在区块链上。
*/
