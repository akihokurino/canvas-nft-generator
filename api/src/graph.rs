mod errors;
mod mutation;
mod query;
mod types;

use crate::graph::mutation::MutationRoot;
use crate::graph::query::QueryRoot;
use actix_web::HttpRequest;
use app::errors::AppError;
use async_graphql::{Context, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use http::HeaderMap;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[async_trait]
pub trait AppContext {
    fn ctx(&self) -> &Context;
}

impl<'a> AppContext for Context<'_> {
    fn ctx(&self) -> &Context {
        &self
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone)]
pub struct HttpHandler {
    pub schema: Schema,
}

impl HttpHandler {
    pub async fn new() -> Self {
        let schema = Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .finish();

        HttpHandler { schema }
    }

    pub async fn handle(&self, http_req: HttpRequest, gql_req: GraphQLRequest) -> GraphQLResponse {
        let mut gql_req = gql_req.into_inner();

        let headers: HeaderMap = HeaderMap::from_iter(http_req.headers().clone().into_iter());

        gql_req = gql_req.data(if let Some(hv) = headers.get("authorization") {
            Ok("")
        } else {
            Err(AppError::auth_error())
        });

        self.schema.execute(gql_req).await.into()
    }
}
