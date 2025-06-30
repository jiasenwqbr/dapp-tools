const { ethers, upgrades } = require('hardhat');

async function main () {
  const accounts = (await ethers.getSigners()).map(signer => signer.address);
  const Box = await ethers.getContractFactory('Box');
  console.log('Deploying Box...');
  const box = await upgrades.deployProxy(Box, [accounts[0]],{initializer: 'initialize' });
  await box.waitForDeployment();
  console.log('Box deployed to:', await box.getAddress());
}

main();