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

## Bridge.sol

### 8.`deposite` 与 `withdraw` 缺少重入保护

虽然合约继承了 `ReentrancyGuardUpgradeable`，但在 `deposite` 和 `withdraw` 函数上并未添加 `nonReentrant` 修饰符。

攻击者可在中间状态进行二次调用，造成资金重复转移或逻辑异常。

### 9.receiver`、`outgoingAddress`、`feeReceiver` 等地址为 `private

关键地址（接收方、出金方、手续费方）被标记为 `private`，且只有合约内部访问。无法在链上验证合约参数是否被篡改。

解决办法：添加getter方法

```solidity
 function getReceiver() external view returns (address) {
        return receiver;
    }

    function getOutgoingAddress() external view returns (address) {
        return outgoingAddress;
    }

    function getFeeReceiver() external view returns (address) {
        return feeReceiver;
    }
```

###  10.签名范围与 “重复使用” 攻击

用户提交的存款与提取签名中，包含了 `order` 字段，用于标识请求顺序；但合约并未记录已使用的 `order`，也不防止同一签名重复提交。

攻击者可抓包后多次调用同一笔签名请求，造成重复存款或重复提取。

```solidity
mapping(uint256 => bool) private usedOrders;

// 在 parseData / withdraw 前检查
require(!usedOrders[order], "Bridge: ORDER_USED");
usedOrders[order] = true;

```



### 11.单笔提取金额上限未防范溢出

`require((usdtAmount + feeAmount) <= 1000 * 1e18)` 用于限制最大值，但两个数相加可能溢出触发异常前检查。

在极端情况下，`usdtAmount + feeAmount` 溢出后反而绕过检查。

```solidity
require(usdtAmount <= 1000 * 1e18, “value error”);
require(feeAmount <= 1000 * 1e18 - usdtAmount, “value error”);
```



### 12.缺少紧急停止（Pausable）机制

一旦发现异常（如私钥泄露、签名者被攻破），无法快速暂停合约操作。

继承 `PausableUpgradeable`，在 `deposite`/`withdraw` 中加入：

```solidity
function deposite(...) public whenNotPaused nonReentrant { … }
function withdraw(...) public onlyRole(OPERATE_ROLE) whenNotPaused nonReentrant { … }
```



### 13.`parseData` 与 `withdraw` 重复签名验证逻辑

两个函数都对签名做了几乎相同的验证，只是 `chainId` 字段多与少。重复代码增加维护与出错风险。

抽取通用的签名验证内部函数，统一处理，并在参数中区分 `chainId` 是否参与哈希。



### PiJFactory.sol

#### PiJFactory

##### 14.INIT_CODE_PAIR_HASH计算重复

其中在常量中已经定义了INIT_CODE_PAIR_HASH这个常量，为什么还要在createPair中重复执行相同的代码？

```solidity
bytes32 public constant INIT_CODE_PAIR_HASH =
        keccak256(abi.encodePacked(type(PiJPair).creationCode));
```

```solidity
bytes memory bytecode = type(PiJPair).creationCode;
        bytes32 salt = keccak256(abi.encodePacked(token0, token1));
```

两段代码重复



##### 15.未检查create2是否成功

```solidity
assembly {
        pair := create2(0, add(bytecode, 32), mload(bytecode), salt)
    }
    require(pair != address(0), "PiJ: PAIR_DEPLOY_FAILED");
```



require(pair != address(0)) 保证部署成功，这样能避免零地址污染的漏洞。













