use crate::graph;
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct WalletQuery;

#[Object]
impl WalletQuery {
    async fn wallet(&self, ctx: &Context<'_>) -> graph::Result<graph::types::wallet::MyWallet> {
        let my_wallet = ctx.data::<app::ethereum::MyWallet>()?;

        Ok(graph::types::wallet::MyWallet {
            my_wallet: my_wallet.clone(),
        })
    }
}
