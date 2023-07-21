use crate::graph;
use app::ddb;
use async_graphql::connection::{Connection, Edge};
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct ContractQuery;

#[Object]
impl ContractQuery {
    async fn contracts(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> graph::Result<Connection<String, graph::types::contract::Contract>> {
        let contract_repository = ctx.data::<ddb::contract::Repository>()?;

        let paging = graph::pagination::Pagination::calc(after, before, first, last, 20)?;

        let items = contract_repository
            .get_with_pager(paging.cursor.clone(), paging.limit, paging.forward)
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
