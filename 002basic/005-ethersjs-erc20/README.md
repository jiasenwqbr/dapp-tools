# ethersjs-erc20

## About this task 前言

This demo shows the procedure for creating an `ERC20` contract using `ethers.js`
本样例演示了使用 `ethers.js` 调用 `ERC20` 合约的开发流程
## Contents 内容

1. Deploy an `ERC20` contract  ERC20 合约部署  
   Deploying by using `depoly.js`. In this demo, we use the test network `Goerli` to deploy the contract, and we need to use an account with Ether to send the transaction.<br>通过 `deploy.js` 进行部署，样例中链接的测试网为 Goerli, 对应需要使用有 Ether 的账户进行发送

2. Call the contract 合约调用  
   Call `transfer`, `balanceof` functions of the contract, and check the result.<br>调用 erc20 的 `transfer`, `balanceof` 接口, 验证合约部署结果

3. Listen to events 事件监听  
   Listen `Transfer` events by using `providerContract.once` and `providerContract.on`<br>之后使用 `providerContract.once` 和 `providerContract.on` 对 Transfer 事件进行一次和多次的监听

## How to run this task 测试流程

1. Install dependencies 安装依赖

   ```bash
   npm install
   ```

2. Config `.env` 配置 .env 

   ```bash
   cp .env.example .env
   # replace the xxx and yyy with your own key
   PRIVATE_KEY=xxxxxxxxxxxxxxxx
   INFURA_ID=yyyyyyyy
   ```

3. Run it 执行测试

   ```bash
   node index.js
   ```

## References

Official documentation:

- <https://docs.ethers.io/v4/api-providers.html>
- <https://docs.ethers.io/v5/getting-started/#getting-started--contracts>

