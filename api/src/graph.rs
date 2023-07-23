mod directive;
mod errors;
mod mutation;
mod pagination;
mod query;
mod types;

use crate::graph::directive::auth;
use crate::graph::mutation::MutationRoot;
use crate::graph::query::QueryRoot;
use actix_web::HttpRequest;
use app::errors::AppError;
use app::{di, ethereum, AppResult};
use async_graphql::{Context, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use http::HeaderMap;

pub type Result<T> = std::result::Result<T, errors::Error>;

#[async_trait]
pub trait AppContext {
    fn ctx(&self) -> &Context;

    fn authorized(&self) -> AppResult<&Authorized> {
        self.ctx()
            .data_opt::<AppResult<Authorized>>()
            .ok_or_else(|| AppError::un_authorized("署名が不正です"))?
            .as_ref()
            .map_err(|err| err.clone().into())
    }
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
        let my_wallet = di::MY_WALLET.get().await.clone();
        let contract_repository = di::CONTRACT_REPOSITORY.get().await.clone();
        let token_repository = di::TOKEN_REPOSITORY.get().await.clone();
        let sns_adapter = di::SNS_ADAPTER.get().await.clone();
        let nft_app = di::NFT_APP.get().await.clone();
        let canvas = di::CANVAS.get().await.clone();

        let schema = Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .data(my_wallet.clone())
        .data(contract_repository.clone())
        .data(token_repository.clone())
        .data(sns_adapter.clone())
        .data(nft_app.clone())
        .data(canvas.clone())
        .directive(auth)
        .finish();

        HttpHandler { schema }
    }

    pub async fn handle(
        &self,
        http_req: HttpRequest,
        gql_req: GraphQLRequest,
        my_wallet: ethereum::MyWallet,
    ) -> GraphQLResponse {
        let mut gql_req = gql_req.into_inner();
        let headers: HeaderMap = HeaderMap::from_iter(http_req.headers().clone().into_iter());

        let authorized: AppResult<Authorized> =
            if let Some(sig) = headers.get("x-sig").and_then(|v| v.to_str().ok()) {
                my_wallet
                    .verify(sig.to_string())
                    .and_then(|_| Ok(Authorized {}))
            } else {
                Err(AppError::un_authorized("署名が不正です"))
            };

        gql_req = gql_req.data(authorized);

        self.schema.execute(gql_req).await.into()
    }
}

#[derive(Debug, Clone)]
pub struct Authorized {}
