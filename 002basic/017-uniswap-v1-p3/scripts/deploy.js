const { ethers } = require("hardhat");

async function main() {
  const Token = await ethers.getContractFactory("Token");
  const token = await Token.deploy("Token", "TKN", (10 ** 18).toString());
  // await token.deployed();
  console.log("Token deployed to:", token.address);

  const Exchange = await ethers.getContractFactory("Exchange");
  const exchange = await Exchange.deploy(token.address);

  // await greeter.deployed();

  console.log("Exchange deployed to:", exchange.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
