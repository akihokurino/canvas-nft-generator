use crate::graph;
use crate::graph::AppContext;
use app::{ddb, domain};
use async_graphql::Object;
use async_graphql::{Context, ID};

#[derive(Default)]
pub struct ContractQuery;

#[Object]
impl ContractQuery {
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
