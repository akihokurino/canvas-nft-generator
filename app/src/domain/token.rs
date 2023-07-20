use crate::domain;
use crate::domain::contract::ContractId;
use crate::domain::wallet::{Wallet, WalletAddress};

pub type TokenId = domain::Id<Token>;

#[derive(Clone, Debug)]
pub struct Token {
    pub address: ContractId,
    pub token_id: TokenId,
    pub work_id: String,
    pub owner_address: WalletAddress,
    pub ipfs_hash: String,
    pub price_eth: Option<f64>,
}

impl Token {
    pub fn new(
        address: ContractId,
        token_id: TokenId,
        work_id: String,
        ipfs_hash: String,
        wallet: Wallet,
    ) -> Self {
        Self {
            address,
            token_id,
            work_id,
            owner_address: wallet.address,
            ipfs_hash,
            price_eth: None,
        }
    }

    pub fn update_price(self, price: f64) -> Self {
        Self {
            price_eth: Some(price),
            ..self
        }
    }

    pub fn transfer(self, address: WalletAddress) -> Self {
        Self {
            owner_address: address,
            ..self
        }
    }
}
