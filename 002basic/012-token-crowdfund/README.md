## Crowdfunding Contract  众筹合约
This sample demonstrates the basic process of a crowdfunding contract, including the deployment of the contract, the contract, and the launch of a crowdfunding project.
本样例演示众筹合约的基本流程，包括部署合约、合约以及启动众筹项目。

## Operation Process 操作流程

- Config **.env** 配置 **.env**

```sh
cp .env.example .env

## Modify .env to the actual values of INFURA_ID and PRIVATE_KEY
PRIVATE_KEY=xxxxxxxxxxxxxxxx
INFURA_ID=yyyyyyyy
```

- Install Dependencies 安装依赖

```bash
npm install 
```

- Compile Contract 编译合约

```bash
npx hardhat compile
```

- Test Contract 测试合约

```bash
npx hardhat test
```

- Deploy Contract 部署合约

```bash
npx hardhat run scripts/deploy_crowdfunding.js --network goerli
```

## Refer to the link 参考链接

- https://medium.com/openberry/creating-a-simple-crowdfunding-dapp-with-ethereum-solidity-and-vue-js-69ddb8e132dd  
- https://medium.com/extropy-io/crowdsales-on-ethereum-with-openzeppelin-57bbdea95390  
- https://www.programmersought.com/article/1396206575/  
- https://github.com/OpenZeppelin/openzeppelin-contracts/tree/release-v2.3.0/contracts/crowdsale
- Linear Unlock： https://cloud.tencent.com/developer/article/1182701
