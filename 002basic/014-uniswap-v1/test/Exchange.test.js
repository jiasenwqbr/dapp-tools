const { expect } = require("chai");
const { ethers } = require("hardhat");
const toWei = (value) => ethers.parseEther(value.toString());
const fromWei = (value) =>
  ethers.formatEther(typeof value === "string" ? value : value.toString());

describe("Exchange", () => {
  let owner;
  let user;
  let exchange;
  let token;
  beforeEach(async () => {
    [owner, user] = await ethers.getSigners();
    const Token = await ethers.getContractFactory("Token");
    token = await Token.deploy("Token", "TKN", toWei(1000000));
    console.log("token.address:", token.target);
    const Exchange = await ethers.getContractFactory("Exchange");
    exchange = await Exchange.deploy(token.target);
    console.log("exchange:", exchange);
  });
  describe("addLiquidity", async () => {
    it("adds liquidity", async () => {
      await token.approve(exchange.target, toWei(200));
      await exchange.addLiquidity(toWei(200), { value: toWei(100) });

      expect(await ethers.provider.getBalance(exchange.target)).to.equal(
        toWei(100)
      );
      expect(await exchange.getReserve()).to.equal(toWei(200));
    });

    it("allows zero amounts", async () => {
      await token.approve(exchange.target, 0);
      await exchange.addLiquidity(0, { value: 0 });

      expect(await ethers.provider.getBalance(exchange.target)).to.equal(0);
      expect(await exchange.getReserve()).to.equal(0);
    });
  });

  describe("getTokenAmount", async () => {
    it("returns correct token amount", async () => {
      await token.approve(exchange.target, toWei(2000));
      await exchange.addLiquidity(toWei(2000), { value: toWei(1000) });

      let tokensOut = await exchange.getTokenAmount(toWei(1));
      expect(fromWei(tokensOut)).to.equal("1.998001998001998001");

      tokensOut = await exchange.getTokenAmount(toWei(100));
      expect(fromWei(tokensOut)).to.equal("181.818181818181818181");

      tokensOut = await exchange.getTokenAmount(toWei(1000));
      expect(fromWei(tokensOut)).to.equal("1000.0");
    });
  });
  describe("getEthAmount", async () => {
    it("returns correct ether amount", async () => {
      await token.approve(exchange.target, toWei(2000));
      await exchange.addLiquidity(toWei(2000), { value: toWei(1000) });

      let ethOut = await exchange.getEthAmount(toWei(2));
      expect(fromWei(ethOut)).to.equal("0.999000999000999");

      ethOut = await exchange.getEthAmount(toWei(100));
      expect(fromWei(ethOut)).to.equal("47.619047619047619047");

      ethOut = await exchange.getEthAmount(toWei(2000));
      expect(fromWei(ethOut)).to.equal("500.0");
    });
  });
  describe("ethToTokenSwap", async () => {
    beforeEach(async () => {
      await token.approve(exchange.target, toWei(2000));
      await exchange.addLiquidity(toWei(2000), { value: toWei(1000) });
    });

    it("transfers at least min amount of tokens", async () => {
      const userBalanceBefore = await ethers.provider.getBalance(user.address);

      await exchange
        .connect(user)
        .ethToTokenSwap(toWei(1.99), { value: toWei(1) });

      const userBalanceAfter = await ethers.provider.getBalance(user.address);
      expect(fromWei(userBalanceAfter - userBalanceBefore)).to.equal(
        "-1.0004877520006021"
      );

      const userTokenBalance = await token.balanceOf(user.address);
      expect(fromWei(userTokenBalance)).to.equal("1.998001998001998001");

      const exchangeEthBalance = await ethers.provider.getBalance(
        exchange.target
      );
      expect(fromWei(exchangeEthBalance)).to.equal("1001.0");

      const exchangeTokenBalance = await token.balanceOf(exchange.target);
      expect(fromWei(exchangeTokenBalance)).to.equal("1998.001998001998001999");
    });
  });
});
