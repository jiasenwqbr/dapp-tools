import { expect } from "chai";
import { ethers, upgrades } from "hardhat";
import { PIJSOrderV1 } from "../typechain-types";
describe("PIJSOrderV1", () => {
  let orderContract: PIJSOrderV1;
  let owner: any;
  let user1: any;
  let user2: any;
  beforeEach(async () => {
    [owner, user1, user2] = await ethers.getSigners();

    const PIJSOrderFactory = await ethers.getContractFactory("PIJSOrderV1");
    orderContract = (await upgrades.deployProxy(PIJSOrderFactory, [owner.address], {
      initializer: "initialize",
    })) as PIJSOrderV1;

    await orderContract.deployed();
  });
  describe("Initialization", () => {
    it("should assign roles correctly", async () => {
      const hasManageRole = await orderContract.hasRole(
        await orderContract.MANAGE_ROLE(),
        owner.address
      );
      expect(hasManageRole).to.be.true;
    });
  });

  

})