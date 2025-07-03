import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@openzeppelin/hardhat-upgrades";
const config: HardhatUserConfig = {
  networks: {
    hardhat: {
    },
    PIJSLOCAL: {
      url: "http://192.168.10.132:8543",
      chainId: 20250521,
      accounts: ['0x7305aad8807af71adb0d86f9b7b72d009407d41e8f5f075cd67ca02995d692fc'],
    },
    ganache: {
      url: "http://127.0.0.1:7545",
      chainId: 1337,
      accounts: ['0x67e46235d56575ab55fdcaafd40c82833476f00bc5270dca0504193b30a53632'
        ,'0x60a0db20bc511b00071db794d6bf3ea0261e4ea88688e7841372f854377bfe8a'
        ,'0x81c71dface80de03b7f659bc6cb3211afe4c2abe78362d847e57b79b8bfe61c8'
        ,'0x4cff6db8f0a190277cfbe41f112ca1c30aef43f4bea4a191db820ac17bab9caf'
        ,'0xa53210b0dc3f83a9d586da8347650830c1beb41fd5a1e977bd816713d60ba3be'],
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
