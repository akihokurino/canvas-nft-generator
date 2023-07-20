mod contract;

use crate::graph::mutation::contract::ContractMutation;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct MutationRoot(ContractMutation);
