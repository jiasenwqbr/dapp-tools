/**
 *
 * 掌握了用private key就可以控制钱包，两种类型的key互相转换的代码如下：
 */
const bs58 = require("bs58");
const key =
  "3prF4PQb4cAyKpVU4XQBxrUM5CZtsTFHL6V2h2qdWceH2FK79dhj8vjFWjGvmNAvEWGoqpyQidbFw1sQ5gntGkbC";
const address = bs58.decode(key);
console.log(address);

// uint8 => base58
const array = Uint8Array.from([
  130, 165, 34, 66, 102, 45, 248, 216, 178, 113, 137, 39, 206, 122, 92, 222,
  241, 214, 146, 124, 3, 136, 125, 35, 91, 79, 111, 219, 242, 161, 217, 57, 22,
  33, 222, 219, 118, 229, 49, 154, 163, 148, 148, 91, 69, 185, 21, 223, 26, 224,
  151, 155, 166, 179, 59, 167, 111, 181, 71, 183, 86, 245, 229, 175,
]);
const address1 = bs58.encode(array);
console.log(address1);

/**
 * 如果想获取private key对应的publick key（假设private key都是uint8格式）：
 */
const { Keypair } = require("@solana/web3.js");

const key1 = Uint8Array.from([
  130, 165, 34, 66, 102, 45, 248, 216, 178, 113, 137, 39, 206, 122, 92, 222,
  241, 214, 146, 124, 3, 136, 125, 35, 91, 79, 111, 219, 242, 161, 217, 57, 22,
  33, 222, 219, 118, 229, 49, 154, 163, 148, 148, 91, 69, 185, 21, 223, 26, 224,
  151, 155, 166, 179, 59, 167, 111, 181, 71, 183, 86, 245, 229, 175,
]);
const publicKey = Keypair.fromSecretKey(key1).publicKey;
console.log(publicKey);
