const { expect } = require("chai");
// `describe` is a Mocha function that allows you to organize your tests. It's
// not actually needed, but having your tests organized makes debugging them
// easier. All Mocha functions are available in the global scope.

// `describe` receives the name of a section of your test suite, and a callback.
// The callback must define the tests of that section. This callback can't be
// an async function.
// `describe` 是一个 Mocha 函数，可让您组织测试。 它是
// 实际上并不需要，但是组织测试可以调试它们
// 更轻松。 所有 Mocha 功能都在全局范围内可用。

// `describe` 接收测试套件的一部分的名称和回调。
// 回调必须定义该部分的测试。 这个回调不能
// 一个异步函数。

describe("Token contract", function () {
  // Mocha has four functions that let you hook into the the test runner's  lifecyle. These are: `before`, `beforeEach`, `after`, `afterEach`.
  // They're very useful to setup the environment for tests, and to clean it up after they run.
  // A common pattern is to declare some variables, and assign them in the `before` and `beforeEach` callbacks.
  // Mocha 有四个函数可让您挂钩测试运行程序的生命周期。 它们是：`before`、`beforeEach`、`after`、`afterEach`。
  // 它们对于设置测试环境以及运行后清理环境非常有用。
  // 一种常见的模式是声明一些变量，并在 `before` 和 `beforeEach` 回调中分配它们。
  let Token;
  let hardhatToken;
  let owner;
  let addr1;
  let addr2;
  let addrs;
  // `beforeEach` will run before each test, re-deploying the contract every
  // time. It receives a callback, which can be async.
  // `beforeEach` 将在每次测试之前运行，每次都重新部署合约。 它接收一个回调，该回调可以是异步的。
  beforeEach(async function () {
    // Get the ContractFactory and Signers here.
    // 获取ContractFactory和Signers
    Token = await ethers.getContractFactory("SimpleToken");
    [owner, addr1, addr2, ...addrs] = await ethers.getSigners();

    // To deploy our contract, we just have to call Token.deploy() and await
    // for it to be deployed(), which happens onces its transaction has been
    // mined.
    // 要部署我们的合约，我们只需调用 Token.deploy() 并等待它被部署()，这会在其交易被挖矿后发生。

    hardhatToken = await Token.deploy("HEHE", "HH", 1, 100000000);
  });
  // You can nest describe calls to create subsections.
  // 您可以嵌套描述调用来创建小节。
  describe("Deployment", function () {
    // `it` is another Mocha function. This is the one you use to define your
    // tests. It receives the test name, and a callback function.
    // `it` 是另一个 Mocha 函数。 这是您用来定义测试的一个。 它接收测试名称和回调函数。
    it("Should assign the total supply of tokens to the owner", async function () {
      const ownerBalance = await hardhatToken.balanceOf(owner.address);
      expect(await hardhatToken.totalSupply()).to.equal(ownerBalance);
    });
  });
  describe("Transactions", function () {
    it("Should transfer tokens between accounts", async function () {
      // Transfer 50 tokens from owner to addr1
      await hardhatToken.transfer(addr1.address, 50);
      const addr1Balance = await hardhatToken.balanceOf(addr1.address);
      expect(addr1Balance).to.equal(50);

      // Transfer 50 tokens from addr1 to addr2
      // We use .connect(signer) to send a transaction from another account
      await hardhatToken.connect(addr1).transfer(addr2.address, 50);
      const addr2Balance = await hardhatToken.balanceOf(addr2.address);
      expect(addr2Balance).to.equal(50);
    });

    it("Should fail if sender doesn’t have enough tokens", async function () {
      const initialOwnerBalance = await hardhatToken.balanceOf(owner.address);

      // Try to send 1 token from addr1 (0 tokens) to owner (1000 tokens).
      // `require` will evaluate false and revert the transaction.
      await expect(
        hardhatToken.connect(addr1).transfer(owner.address, 1)
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");

      // Owner balance shouldn't have changed.
      expect(await hardhatToken.balanceOf(owner.address)).to.equal(
        initialOwnerBalance
      );
    });

    it("Should update balances after transfers", async function () {
      const initialOwnerBalance = await hardhatToken.balanceOf(owner.address);

      // Transfer 100 tokens from owner to addr1.
      await hardhatToken.transfer(addr1.address, 100);

      // Transfer another 50 tokens from owner to addr2.
      await hardhatToken.transfer(addr2.address, 50);

      // Check balances.
      const finalOwnerBalance = await hardhatToken.balanceOf(owner.address);
      expect(finalOwnerBalance).to.equal(initialOwnerBalance - 150);

      const addr1Balance = await hardhatToken.balanceOf(addr1.address);
      expect(addr1Balance).to.equal(100);

      const addr2Balance = await hardhatToken.balanceOf(addr2.address);
      expect(addr2Balance).to.equal(50);
    });
  });
});
