const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MyContract",function(){
    it ("should return correct name",async function(){
        const myContract =  await hre.ethers.getContractFactory("MyContract");
        const myContractDeployed = await myContract.deploy("MyContractName","MCN");
        await myContractDeployed.deployed();
        expect(await myContractDeployed.name()).to.equal("MyContractName");
        expect(await myContractDeployed.symbol()).to.equal("MCN");
    });
});