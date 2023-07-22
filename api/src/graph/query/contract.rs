use crate::graph;
use crate::graph::AppContext;
use app::{ddb, domain};
use async_graphql::Object;
use async_graphql::{Context, ID};
use std::collections::HashSet;

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

    async fn contract_multi(
        &self,
        ctx: &Context<'_>,
        addresses: Vec<ID>,
    ) -> graph::Result<Vec<graph::types::contract::Contract>> {
        ctx.authorized()?;

        let contract_repository = ctx.data::<ddb::contract::Repository>()?;

        let mut ids: HashSet<domain::contract::ContractId> = HashSet::new();
        for address in addresses {
            ids.insert(domain::contract::ContractId::from(address.to_string()));
        }

        let contracts = contract_repository.batch_get(&ids).await?;

        Ok(contracts
            .into_iter()
            .map(|v| graph::types::contract::Contract { contract: v })
            .collect())
    }
}
