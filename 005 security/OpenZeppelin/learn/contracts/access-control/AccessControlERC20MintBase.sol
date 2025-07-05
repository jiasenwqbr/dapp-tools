// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {AccessControl} from "@openzeppelin/contracts/access/AccessControl.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract AccessControlERC20MintBase is ERC20,AccessControl {
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant BURNER_ROLE = keccak256("BURNER_ROLE");

    error CallerNotMinter(address caller);
    constructor(address miner,address burner) ERC20("MyToken", "TKN"){
        //_grantRole(MINTER_ROLE,miner);
        //_grantRole(BURNER_ROLE,burner);
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function mint(address to,uint256 amount) public {
        if (!hasRole(MINTER_ROLE,msg.sender)){
            revert CallerNotMinter(msg.sender);
        }
        _mint(to,amount);
    }

    function burn(address from, uint256 amount) public onlyRole(BURNER_ROLE) {
        _burn(from, amount);
    }
}