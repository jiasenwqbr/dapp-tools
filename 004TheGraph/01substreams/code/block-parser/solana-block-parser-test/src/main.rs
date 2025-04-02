fn main() {
    signature_parser();
    account_keys_parser();
    instruction_parser();
    // let vec_arr: [u8; 20] = [
    //     0, 0, 0, 212, 11, 89, 91, 148, 145, 138, 40, 178, 125, 30, 44, 102, 244, 58, 81, 211
    // ];
    // //let ss = parse_vec_to_string(vec_arr);
    // let ss = bytes_to_eth(&vec_arr);
    // println!("parse_vec_to_string is : {:?}",ss);

    let hash_bytes: Vec<u8> = vec![
        19, 70, 67, 239
    ];

    // 使用 hex crate 将字节数组转换为十六进制字符串
    let hash_hex = hex::encode(hash_bytes);
    

    // 输出结果
    println!("Hash (Hex): {}", hash_hex);
}


struct Instruction {
    program_id_index: u8,
    accounts: Vec<u8>,
    data: Vec<u8>,
}
#[allow(dead_code)] 
#[derive(Debug)]
struct InstructionParse {
    program_id_index: u8,
    accounts: String,
    data: String,
}

fn signature_parser() {
    let signature_bytes: Vec<u8> = vec![
        14, 77, 249, 178, 2, 126, 86, 44, 172, 140, 8, 55, 79, 179, 40, 7, 99, 230, 140, 171, 162,
        231, 15, 178, 4, 100, 204, 222, 138, 224, 47, 236, 216, 79, 202, 190, 64, 233, 123, 92, 59,
        186, 35, 41, 95, 103, 32, 214, 88, 58, 170, 146, 221, 215, 255, 220, 130, 103, 5, 218, 152,
        161, 37, 1,
    ];
    let signature_base58 = bs58::encode(signature_bytes).into_string();
    // 输出 Base58 编码的签名
    println!("Base58签名: {:?}", signature_base58);
    // Base58签名: "Hb5yiPZc5fdRbmGEbhuD1msT59Hu3uharSs7YJfznsDRA6tpymnkGvpafeHsDzjKEWfL3KhGbfToYesttBT6qiG"
    account_keys_parser();
    
}
fn parse_vec_to_string(vec_arr:Vec<u8>) -> String {
    let signature_base58 = bs58::encode(vec_arr).into_string();
    signature_base58
}
fn account_keys_parser() {
    let account_keys: Vec<Vec<u8>> = vec![
        vec![
            215, 42, 237, 101, 45, 211, 64, 47, 86, 99, 47, 47, 49, 64, 227, 159, 136, 14, 55, 97,
            131, 224, 122, 93, 151, 176, 150, 177, 89, 132, 118, 97,
        ],
        vec![
            118, 104, 154, 45, 28, 210, 221, 36, 214, 59, 97, 166, 86, 52, 208, 146, 182, 173, 184,
            61, 169, 121, 38, 114, 72, 128, 162, 224, 80, 134, 183, 102,
        ],
        vec![
            99, 131, 115, 0, 14, 162, 44, 178, 100, 211, 74, 255, 100, 160, 75, 94, 250, 191, 187,
            116, 221, 205, 4, 137, 151, 177, 152, 21, 71, 215, 209, 16,
        ],
        vec![
            9, 64, 183, 98, 240, 9, 66, 158, 76, 109, 253, 137, 225, 198, 214, 199, 113, 82, 77,
            109, 213, 255, 85, 135, 175, 57, 95, 52, 151, 1, 17, 169,
        ],
        vec![
            33, 220, 194, 160, 149, 131, 20, 191, 170, 89, 116, 163, 99, 194, 105, 197, 149, 24,
            81, 64, 163, 166, 244, 138, 5, 31, 131, 154, 31, 131, 161, 90,
        ],
        vec![
            172, 245, 71, 68, 54, 21, 36, 67, 51, 116, 101, 23, 254, 193, 73, 132, 119, 93, 6, 12,
            35, 36, 19, 214, 180, 121, 141, 231, 140, 207, 142, 42,
        ],
        vec![
            3, 6, 70, 111, 229, 33, 23, 50, 255, 236, 173, 186, 114, 195, 155, 231, 188, 140, 229,
            187, 197, 247, 18, 107, 44, 67, 155, 58, 64, 0, 0, 0,
        ],
        vec![
            140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19,
            153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
        ],
        vec![
            138, 7, 47, 47, 51, 204, 1, 198, 133, 221, 116, 15, 195, 46, 164, 216, 153, 12, 207,
            230, 64, 253, 203, 46, 208, 28, 242, 163, 66, 35, 202, 79,
        ],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ],
        vec![
            6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180,
            133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
        ],
        vec![
            147, 255, 101, 178, 97, 182, 55, 73, 102, 240, 94, 144, 36, 11, 175, 34, 5, 40, 206,
            17, 131, 161, 228, 74, 203, 17, 97, 222, 156, 23, 30, 154,
        ],
        vec![
            58, 134, 94, 105, 238, 15, 84, 128, 202, 188, 246, 99, 87, 228, 220, 47, 24, 213, 141,
            69, 193, 234, 116, 137, 251, 55, 35, 217, 121, 60, 114, 166,
        ],
        vec![
            6, 167, 213, 23, 25, 44, 92, 81, 33, 140, 201, 76, 61, 74, 241, 127, 88, 218, 238, 8,
            155, 161, 253, 68, 227, 219, 217, 138, 0, 0, 0, 0,
        ],
        vec![
            172, 241, 54, 235, 1, 252, 28, 78, 136, 61, 35, 200, 181, 132, 74, 181, 154, 55, 246,
            106, 221, 87, 197, 233, 172, 59, 83, 224, 89, 211, 92, 100,
        ],
        vec![
            1, 86, 224, 246, 147, 102, 90, 207, 68, 219, 21, 104, 191, 23, 91, 170, 81, 137, 203,
            151, 245, 210, 255, 59, 101, 93, 43, 182, 253, 109, 24, 176,
        ],
    ];
    let mut newvec: Vec<String> = Vec::new();
    for ele in account_keys {
        let s = bs58::encode(ele).into_string();
        newvec.push(s);
    }
    println!("account_keys is {:?}", newvec);
    // account_keys is ["FUvcyEAnzCizErHBW9DqeRY5oFZix2vWF99vvqfMTKhz", "8yDcueD1B9PJxUTHBqoSGH4bL7oMsnqzhApkXNQMqhe5", "7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX", "d7uUaknf8t1jXPADDqyW3gbo28ceCJ4ZwTonf3eVDgt", "3HBhLjdFe51qFGfQEN9nEZHWM9pMP6LqwmnxRjrQQSHF", "CeA3sPZfWWToFEBmw5n1Y93tnV66Vmp8LacLzsVprgxZ", "ComputeBudget111111111111111111111111111111", "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", "AHocjgHi5SmF8BhHbKSgsBGpsiXf8i1kpr7HJ65upump", "11111111111111111111111111111111", "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "Axiom3a2w1UbMt2SMgqSvRiuJFTPusDhwKamNgPTeNQ9", "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf", "SysvarRent111111111111111111111111111111111", "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1", "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"]
}

fn instruction_parser() {
    let mut instructions: Vec<Instruction> = Vec::new();
    let ins1 = Instruction {
        program_id_index: 6,
        accounts: vec![],
        data: vec![2, 160, 134, 1, 0],
    };
    instructions.push(ins1);

    let ins2 = Instruction {
        program_id_index: 6,
        accounts: vec![],
        data: vec![3, 64, 66, 15, 0, 0, 0, 0, 0],
    };
    instructions.push(ins2);

    let ins3 = Instruction {
        program_id_index: 7,
        accounts: vec![0, 1, 0, 8, 9, 10],
        data: vec![],
    };
    instructions.push(ins3);

    let ins4 = Instruction {
        program_id_index: 11,
        accounts: vec![12, 2, 8, 3, 4, 1, 0, 9, 10, 13, 14, 15],
        data: vec![
            0, 64, 220, 179, 17, 0, 0, 0, 0, 119, 162, 136, 54, 61, 8, 0, 0,
        ],
    };
    instructions.push(ins4);

    let ins5 = Instruction {
        program_id_index: 9,
        accounts: vec![0, 5],
        data: vec![2, 0, 0, 0, 192, 198, 45, 0, 0, 0, 0, 0],
    };
    instructions.push(ins5);

    let mut ins_parses: Vec<InstructionParse> = Vec::new();
    for ele in instructions {
        // ele.data = bs58::encode(ele.data).;
        let insp = InstructionParse{
            program_id_index:ele.program_id_index,
            accounts:bs58::encode(ele.accounts).into_string(),
            data:bs58::encode(ele.data).into_string()
        };
        ins_parses.push(insp);
    }

    println!("The instructions are:{:?}",ins_parses);
    
}

fn bytes_to_eth(bytes: &[u8; 20]) -> String {
    // 转换为十六进制字符串
    let hex_str = hex::encode(bytes);
    // 添加 0x 前缀
    format!("{}", hex_str)
}