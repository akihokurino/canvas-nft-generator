mod contract;
mod wallet;

use crate::graph::query::contract::ContractQuery;
use crate::graph::query::wallet::WalletQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(WalletQuery, ContractQuery);
