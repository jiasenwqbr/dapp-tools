// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

abstract contract  EIP712Verifier {
    function getDomainSeparator(
        string memory name,
        string memory version,
        uint256 chainId,
        address verifyingContract
    ) internal pure returns (bytes32) {
        return
            keccak256(
                abi.encode(
                    keccak256(
                        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
                    ),
                    keccak256(bytes(name)),
                    keccak256(bytes(version)),
                    chainId,
                    verifyingContract
                )
            );
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

    function verifySignature(
        bytes32 domainSeparator,
        bytes32 typeHash,
        bytes32[] memory values,
        bytes memory signature,
        address expectedSigner
    ) internal pure returns (bool) {
        bytes32 structHash = keccak256(abi.encodePacked(typeHash, values));
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(signature);
        return ecrecover(digest, v, r, s) == expectedSigner;
    }

    function hashString(string memory s) internal pure returns (bytes32) {
        return keccak256(bytes(s));
    }

    function bytes32ToString(bytes32 _bytes32) public pure returns (string memory) {
    uint8 i = 0;
    while (i < 32 && _bytes32[i] != 0) {
        i++;
    }
    bytes memory bytesArray = new bytes(i);
    for (uint8 j = 0; j < i; j++) {
        bytesArray[j] = _bytes32[j];
    }
    return string(bytesArray);
}
}