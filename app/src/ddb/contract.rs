use crate::ddb::{
    condition_eq, condition_start_from, AttributeStringValue, AttributeValueResolver, Cursor,
    EntityWithCursor, MustPresent, PrimaryKey,
};
use crate::domain::contract::{Contract, ContractId, Network, Schema, WalletAddress};
use crate::domain::time::LocalDateTime;
use crate::errors::AppError;
use crate::AppResult;
use aws_sdk_dynamodb::types::{AttributeValue, KeysAndAttributes};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const TABLE_NAME: &str = "cng-contract";

impl PrimaryKey for ContractId {
    fn typename() -> String {
        "contract".to_string()
    }

    fn raw(&self) -> String {
        self.to_string()
    }

    fn from(from: String) -> Self {
        ContractId::new(from)
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Contract {
    type Error = AppError;

    fn try_from(v: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Contract {
            address: v.get_map("pk", |v| {
                v.must_present()
                    .and_then(ContractId::try_from_attribute_value)
            })?,
            wallet_address: v.get_map("walletAddress", |v| {
                let raw = v
                    .must_present()
                    .and_then(String::try_from_attribute_value)?;
                Ok(WalletAddress::from(raw))
            })?,
            schema: v.get_map("schema", |v| {
                let raw = v
                    .must_present()
                    .and_then(String::try_from_attribute_value)?;
                Schema::from_str(raw.as_str()).map_err(|_| "parse error".into())
            })?,
            network: v.get_map("network", |v| {
                let raw = v
                    .must_present()
                    .and_then(String::try_from_attribute_value)?;
                Network::from_str(raw.as_str()).map_err(|_| "parse error".into())
            })?,
            abi: v.get_map("abi", |v| {
                v.must_present().and_then(String::try_from_attribute_value)
            })?,
            created_at: v.get_map("createdAt", |v| {
                v.must_present()
                    .and_then(LocalDateTime::try_from_attribute_value)
            })?,
        })
    }
}

impl Into<HashMap<String, AttributeValue>> for Contract {
    fn into(self) -> HashMap<String, AttributeValue> {
        [
            (
                "walletAddress".to_string(),
                Some(self.wallet_address.to_string().to_attribute_value()),
            ),
            (
                "schema".to_string(),
                Some(self.schema.to_string().to_attribute_value()),
            ),
            (
                "network".to_string(),
                Some(self.network.to_string().to_attribute_value()),
            ),
            ("abi".to_string(), Some(self.abi.to_attribute_value())),
            (
                "createdAt".to_string(),
                Some(self.created_at.to_attribute_value()),
            ),
            (
                "glk".to_string(),
                Some(ContractId::typename().to_attribute_value()),
            ),
            (
                "walletAddress_schema".to_string(),
                Some(
                    wallet_address_with_schema(self.wallet_address.clone(), self.schema.clone())
                        .to_attribute_value(),
                ),
            ),
        ]
        .into_iter()
        .flat_map(|(k, v)| v.map(|v| (k.to_string(), v)))
        .chain(self.address.key_tuples())
        .collect()
    }
}

fn wallet_address_with_schema(wallet_address: WalletAddress, schema: Schema) -> String {
    format!("{}_{}", wallet_address.to_string(), schema.to_string())
}

#[derive(Clone, Debug)]
pub struct Repository {
    pub cli: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(cli: aws_sdk_dynamodb::Client) -> Self {
        Self { cli }
    }

    pub async fn get_all(&self) -> AppResult<Vec<Contract>> {
        let res = self
            .cli
            .query()
            .index_name("glk-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "glk".to_string(),
                condition_eq(ContractId::typename().to_attribute_value()),
            )])))
            .table_name(TABLE_NAME)
            .send()
            .await?;

        res.items
            .unwrap_or_default()
            .into_iter()
            .map(|v| Contract::try_from(v))
            .collect::<AppResult<Vec<_>>>()
    }

    pub async fn get_by_wallet_address_with_pager(
        &self,
        wallet_address: &WalletAddress,
        cursor: Option<Cursor>,
        limit: i32,
        forward: bool,
    ) -> AppResult<Vec<EntityWithCursor<Contract>>> {
        let mut q = self
            .cli
            .query()
            .index_name("walletAddress-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "walletAddress".to_string(),
                condition_eq(wallet_address.to_string().to_attribute_value()),
            )])))
            .limit(limit)
            .scan_index_forward(forward)
            .table_name(TABLE_NAME);

        if let Some(cursor) = cursor {
            q = q.key_conditions("createdAt", condition_start_from(cursor, forward))
        }

        let res = q.send().await?;

        res.items
            .unwrap_or_default()
            .into_iter()
            .map(|v| EntityWithCursor::new(v, |v| Contract::try_from(v)))
            .collect::<AppResult<Vec<EntityWithCursor<Contract>>>>()
    }

    pub async fn get_latest_by_wallet_address_and_schema(
        &self,
        wallet_address: &WalletAddress,
        schema: &Schema,
    ) -> AppResult<Contract> {
        let res = self
            .cli
            .query()
            .index_name("walletAddress_schema-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "walletAddress_schema".to_string(),
                condition_eq(
                    wallet_address_with_schema(wallet_address.clone(), schema.clone())
                        .to_attribute_value(),
                ),
            )])))
            .limit(1)
            .scan_index_forward(false)
            .table_name(TABLE_NAME)
            .send()
            .await?;

        let items = res.items().unwrap_or_default();
        if items.is_empty() {
            return Err(AppError::not_found("データが存在しません"));
        }

        Ok(Contract::try_from(items.first().unwrap().to_owned())?)
    }

    pub async fn batch_get(&self, addresses: &HashSet<ContractId>) -> AppResult<Vec<Contract>> {
        if addresses.is_empty() {
            return Ok(vec![]);
        }

        let res = self
            .cli
            .batch_get_item()
            .request_items(
                TABLE_NAME,
                KeysAndAttributes::builder()
                    .set_keys(Some(addresses.iter().map(|v| v.key_map()).collect()))
                    .build(),
            )
            .send()
            .await?;

        let mut entities: Vec<Contract> = vec![];
        for (table, data) in res.responses.unwrap_or_default() {
            if table == TABLE_NAME {
                for item in data {
                    entities.push(Contract::try_from(item)?)
                }
            }
        }

        Ok(entities)
    }

    pub async fn get(&self, address: &ContractId) -> AppResult<Contract> {
        let res = self
            .cli
            .get_item()
            .table_name(TABLE_NAME)
            .set_key(Some(address.key_map()))
            .send()
            .await?;

        res.item.map_or(
            Err(AppError::not_found("データが存在しません")),
            |v| Ok(Contract::try_from(v)?),
        )
    }

    pub async fn put(&self, item: Contract) -> AppResult<()> {
        self.cli
            .put_item()
            .table_name(TABLE_NAME)
            .set_item(Some(item.into()))
            .send()
            .await?;

        Ok(())
    }
}
