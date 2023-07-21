import { expect } from "chai";
import { ethers, upgrades } from "hardhat";

describe("Canvas", function () {
  it("should mint and get token url", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("Canvas"),
      []
    );

    const [owner] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    expect(await contract.tokenURI(1)).to.equal("ipfs://A");

    await contract.mint(owner.address, "B");
    expect(await contract.tokenURI(2)).to.equal("ipfs://B");
  });

  it("should error when mint duplicate token name", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("Canvas"),
      []
    );

    const [owner] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await expect(contract.mint(owner.address, "A")).to.be.revertedWith(
      "already mint"
    );
  });

  it("should get name and symbol", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("Canvas"),
      []
    );

    expect(await contract.name()).to.equal("Canvas");
    expect(await contract.symbol()).to.equal("CS");
  });

  it("should get tokenIdOf", async () => {
    const contract = await upgrades.deployProxy(
      await ethers.getContractFactory("Canvas"),
      []
    );

    const [owner, other] = await ethers.getSigners();

    await contract.mint(owner.address, "A");
    await contract.mint(other.address, "B");

    expect(await contract.tokenIdOf("A")).to.equal(1);
    expect(await contract.tokenIdOf("B")).to.equal(2);
    await expect(contract.tokenIdOf("C")).to.be.revertedWith("not mint");
  });
});
