const SimpleToken = artifacts.require("SimpleToken");
contract("SimpleToken", (accounts) => {
  it(`Should put 100000 to the ${accounts[0]}`, async () => {
    const SimpleTokenIns = await SimpleToken.deployed();
    const balance = (
      await simpleTokenIns.balanceOf.call(accounts[0])
    ).toNumber();
  });
  assert.equal(
    balance,
    100000,
    `the balance of ${accounts[0]} wasn not 100000`
  );
});
