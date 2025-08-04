// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin/contracts-upgradeable/token/ERC20/IERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/AccessControlEnumerableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";


contract LPStaking is
    Initializable,
    AccessControlEnumerableUpgradeable,
    ReentrancyGuardUpgradeable,
    UUPSUpgradeable
{
    using SafeERC20Upgradeable for IERC20Upgradeable;

    bytes32 public constant MANAGE_ROLE = keccak256("MANAGE_ROLE");

    bytes32 public constant OPERATE_ROLE = keccak256("OPERATE_ROLE");

    mapping(uint => uint) public stakeTypeNumber;

    bool private funcSwitch;

    IERC20Upgradeable private lpToken;
    address private signer;
    bytes32 public DOMAIN_SEPARATOR;

    struct StakeOrder {
        address caller;
        uint256 amount;
        uint256  stakingTime;
        uint256 stakingLockTime;
        uint256 stakingId;
        uint8 renewable;
        uint8 status;
        uint256 renewTime;
    }
    struct RenewOrder {
        uint256 stakingId;
        uint256 renewTime;
        uint256 blockTime;
    }
    mapping(address => mapping(uint256 => StakeOrder)) public userOrders;
    mapping(address => uint256[]) userOrderIds;
    mapping(address => RenewOrder[]) public userRenewOrders;

     bytes32 private constant STAKE_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(address caller,uint256 amount,uint256 stakingTime,uint256 stakingLockTime,uint256 stakingId,uint8 renewable,uint8 status)"
            )
        );
    
    bytes32 private constant RENEW_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 orderId,uint256 newEndTimestamp)"
            )
        );
    bytes32 private constant BEBACK_TYPEHASH =
        keccak256(
            abi.encodePacked(
                "Permit(uint256 stakingId)"
            )
        );

    event StakeLP(address caller,address lpToken,uint256 stakingId,uint256 amount,uint256 stakingTime,uint256 stakingLockTime,uint8 renewable);
    event ReStakingLP(address caller, uint256 stakingId, uint256 renewTime);
    event BetBackStaking(address caller, uint256 stakingId, uint256 amount);

    function initialize(address _LPToken,address _signer) public initializer {
        __AccessControlEnumerable_init();
        __ReentrancyGuard_init();
        __UUPSUpgradeable_init();

        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(MANAGE_ROLE, msg.sender);

        lpToken = IERC20Upgradeable(_LPToken);
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
                keccak256(bytes("LPStaking")),
                keccak256(bytes("1")),
                chainId,
                address(this)
            )
        );
    }
    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function _authorizeUpgrade(
        address newImplementation
    ) internal override onlyRole(MANAGE_ROLE) {}

    function balance(address token) public view returns (uint256) {
        if (token == address(0)) {
            return address(this).balance;
        }
        return IERC20Upgradeable(token).balanceOf(address(this));
    }

    function setFuncSwith(bool _funcSwitch) public onlyRole(MANAGE_ROLE) {
        funcSwitch = _funcSwitch;
    }

    function stakeLP(bytes memory data) public payable nonReentrant {
        StakeOrder memory stakeOrder  =  parseStakeOrder(data);
        require(stakeOrder.amount > 0, "LPStaking: amount error");
        require(stakeOrder.caller == msg.sender,"LPStaking: caller error");
        require(userOrders[msg.sender][stakeOrder.stakingId].stakingId == 0, "LPStaking: ORDER_EXISTS");
        lpToken.safeTransferFrom(msg.sender, address(this), stakeOrder.amount);
        userOrders[msg.sender][stakeOrder.stakingId] = stakeOrder;
        userOrderIds[msg.sender].push(stakeOrder.stakingId);

        emit StakeLP(msg.sender,address(lpToken),stakeOrder.stakingId,stakeOrder.amount,stakeOrder.stakingTime,stakeOrder.stakingLockTime,stakeOrder.renewable);
    }

    function parseStakeOrder(bytes memory data) internal view returns (StakeOrder memory) {
        (
            address caller,
            uint256 amount,
            uint256  stakingTime,
            uint256 stakingLockTime,
            uint256 stakingId,
            uint8 renewable,
            bytes memory signature
        ) = abi.decode(
            data,(address,uint256,uint256,uint256,uint256,uint8,bytes)
        );
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        STAKE_TYPEHASH,
                        caller,
                        amount,
                        stakingTime,
                        stakingLockTime,
                        stakingId,
                        renewable
                    )
                )
            )
        );
        require(signer == ecrecover(signHash, v, r, s),"LPStaking:INVALID_REQUEST");
        return StakeOrder({
            caller:caller,
            amount:amount,
            stakingTime:stakingTime,
            stakingLockTime:stakingLockTime,
            stakingId:stakingId,
            renewable:renewable,
            status:0,
            renewTime:0
        });
    }
    function splitSignature(
        bytes memory sig
    ) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        require(sig.length == 65, "EIP712: invalid signature length");
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
    }

    function reStakingLP(bytes memory data) public nonReentrant {
        RenewOrder memory renewOrder = parseRenewOrder(data);
        require(userOrders[msg.sender][renewOrder.stakingId].stakingId != 0, "LPStaking: no staking");
        require(userOrders[msg.sender][renewOrder.stakingId].status == 0, "LPStaking: no staking status");
        require(userOrders[msg.sender][renewOrder.stakingId].stakingLockTime <= block.timestamp,"LPStaking:stakingLockTime is more than block time");
        require(userOrders[msg.sender][renewOrder.stakingId].renewable == 1, "LPStaking: order is not renewable");
        if (userOrders[msg.sender][renewOrder.stakingId].renewTime > 0) {
            require(
                renewOrder.renewTime > block.timestamp &&
                    renewOrder.renewTime > userOrders[msg.sender][renewOrder.stakingId].renewTime,
                "LPStaking: renewTime is invalid"
            );
        } else {
            require(
                renewOrder.renewTime > block.timestamp &&
                    renewOrder.renewTime > userOrders[msg.sender][renewOrder.stakingId].stakingLockTime,
                "LPStaking: renewTime is invalid"
            );
        }
        userOrders[msg.sender][renewOrder.stakingId].renewTime = renewOrder.renewTime;
        userRenewOrders[msg.sender].push(
            RenewOrder({
                stakingId: renewOrder.stakingId,
                renewTime: renewOrder.renewTime,
                blockTime: block.timestamp
            })
        );
        emit ReStakingLP(msg.sender, renewOrder.stakingId, renewOrder.renewTime);
    }

    function parseRenewOrder(bytes memory data) internal view returns(RenewOrder memory) {
        (
            uint256 stakingId,
            uint256 renewTime,
            bytes memory signature
        ) = abi.decode(
                data,
                (
                    uint256,
                    uint256,
                    bytes
                )
            );
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        RENEW_TYPEHASH,
                        stakingId,
                        renewTime
                    )
                )
            )
        );

        require(signer == ecrecover(signHash, v, r, s));
        return RenewOrder({
            stakingId:stakingId,
            renewTime:renewTime,
            blockTime:block.timestamp
        });
    }

    function betBackStaking(bytes memory data) public nonReentrant {
        (uint256 stakingId,bytes memory signature) = abi.decode(data,(uint256, bytes));
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        bytes32 signHash = keccak256(
            abi.encodePacked(
                "\x19\x01",
                DOMAIN_SEPARATOR,
                keccak256(
                    abi.encode(
                        BEBACK_TYPEHASH,
                        stakingId
                    )
                )
            )
        );
        require( signer == ecrecover(signHash, v, r, s),"LPStaking:INVALID_REQUEST");
        require(userOrders[msg.sender][stakingId].stakingId != 0, "LPStaking: no staking");
        require(userOrders[msg.sender][stakingId].status == 0, "LPStaking: order is invalid");
        if (userOrders[msg.sender][stakingId].renewTime > 0) {
            require(
                userOrders[msg.sender][stakingId].renewTime <= block.timestamp,
                "LPStaking: newEndTimestamp is invalid"
            );
        } else {
            require(
                userOrders[msg.sender][stakingId].stakingLockTime <= block.timestamp,
                "LPStaking: newEndTimestamp is invalid"
            );
        }

        userOrders[msg.sender][stakingId].status = 1;
        (bool success, ) = payable(msg.sender).call{value: userOrders[msg.sender][stakingId].amount}("");
        require(success, "Transfer failed");
        emit BetBackStaking(msg.sender, stakingId, userOrders[msg.sender][stakingId].amount);
    }




}