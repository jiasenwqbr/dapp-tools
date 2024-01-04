// SPDX-License-Identifier: Unlicense
pragma solidity ^0.8.13;
import "forge-std/Test.sol";
contract ContractBTest is Test {
    uint256 testNumber;
    function setUp() public {
        testNumber = 42;
    }
    function test_NumberIs42() public {
        assertEq(testNumber,42);
    }
    function testFaild_Subtract43() public {
        testNumber -= 43;
    }
    function testSuccess_Subtract43() public {
        testNumber -= 41;
    }
}