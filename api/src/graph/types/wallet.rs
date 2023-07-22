use crate::graph;
use crate::graph::AppContext;
use app::{ddb, ethereum};
use async_graphql::connection::{Connection, Edge};
use async_graphql::Context;

pub struct MyWallet {
    pub my_wallet: ethereum::MyWallet,
}

#[async_graphql::Object]
impl MyWallet {
    async fn address(&self) -> String {
        format!("{:?}", self.my_wallet.address.clone())
    }

    async fn balance(&self) -> graph::Result<f64> {
        let balance = self.my_wallet.get_balance().await?;
        Ok(balance)
    }

    async fn contracts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> graph::Result<Connection<String, graph::types::contract::Contract>> {
        ctx.authorized()?;

        let contract_repository = ctx.data::<ddb::contract::Repository>()?;

        let paging = graph::pagination::Pagination::calc(after, before, first, last, 20)?;

        let items = contract_repository
            .get_by_wallet_address_with_pager(
                &self.my_wallet.address.into(),
                paging.cursor.clone(),
                paging.limit,
                paging.forward,
            )
            .await?;

        Ok(paging.connection(
            items
                .into_iter()
                .map(|v| {
                    Edge::new(
                        v.cursor.into(),
                        graph::types::contract::Contract { contract: v.entity },
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }
}
