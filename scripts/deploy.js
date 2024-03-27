//  Script to deploy smart contracts

const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with the account:", deployer.address);

  const Token = await ethers.getContractFactory("Token");
  console.log("Deploying Token...");
  const token = await Token.deploy();
  console.log("Token address:", token.address);

  const DataVaultX = await ethers.getContractFactory("DataVaultX");
  console.log("Deploying DataVaultX...");
  const dataVaultX = await DataVaultX.deploy();
  console.log("DataVaultX address:", dataVaultX.address);
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
