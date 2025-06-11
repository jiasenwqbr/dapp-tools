## AirDrop.sol

### 1.调用 airdrop时未做校验
   调用 airdrop 时默认认为合约中已经有足够的 token，若资金不足，将会在 safeTransfer 时 失败整个交易。

### 2.空投无法防止失败，一个失败会导致整个空头失败


### 3.没有事件（Event）记录
没有记录每次空投操作，用户和管理员都无法在链上查询日志。
容易造成信任和可追溯性问题。

###  4.没有提现函数（Withdraw Function）
合约收到了 BNB（receive()），但没有任何提现函数。
若误发或剩余资金将永久锁死。

### 5.重入攻击--空投逻辑没有防重入保护
虽然继承了 ReentrancyGuardUpgradeable，但 airdrop / airdropBNB 函数没有加上 nonReentrant 修饰符。
```solidity
function airdrop(...) public nonReentrant onlyRole(OPERATE_ROLE) { ... }

```

### 6.未检查地址是否为零地址
空投地址数组中如果包含 address(0)，将导致转账失败或资金丢失。

### 7.没有暂停功能
一旦合约发现有误操作或被攻击，无法快速停用空投功能。

