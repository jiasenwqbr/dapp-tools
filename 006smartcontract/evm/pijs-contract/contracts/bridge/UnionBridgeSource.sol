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



contract UnionBridgeSource is
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
    // PERMIT_DEPOSIT_TYPEHASH、WITHDRAW_PERMIT_TYPEHASH：分别定义存款和提取请求的签名结构
    bytes32 private constant PERMIT_DEPOSIT_TYPEHASH =keccak256(
        abi.encodePacked(
            "Permit(address userAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant WITHDRAW_PERMIT_TYPEHASH =keccak256(
        abi.encodePacked(
            "Permit(address caller,address feeReceiver,uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant PERMIT_DEPOSIT_ERC20_TYPEHASH = keccak256(
        abi.encodePacked(
             "Permit(address userAddr,address tokenAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId)"
        )
    );
    bytes32 private constant WITHDRAWERC20_PERMIT_TYPEHASH = keccak256(
        abi.encodePacked(
            "Permit(address caller,address tokenAddr,address feeReceiver,uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId)"
        )
    );

    // 签名者地址
    address public signer;
    // receiver：deposite() 时实际收币的地址。
    address private receiver;
    // outgoingAddress：withdraw() 时从哪划出资金。 提现代币来源地址
    address private outgoingAddress;
    // feeReceiver：存款产生的手续费去向。 手续费接收地址
    address private feeReceiver;

    // event
    event DepositeUNI(address caller,uint256 amount, address receiver, uint256 order, uint256 chainId);
    event WithDrawUNI(address caller,address feeReceiver, uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId);
    event DepositeERC20(address caller,address tokenAddr,address receiver,uint256 amount,uint256 orderId,uint256 chainId);
    event WithdrawERC20(address caller,address tokenAddr,address feeReceiver,uint256 feeAmount,address userAddr,uint256 userAmount,uint256 orderId,uint256 chainId);


    constructor() {
        _disableInitializers();
    }

    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyRole(MANAGE_ROLE) {}

    function initialize(
        address _receiver,
        address _outgoingAddress,
        address _feeReceiver,
        address _signer,
        address _operator
    ) public initializer {
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);
        _grantRole(OPERATE_ROLE, _operator);

        receiver = _receiver;
        outgoingAddress = _outgoingAddress;
        feeReceiver = _feeReceiver;
        signer = _signer;

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

    function setReceiver(address _receiver) public onlyRole(MANAGE_ROLE) {
        receiver = _receiver;
    }
     function setOutGoingAddress(
        address _outGoingAddress
    ) public onlyRole(MANAGE_ROLE) {
        outgoingAddress = _outGoingAddress;
    }

    function setFeeReceiver(address _feeReceiver) public onlyRole(MANAGE_ROLE) {
        feeReceiver = _feeReceiver;
    }
/////////////////////////////////////////// depositeUNI /////////////////////////
    struct DepositeData {
        address receiver;
        uint256 amoount;
        uint256 orderId;
        uint256 chainId;
    }
    function depositeUNI(bytes calldata data) external nonReentrant payable {
        require(msg.value > 0, "No ETH sent");
        DepositeData memory depositeData = parseDepositeData(data);
        emit DepositeUNI(msg.sender,depositeData.amoount,depositeData.receiver,depositeData.orderId,depositeData.chainId);

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
                amoount:amount,
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
        address feeReceiver;
        uint256 feeAmount;
        address userAddr;
        uint256 userAmount;
        uint256 orderId;
        uint256 chainId;
    }
    function withdrawUNI(bytes calldata data) public nonReentrant onlyRole(OPERATE_ROLE) {
        WithDrawData memory withDrawData = parseWithDrawData(data);
        require(withDrawData.feeAmount > 0,"UnionBridgeSource:No ETH withdraw");
        require(withDrawData.feeAmount <= address(this).balance,"UnionBridgeSource:");
         (bool sentFee, ) = payable(withDrawData.feeReceiver).call{value: withDrawData.feeAmount}("");
        require(sentFee, "ETH transfer failed");
        (bool sendUserValue,) = payable(withDrawData.userAddr).call{value: withDrawData.userAmount}("");
        require(sendUserValue, "ETH transfer failed");
        emit WithDrawUNI(msg.sender,withDrawData.feeReceiver,withDrawData.feeAmount,withDrawData.userAddr,withDrawData.userAmount,withDrawData.orderId,withDrawData.chainId);
    }

    function parseWithDrawData(bytes calldata data) internal view returns (WithDrawData memory) {
        (
            address callerAddr,
            address _feeReceiver,
            uint256 feeAmount,
            address userAddr,
            uint256 userAmount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) =  abi.decode(
            data,
            (
                address,
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
        require(_feeReceiver == feeReceiver, "UnionBridgeSource: INVALID_FEE_RECEIVER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        WITHDRAW_PERMIT_TYPEHASH,
                        callerAddr,
                        _feeReceiver,
                        feeAmount,
                        userAddr,
                        userAmount,
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
            feeReceiver:_feeReceiver,
            feeAmount:feeAmount,
            userAddr:userAddr,
            userAmount:userAmount,
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
    function depositeERC20(bytes calldata data) external  payable {
        require(msg.value > 0, "No ETH sent");
        DepositeERC20Data memory depositeERC20Data = parseDepositeERC20Data(data);
        require(msg.value == depositeERC20Data.amount,"UnionBridgeSource:invalid amount");
        IERC20(depositeERC20Data.tokenAddr).safeTransfer(address(this), msg.value);
        // emit
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
        address feeReceiver;
        uint256 feeAmount;
        address userAddr;
        uint256 userAmount;
        uint256 orderId;
        uint256 chainId;
    } 
    function withdrawERC20(bytes calldata data) public nonReentrant onlyRole(OPERATE_ROLE) {
        WithDrawERC20Data memory withDrawERC20Data = parseWithDrawERC20Data(data);
        require(withDrawERC20Data.feeAmount > 0,"UnionBridgeSource:No ETH withdraw");
        require(withDrawERC20Data.feeAmount <= address(this).balance,"UnionBridgeSource:");
        IERC20(withDrawERC20Data.tokenAddr).safeTransfer(withDrawERC20Data.feeReceiver, withDrawERC20Data.feeAmount);
        IERC20(withDrawERC20Data.tokenAddr).safeTransfer(withDrawERC20Data.userAddr, withDrawERC20Data.userAmount);
        emit WithdrawERC20(msg.sender,withDrawERC20Data.tokenAddr,withDrawERC20Data.feeReceiver,withDrawERC20Data.feeAmount,withDrawERC20Data.userAddr,withDrawERC20Data.userAmount,withDrawERC20Data.orderId,withDrawERC20Data.chainId);

    }

    function parseWithDrawERC20Data(bytes calldata data) internal view returns (WithDrawERC20Data memory) {
        (
            address callerAddr,
            address tokenAddr,
            address _feeReceiver,
            uint256 feeAmount,
            address userAddr,
            uint256 userAmount,
            uint256 orderId,
            uint256 chainId,
            bytes memory signature
        ) = abi.decode(
            data,(
                address,
                address,
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
        require(_feeReceiver == feeReceiver, "UnionBridgeSource: INVALID_FEE_RECEIVER");
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
         bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        WITHDRAWERC20_PERMIT_TYPEHASH,
                        callerAddr,
                        tokenAddr,
                        _feeReceiver,
                        feeAmount,
                        userAddr,
                        userAmount,
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
            feeReceiver:_feeReceiver,
            feeAmount:feeAmount,
            userAddr:userAddr,
            userAmount:userAmount,
            orderId:orderId,
            chainId:chainId
        });
    }



}