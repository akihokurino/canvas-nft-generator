use crate::domain;
use crate::domain::time::LocalDateTime;
use crate::domain::wallet::WalletAddress;

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

#[derive(Clone, Debug)]
pub struct Contract {
    pub address: ContractId,
    pub wallet_address: WalletAddress,
    pub schema: Schema,
    pub network: Network,
    pub abi: String,
    pub auto_sell_ether: Option<f64>,
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
            auto_sell_ether: None,
            created_at: now,
        }
    }

    pub fn setting_auto_sell(self, ether: Option<f64>) -> Self {
        Self {
            auto_sell_ether: ether,
            ..self
        }
    }
}
