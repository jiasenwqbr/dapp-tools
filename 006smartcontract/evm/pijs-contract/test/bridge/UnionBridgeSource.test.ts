import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { network } from "hardhat";
import { UnionBridgeSource } from  "../../typechain-types";

describe("UnionBridgeSource", () => {
    let unionBridgeSourceContract:UnionBridgeSource;
    let owner: any;
    let user1: any;
    let user2: any;
    let user3: any;

    beforeEach(async () => {
        [owner, user1 ,user2, user3] = await ethers.getSigners();
        const unionBridgeSourceFactory =  await ethers.getContractFactory("UnionBridgeSource");
        unionBridgeSourceContract = (await upgrades.deployProxy(unionBridgeSourceFactory,[owner.address,owner.address,owner.address,owner.address,500],{
            initializer: "initialize",
        })) as UnionBridgeSource;
        await unionBridgeSourceContract.deployed();
    });

    describe("Initialization",() => {
        it("shold owner has the role of manage",async () => {
            const  hasManageRole =  await unionBridgeSourceContract.hasRole(
                await unionBridgeSourceContract.MANAGE_ROLE(),
                owner.address
            );
            expect(hasManageRole).to.be.true;
        });
    });





});


/**
 npx hardhat test ./test/bridge/UnionBridgeSource.test.ts --network ganache
 */