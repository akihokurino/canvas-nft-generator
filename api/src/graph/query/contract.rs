use crate::graph;
use crate::graph::AppContext;
use app::{ddb, domain};
use async_graphql::connection::{Connection, Edge};
use async_graphql::Object;
use async_graphql::{Context, ID};

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
        ctx.authorized()?;

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

    async fn contract(
        &self,
        ctx: &Context<'_>,
        address: ID,
    ) -> graph::Result<graph::types::contract::Contract> {
        ctx.authorized()?;

        let contract_repository = ctx.data::<ddb::contract::Repository>()?;

        let contract = contract_repository
            .get(&domain::contract::ContractId::from(address.to_string()))
            .await?;

        Ok(graph::types::contract::Contract { contract })
    }
}
