use crate::graph;
use crate::graph::types::DateTime;
use app::domain::contract::WalletAddress;
use app::{di, domain, ethereum};
use async_graphql::{Context, ID};

pub struct Token {
    pub token: domain::token::Token,
}

#[async_graphql::Object]
impl Token {
    async fn id(&self) -> graph::Result<ID> {
        Ok(format!(
            "{}#{}",
            self.token.address.to_string(),
            self.token.token_id.to_string()
        )
        .into())
    }

    async fn address(&self) -> graph::Result<String> {
        Ok(self.token.address.to_string())
    }

    async fn token_id(&self) -> graph::Result<String> {
        Ok(self.token.token_id.to_string())
    }

    async fn work_id(&self) -> graph::Result<String> {
        Ok(self.token.work_id.clone())
    }

    async fn image_url(&self) -> graph::Result<String> {
        Ok(format!(
            "{}/ipfs/{}",
            di::IPFS_GATEWAY.clone(),
            self.token.ipfs_image_hash.clone()
        ))
    }

    async fn name(&self) -> graph::Result<String> {
        Ok(self.token.name.clone())
    }

    async fn description(&self) -> graph::Result<String> {
        Ok(self.token.description.clone())
    }

    async fn price_eth(&self) -> graph::Result<Option<f64>> {
        Ok(self.token.price_eth.clone())
    }

    async fn is_owner(&self, ctx: &Context<'_>) -> graph::Result<bool> {
        let my_wallet = ctx.data::<ethereum::MyWallet>()?;
        Ok(WalletAddress::from(my_wallet.address.clone()) == self.token.owner_address)
    }

    async fn created_at(&self) -> graph::Result<DateTime> {
        Ok(self.token.created_at.clone().into())
    }
}
