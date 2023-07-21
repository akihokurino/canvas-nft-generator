use crate::domain;
use crate::domain::contract::ContractId;
use crate::domain::time::LocalDateTime;
use crate::domain::wallet::{Wallet, WalletAddress};

pub type TokenId = domain::Id<Token>;

#[derive(Clone, Debug)]
pub struct Token {
    pub address: ContractId,
    pub token_id: TokenId,
    pub work_id: String,
    pub owner_address: WalletAddress,
    pub ipfs_hash: String,
    pub name: String,
    pub description: String,
    pub price_eth: Option<f64>,
    pub created_at: LocalDateTime,
}

impl Token {
    pub fn new(
        address: ContractId,
        token_id: TokenId,
        work_id: String,
        ipfs_hash: String,
        name: String,
        description: String,
        wallet: Wallet,
        now: LocalDateTime,
    ) -> Self {
        Self {
            address,
            token_id,
            work_id,
            owner_address: wallet.address,
            ipfs_hash,
            name,
            description,
            price_eth: None,
            created_at: now,
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
