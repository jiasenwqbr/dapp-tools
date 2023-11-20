const { expect } = require("chai");
const tokens = (n) => {
  return ethers.utils.parseUnits(n.toString(), "ether");
};

describe("Dappcord", function () {
  let deployer, user;
  let dappcord;

  const NAME = "Dappcord";
  const SYMBOL = "DC";
  beforeEach(async () => {
    // Setup accounts
    [deployer, user] = await ethers.getSigners();
    // Deploy contract
    const Dappcord = await ethers.getContractFactory("Dappcord");
    dappcord = await Dappcord.deploy(NAME, SYMBOL);
    // Create a channel
    const transaction = await dappcord
      .connect(deployer)
      .createChannel("general", tokens(1));
    await transaction.wait();
  });
  describe("Deployment", function () {
    it("Sets the name", async () => {
      const result = await dappcord.name();
      expect(result).to.equal(NAME);
    });

    it("Sets the symbol", async () => {
      const result = await dappcord.symbol();
      expect(result).to.equal(SYMBOL);
    });

    it("Sets the owner", async () => {
      const result = await dappcord.owner();
      expect(result).to.equal(deployer.address);
    });
  });
});
