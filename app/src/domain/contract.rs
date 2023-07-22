use crate::domain;
use crate::domain::time::LocalDateTime;
use derive_more::{From, Into};

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Schema {
    ERC721,
    ERC1155,
}

#[derive(PartialEq, Clone, Debug, Copy, strum_macros::EnumString, strum_macros::Display)]
pub enum Network {
    Avalanche,
}

pub type ContractId = domain::Id<Contract>;

#[derive(From, Into, Clone, Debug)]
pub struct WalletAddress(pub String);

#[derive(Clone, Debug)]
pub struct Contract {
    pub address: ContractId,
    pub wallet_address: WalletAddress,
    pub schema: Schema,
    pub network: Network,
    pub abi: String,
    pub created_at: LocalDateTime,
}

impl Contract {
    pub fn new(
        address: ContractId,
        wallet_address: WalletAddress,
        abi: String,
        now: LocalDateTime,
    ) -> Self {
        Self {
            address,
            wallet_address,
            schema: Schema::ERC721,
            network: Network::Avalanche,
            abi,
            created_at: now,
        }
    }
}
