// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/math/MathUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";



contract CrossChainLock is
    Initializable,
    AccessControlEnumerableUpgradeable,
    ReentrancyGuardUpgradeable,
    UUPSUpgradeable
{

    using SafeERC20Upgradeable for IERC20Upgradeable;
    using SafeERC20 for IERC20;
    // 拥有合约升级、参数配置权限。
    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");
    // 可调用 withdraw()、执行出金操作。
    bytes32 public constant OPERATE_ROLE = keccak256("OPERATE_ROLE");
    // EIP‑712 签名域（domain）哈希，用于防重放攻击。
    bytes32 public DOMAIN_SEPARATOR;
    
    bytes32 private constant PERMIT_DEPOSIT_TYPEHASH =keccak256(
        abi.encodePacked(
            "Permit(address userAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant WITHDRAW_PERMIT_TYPEHASH = keccak256(
        abi.encodePacked(
            "Permit(address caller,uint24 feeType,uint256 amount,address userAddr,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant PERMIT_DEPOSIT_ERC20_TYPEHASH = keccak256(
        abi.encodePacked(
             "Permit(address userAddr,address tokenAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant WITHDRAWERC20_PERMIT_TYPEHASH = keccak256(
        abi.encodePacked(
            "Permit(address caller,uint256 fee,address tokenAddr,uint256 amount,address userAddr,uint256 orderId,uint256 chainId)"
        )
    );

    // 签名者地址
    address public signer;
    // receiver：deposite() 时实际收币的地址。
    // feeReceiver：存款产生的手续费去向。 手续费接收地址
    address private feeReceiver;

    uint256 public constant FEE_DENOMINATOR = 10000; // 精度：10000表示万分比
    uint256 public feePercent;      // 万分比手续费，例如 30 表示 0.3%
    mapping(address => bool) private whiteList;  // 手续费白名单
    mapping(uint24 => uint24) public  feeAmountTick; // 不同类型的手续费

    // event
    event DepositeUNI(address caller,uint256 amount, address receiver, uint256 order, uint256 chainId);
    event WithDrawUNI(address caller,address feeReceiver, uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId);
    event DepositeERC20(address caller,address tokenAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId);
    event WithdrawERC20(address caller,address tokenAddr,address feeReceiver,uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId);

    event FeeUpdated(uint256 newFee);
    event FeeReceiverUpdated(address newReceiver);


    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyRole(MANAGE_ROLE) {}

    function initialize(
        address _feeReceiver,
        address _signer,
        address _operator,
        uint24 _feePercent
    ) public initializer {
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);
        _grantRole(OPERATE_ROLE, _operator);
        feeReceiver = _feeReceiver;
        signer = _signer;
        // feePercent = _feePercent;
        feeAmountTick[1] = _feePercent;
        uint256 chainId;
        assembly {
            chainId := chainid()
        }
        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256(
                    "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
                ),
                keccak256(bytes("UnionBridgeSource")),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }

    

    function balance(address token) public view returns (uint256) {
        if (token == address(0)) {
            return address(this).balance;
        }
        return IERC20Upgradeable(token).balanceOf(address(this));
    }

    function setSigner(address _signer) public onlyRole(MANAGE_ROLE) {
        signer = _signer;
    }
    
    function setFeeReceiver(address _receiver) external onlyRole(DEFAULT_ADMIN_ROLE) {
        feeReceiver = _receiver;
        emit FeeReceiverUpdated(_receiver);
    }
    function setFeePercent(uint256 _feePercent) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(_feePercent <= 1500, "Fee too high"); // 最多 15%
        feePercent = _feePercent;
        emit FeeUpdated(_feePercent);
    }

    // add to blacklist
    function setAddressToWhiteList(address _address) external onlyRole(OPERATE_ROLE) {
        whiteList[_address] = true;
    }

    // remove from blacklist
    function removeFromBlackList(address _address) external  onlyRole(OPERATE_ROLE) {
        require(isWhiteListed(_address),"Address not in white list");
        whiteList[_address] = false;
    }
    // 检查地址是否在黑名单中
    function isWhiteListed(address _address) public view returns (bool) {
        return  whiteList[_address];
    }

    function setFeeAmountTick(uint24 fee_type,uint24 fee_amount) external onlyRole(OPERATE_ROLE) {
        feeAmountTick[fee_type] = fee_amount;
    }
    function getFeeAmountTick(uint24 fee_type) public view  returns (uint24) {
        if (feeAmountTick[fee_type] == 0){
            return feeAmountTick[1];
        }
        return feeAmountTick[fee_type];
    }
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }
/////////////////////////////////////////// depositeUNI /////////////////////////
    struct DepositeData {
        address receiver;
        uint256 amount;
        uint256 orderId;
        uint256 chainId;
    }
    function depositeUNI(bytes calldata data) external nonReentrant payable {
        require(msg.value > 0, "No ETH sent");
        DepositeData memory depositeData = parseDepositeData(data);
        emit DepositeUNI(msg.sender,depositeData.amount,depositeData.receiver,depositeData.orderId,depositeData.chainId);

    }

    function parseDepositeData(bytes calldata data) internal view returns (DepositeData memory) {
        (
            address userAddr,
            address _receiver,
            uint256 amount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
                address,
                uint256,
                uint256,
                uint256,
                bytes
            )
        );
        require(userAddr == msg.sender, "UnionBridgeSource: INVALID_USER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);

         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_DEPOSIT_TYPEHASH,
                        userAddr,
                        _receiver,
                        amount,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "UnionBridgeSource: INVALID_REQUEST"
        );
        return
            DepositeData({
                receiver:_receiver,
                amount:amount,
                orderId:orderId,
                chainId:chainId
            });
    }
    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8, bytes32, bytes32) {
        require(sig.length == 65, "UnionBridgeSource:Not Invalid Signature Data");
        bytes32 r;
        bytes32 s;
        uint8 v;
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }
/////////////////////////////////////////// withdrawUNI /////////////////////////
    struct WithDrawData {
        uint256 amount;
        address userAddr;
        uint256 fee;
        uint256 orderId;
        uint256 chainId;
    }
    function withdrawUNI(bytes calldata data) public nonReentrant onlyRole(OPERATE_ROLE) {
        WithDrawData memory withDrawData = parseWithDrawData(data);
        require(withDrawData.amount > 0,"UnionBridgeSource:No ETH withdraw");
        require(withDrawData.amount <= address(this).balance,"UnionBridgeSource:");

       // uint256 feeAmount = (withDrawData.amount * getFeeAmountTick(withDrawData.feeType)) / FEE_DENOMINATOR;
      //  uint256 userAmount = withDrawData.amount - feeAmount;
        (bool sentFee, ) = payable(feeReceiver).call{value: withDrawData.fee}("");
        require(sentFee, "ETH transfer failed");
        (bool sendUserValue,) = payable(withDrawData.userAddr).call{value: withDrawData.amount}("");
        require(sendUserValue, "ETH transfer failed");
        emit WithDrawUNI(msg.sender,feeReceiver,withDrawData.fee,withDrawData.userAddr,withDrawData.amount,withDrawData.orderId,withDrawData.chainId);
    }

    function parseWithDrawData(bytes calldata data) internal view returns (WithDrawData memory) {
        (
            address callerAddr,
            uint256 amount,
            address userAddr,
            uint256  fee,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
                uint256,
                address,
                uint256,
                uint256,
                uint256,
                bytes
            )
        );
        require(callerAddr == msg.sender, "UnionBridgeSource: INVALID_USER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        WITHDRAW_PERMIT_TYPEHASH,
                        callerAddr,
                        amount,
                        userAddr,
                        fee,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "UnionBridgeSource: INVALID_REQUEST"
        );
        return WithDrawData({
            amount:amount,
            userAddr:userAddr,
            fee:fee,
            orderId:orderId,
            chainId:chainId
        });
    }
    /////////////////////////////////////////// depositeERC20 /////////////////////////
    struct DepositeERC20Data {
        address tokenAddr;
        address receiver;
        uint256 amount;
        uint256 orderId;
        uint256 chainId;
    }
    function depositeERC20(bytes calldata data) external{
        DepositeERC20Data memory depositeERC20Data = parseDepositeERC20Data(data);
        require(depositeERC20Data.amount > 0, "UnionBridgeSource:No token sent");
        IERC20(depositeERC20Data.tokenAddr).safeTransferFrom(
            msg.sender,
            address(this),
            depositeERC20Data.amount
        );
        emit DepositeERC20(msg.sender,depositeERC20Data.tokenAddr,depositeERC20Data.receiver,depositeERC20Data.amount,depositeERC20Data.orderId,depositeERC20Data.chainId);
    }

    function parseDepositeERC20Data(bytes calldata data) internal view returns (DepositeERC20Data memory) {
        (
            address userAddr,
            address tokenAddr,
            address _receiver,
            uint256 amount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
                address,
                address,
                uint256,
                uint256,
                uint256,
                bytes
            )
        );
        require(userAddr == msg.sender, "UnionBridgeSource: INVALID_USER");
         (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);

        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        PERMIT_DEPOSIT_ERC20_TYPEHASH,
                        userAddr,
                        tokenAddr,
                        _receiver,
                        amount,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "UnionBridgeSource: INVALID_REQUEST"
        );
        return DepositeERC20Data({
            tokenAddr:tokenAddr,
            receiver:_receiver,
            amount:amount,
            orderId:orderId,
            chainId:chainId
        });
    }

    /////////////////////////////////////////// withdrawERC20 /////////////////////////
    struct WithDrawERC20Data {
        address tokenAddr;
        uint256 amount;
        address userAddr;
        uint256 fee;
        uint256 orderId;
        uint256 chainId;
    } 
    function withdrawERC20(bytes calldata data) public nonReentrant onlyRole(OPERATE_ROLE) {
        WithDrawERC20Data memory withDrawERC20Data = parseWithDrawERC20Data(data);
        require(withDrawERC20Data.amount > 0,"UnionBridgeSource:No ETH withdraw");
        require(withDrawERC20Data.amount <= IERC20(withDrawERC20Data.tokenAddr).balanceOf(address(this)), "UnionBridgeSource:Insufficient token balance");
        // uint256 feeAmount = (withDrawERC20Data.amount * getFeeAmountTick(withDrawERC20Data.feeType)) / FEE_DENOMINATOR;
        // uint256 userAmount = withDrawERC20Data.amount - feeAmount;
        IERC20(withDrawERC20Data.tokenAddr).safeTransfer(feeReceiver, withDrawERC20Data.fee);
        IERC20(withDrawERC20Data.tokenAddr).safeTransfer(withDrawERC20Data.userAddr, withDrawERC20Data.amount);
        emit WithdrawERC20(msg.sender,withDrawERC20Data.tokenAddr,feeReceiver,withDrawERC20Data.fee,withDrawERC20Data.userAddr,withDrawERC20Data.amount,withDrawERC20Data.orderId,withDrawERC20Data.chainId);
    }

    function parseWithDrawERC20Data(bytes calldata data) internal view returns (WithDrawERC20Data memory) {
        (
            address caller,
            address tokenAddr,
            uint256 amount,
            address userAddr,
            uint256 fee,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) = abi.decode(
            data,(
                address,
                address,
                uint256,
                address,
                uint24,
                uint256,
                uint256,
                bytes
            )
        );
        require(caller == msg.sender, "UnionBridgeSource: INVALID_USER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        WITHDRAWERC20_PERMIT_TYPEHASH,
                        caller,
                        tokenAddr,
                        amount,
                        userAddr,
                        fee,
                        orderId,
                        chainId
                    )
                )
            )
        );
        require(
            signer == ecrecover(signHash, v, r, s),
            "UnionBridgeSource: INVALID_REQUEST"
        );
        return WithDrawERC20Data({
            tokenAddr:tokenAddr,
            amount:amount,
            userAddr:userAddr,
            fee:fee,
            orderId:orderId,
            chainId:chainId
        });
    }
}