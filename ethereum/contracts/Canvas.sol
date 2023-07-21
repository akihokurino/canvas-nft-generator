// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";

contract Canvas is ERC721Upgradeable, OwnableUpgradeable {
    mapping(string => uint256) private _hash2token;
    mapping(uint256 => string) private _token2hash;
    uint256 private _tokenId;

    function initialize() public initializer {
        __ERC721_init("Canvas", "CS");
        __Ownable_init();
        _tokenId = 1;
    }

    function mint(address to, string memory ipfsHash) public virtual onlyOwner {
        require(_hash2token[ipfsHash] == 0, "already mint");

        uint256 tokenId = _tokenId;
        _hash2token[ipfsHash] = tokenId;
        _token2hash[tokenId] = ipfsHash;
        _mint(to, tokenId);
        _tokenId += 1;
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        string memory ipfsHash = _token2hash[tokenId];
        return string(abi.encodePacked("ipfs://", ipfsHash));
    }

    function tokenIdOf(
        string memory ipfsHash
    ) public view virtual returns (uint256) {
        uint256 tokenId = _hash2token[ipfsHash];
        if (tokenId == 0) {
            revert("not mint");
        }
        return tokenId;
    }
}
