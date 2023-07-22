mod contract;
mod token;
mod wallet;

use crate::graph::query::contract::ContractQuery;
use crate::graph::query::token::TokenQuery;
use crate::graph::query::wallet::WalletQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(WalletQuery, ContractQuery, TokenQuery);
