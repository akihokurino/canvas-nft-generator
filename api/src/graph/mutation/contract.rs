use crate::graph;
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct ContractMutation;

#[Object]
impl ContractMutation {
    async fn contract_create(&self, ctx: &Context<'_>) -> graph::Result<bool> {
        Ok(true)
    }
}
