use crate::graph;
use crate::graph::AppContext;
use app::{ddb, domain};
use async_graphql::Context;
use async_graphql::Object;

#[derive(Default)]
pub struct TokenQuery;

#[Object]
impl TokenQuery {
    async fn token(
        &self,
        ctx: &Context<'_>,
        address: String,
        token_id: String,
    ) -> graph::Result<graph::types::token::Token> {
        ctx.authorized()?;

        let token_repository = ctx.data::<ddb::token::Repository>()?;

        let token = token_repository
            .get(
                &domain::contract::ContractId::from(address),
                &domain::token::TokenId::from(token_id),
            )
            .await?;

        Ok(graph::types::token::Token { token })
    }
}
