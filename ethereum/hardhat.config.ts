import "@nomiclabs/hardhat-etherscan";
import "@nomiclabs/hardhat-waffle";
import "@openzeppelin/hardhat-upgrades";
import dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";

dotenv.config();

const chainIds = {
  fuji: 43113,
  hardhat: 31337,
};

const config: HardhatUserConfig = {
  solidity: "0.8.17",
  networks: {
    hardhat: {
      chainId: chainIds.hardhat,
    },
    fuji: {
      url: process.env.AVALANCHE_CHAIN_URL!,
      accounts: [process.env.WALLET_SECRET!],
      chainId: chainIds.fuji,
    },
  },
  etherscan: {
    apiKey: {
      fuji: process.env.SNOW_TRACE_API_KEY!,
    },
    customChains: [
      {
        network: "fuji",
        chainId: chainIds.fuji,
        urls: {
          apiURL: "https://api-testnet.snowtrace.io/api",
          browserURL: "https://testnet.snowtrace.io",
        },
      },
    ],
  },
};

export default config;
