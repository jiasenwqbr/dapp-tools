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
      accounts: ['0x7ff1783cc0df9b17e6e6178793d673d7f0106e63278020ad199e64977123b987'
        ,'0xef82edea88b8d23e652c607145fa564da2baec70ae3d91041ca1ebc007709b32'
        ,'0xdee0d35e86d72447ee55c1cf28a954aea9347d2c837bf8f1f7e5fbc2c6cb91c3'
        ,'0x18e36fb0981056be4e33f0dab323466c52e729e728e79ddea9712d43b4e2a32c'
        ,'0xdbebd2c321e215dccac2d0f1f4997ad09031c9e572b871de4ce0e396445b07b4'],
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
