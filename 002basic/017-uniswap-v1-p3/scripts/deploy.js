const hre = require("hardhat");

async function main() {
  const Token = await ethers.getContractFactory("Token");
  const token = await Token.deploy("Token", "TKN", (10 ** 18).toString());
  await token.deployed();

  const Exchange = await hre.ethers.getContractFactory("Exchange");
  const greeter = await Exchange.deploy(token.address);

  await greeter.deployed();

  console.log("Exchange deployed to:", greeter.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
