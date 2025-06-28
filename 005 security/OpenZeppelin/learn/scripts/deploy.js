const { ethers } = require("hardhat");

async function main(){
    // we get the contract to deploy
    const Box = await ethers.getContractFactory('Box');
    console.log('Deploying Box...');
    const box = await Box.deploy();
    await box.waitForDeployment();
    console.log('Box deployed to:', await box.getAddress());
}

main().then(()=> process.exit(0))
.catch(
    error => {
        console.log(error);
        process.exit(1);
    }
);