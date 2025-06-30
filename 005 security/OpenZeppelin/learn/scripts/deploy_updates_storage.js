const { ethers, upgrades } = require('hardhat');

async function main () {
  const StorageV2 = await ethers.getContractFactory('StorageV2');
  console.log('Upgrading Storage...');
  await upgrades.upgradeProxy('0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512', StorageV2);
  console.log('Storage upgraded');
}

main();