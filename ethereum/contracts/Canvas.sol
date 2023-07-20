// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";

contract Canvas is ERC1155Upgradeable, OwnableUpgradeable {
    mapping(string => uint256) private _hash2token;
    mapping(uint256 => string) private _token2hash;
    uint256 private _tokenId;
    string public name;
    string public symbol;

    function initialize() public initializer {
        __ERC1155_init("");
        __Ownable_init();
        _tokenId = 1;
        name = "Canvas";
        symbol = "CS";
    }

    function mint(
        address to,
        uint256 amount,
        string memory ipfsHash
    ) public virtual onlyOwner {
        require(_hash2token[ipfsHash] == 0, "already mint");

        uint256 tokenId = _tokenId;
        _hash2token[ipfsHash] = tokenId;
        _token2hash[tokenId] = ipfsHash;
        _mint(to, tokenId, amount, "");
        _tokenId += 1;
    }

    function uri(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        string memory ipfsHash = _token2hash[tokenId];
        return string(abi.encodePacked("ipfs://", ipfsHash));
    }

    function isOwn(
        address addr,
        string memory ipfsHash
    ) public view virtual returns (bool) {
        uint256 tokenId = _hash2token[ipfsHash];
        uint256 balance = balanceOf(addr, tokenId);
        return balance != 0;
    }

    function tokenIdOf(
        string memory ipfsHash
    ) public view virtual returns (uint256) {
        uint256 tokenId = _hash2token[ipfsHash];
        if (tokenId == 0) {
            return 0;
        }
        return tokenId;
    }
}
