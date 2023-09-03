// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/MyContract.sol";

contract ContractTest is Test {
    MyContract drop;
    // make addr is a helper function to create an address from a string
    address testAddr = makeAddr("Test");

    function setUp() public {
        drop = new MyContract(
            testAddr,
            "MintTest",
            "MT",
            testAddr,
            500,
            testAddr
        );
    }

    function testDropWithZeroTokens() public returns (uint256) {
        vm.expectRevert("You must mint at least one token!");
        drop.mint(testAddr, 0);
        assertEq(drop.balanceOf(testAddr), 0);
        return 0;
    }
    
    function testDropWithTwoTokens() public returns (uint256) {
        drop.mint(testAddr, 2);
        assertEq(drop.balanceOf(testAddr), 2);
        return 2;
    }

    function testDropWithCustomTokens(uint256 amount) public returns (uint256) {
        drop.mint(testAddr, amount);
        assertEq(drop.balanceOf(testAddr), amount);
        return amount;
    }
}
