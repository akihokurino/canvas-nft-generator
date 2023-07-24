use app::domain::contract::ContractId;
use app::errors::AppError;
use app::{ddb, domain};
use async_graphql::dataloader;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Loader {
    pub contract_repository: ddb::contract::Repository,
}

impl Loader {
    pub fn new(contract_repository: ddb::contract::Repository) -> Self {
        Self {
            contract_repository,
        }
    }
}

#[async_trait::async_trait]
impl dataloader::Loader<ContractId> for Loader {
    type Value = domain::contract::Contract;
    type Error = AppError;

    async fn load(
        &self,
        keys: &[ContractId],
    ) -> Result<HashMap<ContractId, Self::Value>, Self::Error> {
        let mut result = HashMap::new();

        for key in keys {
            let contract = self.contract_repository.get(&key).await?;
            result.insert(key.clone(), contract);
        }

        Ok(result)
    }
}
