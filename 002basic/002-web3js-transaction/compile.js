const fs = require("fs");
const solc = require("solc");
// Load contracts in UTF-8 mode
// 以utf-8的方式加载合约
const source = fs.readFileSync("Incrementer.sol", "utf8");
// Complie the smart contract
// 编译合约
const input = {
  language: "Solidity",
  sources: {
    "Incrementer.sol": {
      content: source,
    },
  },
  settings: {
    outputSelection: {
      "*": {
        "*": ["*"],
      },
    },
  },
};

const tempFile = JSON.parse(solc.compile(JSON.stringify(input)));
const contractOfIncrementer =
  tempFile.contracts["Incrementer.sol"]["Incrementer"];
// Export contract data and use the console to print specific content information in the contractFile
// 导出合约数据，可以使用 console 打印 contractFile 中的具体内容信息
module.exports = contractOfIncrementer;
