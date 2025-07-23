// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract UAC is ERC20, Ownable {
    constructor(address receiver) ERC20("UAC", "UAC") {
        _mint(receiver, 200_000_000_000_000_000 * 10 ** decimals());
    }
}
