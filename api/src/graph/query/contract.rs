use crate::graph;
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct ContractQuery;

#[Object]
impl ContractQuery {
    async fn contracts(
        &self,
        ctx: &Context<'_>,
    ) -> graph::Result<Vec<graph::types::contract::Contract>> {
        Ok(vec![graph::types::contract::Contract {
            address: "TODO".into(),
        }])
    }
}
