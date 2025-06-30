const { ethers, upgrades } = require('hardhat');

async function main () {
  const accounts = (await ethers.getSigners()).map(signer => signer.address);
  const AdminBox = await ethers.getContractFactory('AdminBox');
  console.log('Deploying AdminBox...');
  const adminBox = await upgrades.deployProxy(AdminBox, [accounts[0]], { initializer: 'initialize' });
  await adminBox.waitForDeployment();
  console.log('AdminBox deployed to:', await adminBox.getAddress());
}

main();

// AdminBox deployed to: 0x0165878A594ca255338adfa4d48449f69242Eb8F