fn main() {
    signature_parser();

}
fn signature_parser(){
    let signature_bytes: Vec<u8> = vec![
        14, 77, 249, 178, 2, 126, 86, 44, 172, 140, 8, 55, 79, 179, 40, 7, 99, 230, 140, 171, 162, 231, 15, 178, 4, 100, 204, 222, 138, 224, 47, 236, 216, 79, 202, 190, 64, 233, 123, 92, 59, 186, 35, 41, 95, 103, 32, 214, 88, 58, 170, 146, 221, 215, 255, 220, 130, 103, 5, 218, 152, 161, 37, 1,
    ];
    let signature_base58 = bs58::encode(signature_bytes).into_string();
    // 输出 Base58 编码的签名
    println!("Base58签名: {:?}", signature_base58);
}