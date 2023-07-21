use crate::graph;
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct WalletQuery;

#[Object]
impl WalletQuery {
    async fn wallet(&self, ctx: &Context<'_>) -> graph::Result<graph::types::wallet::Wallet> {
        let wallet = ctx.data::<app::ethereum::MyWallet>()?;
        let balance = wallet.get_balance().await?;
        Ok(graph::types::wallet::Wallet {
            address: format!("{:?}", wallet.address),
            balance,
        })
    }
}
