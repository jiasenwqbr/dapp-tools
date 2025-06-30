const { ethers, upgrades } = require('hardhat');

async function main () {
  const Storage = await ethers.getContractFactory('Storage');
  console.log('Deploying Storage...');
  const storage = await upgrades.deployProxy(Storage, [42], { initializer: 'store' });
  await storage.waitForDeployment();
  console.log('Box deployed to:', await storage.getAddress());
}

main();