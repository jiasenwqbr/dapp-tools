const SimpleToken = artifacts.require("PijsToken");
module.exports = function (deployer) {
  deployer.deploy(SimpleToken, "PijsToken", "PijsToken", 1, 10000000);
};
