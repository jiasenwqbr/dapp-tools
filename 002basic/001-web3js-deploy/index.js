let { web3 } = require("web3");
let solc = require("solc");
let fs = require("fs");

// Get privatekey from environment
// 从环境中获取私钥
require("dotenv").config();
