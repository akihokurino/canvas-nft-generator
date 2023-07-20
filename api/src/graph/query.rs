mod contract;

use crate::graph::query::contract::ContractQuery;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct QueryRoot(ContractQuery);
