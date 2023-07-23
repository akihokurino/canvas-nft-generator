use crate::graph;
use crate::graph::types::DateTime;
use crate::graph::AppContext;
use app::{ddb, domain, ethereum};
use async_graphql::connection::{Connection, Edge};
use async_graphql::{Context, ID};

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

    async fn name(&self, ctx: &Context<'_>) -> graph::Result<String> {
        let canvas = ctx.data::<ethereum::canvas::Canvas>()?;
        let name = canvas.name(&self.contract).await?;
        Ok(name)
    }

    async fn schema(&self) -> graph::Result<Schema> {
        Ok(self.contract.schema.clone().into())
    }

    async fn network(&self) -> graph::Result<Network> {
        Ok(self.contract.network.clone().into())
    }

    async fn created_at(&self) -> graph::Result<DateTime> {
        Ok(self.contract.created_at.clone().into())
    }

    async fn tokens(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> graph::Result<Connection<String, graph::types::token::Token>> {
        ctx.authorized()?;

        let token_repository = ctx.data::<ddb::token::Repository>()?;

        let paging = graph::pagination::Pagination::calc(after, before, first, last, 20)?;

        let items = token_repository
            .get_by_contract_with_pager(
                &self.contract.address,
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
                        graph::types::token::Token { token: v.entity },
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }

    async fn stock_tokens(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> graph::Result<Connection<String, graph::types::token::Token>> {
        ctx.authorized()?;

        let token_repository = ctx.data::<ddb::token::Repository>()?;

        let paging = graph::pagination::Pagination::calc(after, before, first, last, 20)?;

        let items = token_repository
            .get_stock_by_contract_with_pager(
                &self.contract.address,
                &self.contract.wallet_address,
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
                        graph::types::token::Token { token: v.entity },
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }

    async fn sell_order_tokens(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> graph::Result<Connection<String, graph::types::token::Token>> {
        ctx.authorized()?;

        let token_repository = ctx.data::<ddb::token::Repository>()?;

        let paging = graph::pagination::Pagination::calc(after, before, first, last, 20)?;

        let items = token_repository
            .get_sell_order_by_contract_with_pager(
                &self.contract.address,
                &self.contract.wallet_address,
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
                        graph::types::token::Token { token: v.entity },
                    )
                })
                .collect::<Vec<_>>(),
        ))
    }
}
