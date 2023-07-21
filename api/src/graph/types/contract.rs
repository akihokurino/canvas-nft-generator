use crate::graph;
use crate::graph::types::DateTime;
use app::domain;
use async_graphql::ID;

#[derive(Copy, Clone, Eq, PartialEq, Debug, async_graphql::Enum)]
#[graphql(remote = "domain::contract::Schema")]
pub enum Schema {
    ERC721,
    ERC1155,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, async_graphql::Enum)]
#[graphql(remote = "domain::contract::Network")]
pub enum Network {
    Avalanche,
}

pub struct Contract {
    pub contract: domain::contract::Contract,
}

#[async_graphql::Object]
impl Contract {
    async fn address(&self) -> graph::Result<ID> {
        Ok(self.contract.address.to_string().into())
    }

    async fn wallet_address(&self) -> graph::Result<String> {
        Ok(<domain::contract::WalletAddress as Into<String>>::into(
            self.contract.wallet_address.clone(),
        ))
    }

    async fn schema(&self) -> graph::Result<Schema> {
        Ok(self.contract.schema.clone().into())
    }

    async fn network(&self) -> graph::Result<Network> {
        Ok(self.contract.network.clone().into())
    }

    async fn auto_sell_ether(&self) -> graph::Result<Option<f64>> {
        Ok(self.contract.auto_sell_ether.clone())
    }

    async fn created_at(&self) -> graph::Result<DateTime> {
        Ok(self.contract.created_at.clone().into())
    }
}
