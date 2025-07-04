import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@openzeppelin/hardhat-upgrades";
import * as dotenv from "dotenv";
dotenv.config();

const PRIVATE_KEY = process.env.PRIVATE_KEY || "";
const GANACHE_PRIVATE_KEYS = process.env.GANACHE_PRIVATE_KEYS?.split(",") || [];
const PIJS_TEST_NET_KEYS = process.env.PIJS_TEST_NET_KEYS?.split(",") || []

const config: HardhatUserConfig = {
  networks: {
    hardhat: {
    },
    PIJSLOCAL: {
      url: "http://192.168.10.132:8543",
      chainId: 20250521,
      accounts: PRIVATE_KEY ? [PRIVATE_KEY] : [],
    },
    ganache: {
      url: "http://127.0.0.1:7545",
      chainId: 1337,
      accounts: GANACHE_PRIVATE_KEYS,
    },
    pijstestnet: {
      url: "https://testchain.pijswap.xyz",
      chainId: 20250521,
      accounts:PIJS_TEST_NET_KEYS,
    },
  },
  solidity: {
    version: "0.8.20",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200,
      },
      viaIR: true,  // 启用 IR 编译
    },
  },

};

export default config;
