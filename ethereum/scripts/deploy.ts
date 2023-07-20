import { ethers, upgrades } from "hardhat";
import { HardhatRuntimeEnvironment } from "hardhat/types";

async function main(hre: HardhatRuntimeEnvironment) {
  const contract = await upgrades.deployProxy(
    await ethers.getContractFactory("Canvas", {}),
    []
  );
  console.log("deployed to:", contract.address);

  const receipt = await contract.deployTransaction.wait();
  await ethers.provider.waitForTransaction(receipt.transactionHash, 5);
  await hre.run("verify:verify", {
    address: contract.address,
  });
}

main(require("hardhat")).catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
