// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../bug_report.sol";

interface DAO {
    function totalSupply() external view returns (uint256);
}

contract DAOTest is Test, BugReport {
    string  rpcUrl;
    address targetAddr;
    uint256 forkId;
    uint256 preDaoBalance;

    function setUp() public {
        rpcUrl = vm.envString("MAINNET_RPC_URL");
        targetAddr = vm.envAddress("TARGET");
        forkId = vm.createFork(rpcUrl, vm.envUint("block_num"));
    }

    function pre_inv() internal {
        vm.selectFork(forkId);
        preDaoBalance = address(targetAddr).balance;
    }

    function post_inv() internal {
        uint256 postBalance = address(targetAddr).balance;
        assertEq(
            postBalance,
            preDaoBalance,
            "DAO balance was drained!"
        );
    }

    function testConstraint() public {
        pre_inv();
        bug(forkId, targetAddr);
        post_inv();
    }

    function testTotalSupplyIsNonzero() public {
        bug(forkId, targetAddr);
        uint256 supply = DAO(targetAddr).totalSupply();
        assertGt(supply, 0);
    }
}