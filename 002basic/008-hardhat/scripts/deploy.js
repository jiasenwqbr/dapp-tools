// We require the Hardhat Runtime Environment explicitly here. This is optional
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
// 我们在这里明确要求 Hardhat 运行时环境。 这是可选的
// 但对于通过 `node <script>` 以独立方式运行脚本很有用。
//
// 当使用 `hardhat run <script>` 运行脚本时，您将找到 Hardhat
// 运行时环境的成员在全局范围内可用。
const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  console.log("Account balance:", (await deployer.getBalance()).toString());
  const Token = await ethers.getContractFactory("SimpleToken");
  const token = await Token.deploy(
    "SimpleToken",
    "SimpleToken",
    18,
    10000000000
  );
  console.log("Token address:", token.address);

  let balance = await token.balanceOf(deployer.address);
  console.log(balance.toString());
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
// 我们推荐这种模式，以便能够在任何地方使用 async/await
// 并正确处理错误。
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
