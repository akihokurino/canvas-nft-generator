use crate::ddb::{
    condition_eq, AttributeStringValue, AttributeValueResolver, MustPresent, PagingKey, PrimaryKey,
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
            auto_sell_ether: v.get_map("autoSellEther", |v| {
                v.map_or(Ok(None), |v| {
                    let tmp = f64::from_str(v.as_n().map_err(|_| "not a number".to_string())?);
                    tmp.map_err(|err| err.to_string()).map(Some)
                })
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
                Some(
                    <WalletAddress as Into<String>>::into(self.wallet_address).to_attribute_value(),
                ),
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
                "autoSellEther".to_string(),
                self.auto_sell_ether
                    .map(|v| AttributeValue::N(v.to_string())),
            ),
            (
                "createdAt".to_string(),
                Some(self.created_at.to_attribute_value()),
            ),
            (
                "glk".to_string(),
                Some(ContractId::typename().to_attribute_value()),
            ),
        ]
        .into_iter()
        .flat_map(|(k, v)| v.map(|v| (k.to_string(), v)))
        .chain(self.address.key_tuples())
        .collect()
    }
}

pub struct Repository {
    pub cli: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(cli: aws_sdk_dynamodb::Client) -> Self {
        Self { cli }
    }

    pub async fn get_with_pager(
        &self,
        key: &PagingKey,
        limit: &Option<i32>,
    ) -> AppResult<(Vec<Contract>, PagingKey)> {
        let res = self
            .cli
            .query()
            .index_name("glk-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "glk".to_string(),
                condition_eq(ContractId::typename().to_attribute_value()),
            )])))
            .set_exclusive_start_key(key.val.to_owned())
            .limit(limit.unwrap_or(20))
            .scan_index_forward(false)
            .table_name(TABLE_NAME)
            .send()
            .await?;

        Ok((
            res.items
                .unwrap_or_default()
                .into_iter()
                .map(|v| Contract::try_from(v))
                .collect::<AppResult<Vec<_>>>()?,
            PagingKey::from(res.last_evaluated_key),
        ))
    }

    pub async fn get_latest(&self, schema: &Schema) -> AppResult<Contract> {
        let res = self
            .cli
            .query()
            .index_name("schema-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "schema".to_string(),
                condition_eq(schema.to_string().to_attribute_value()),
            )])))
            .limit(1)
            .scan_index_forward(false)
            .table_name(TABLE_NAME)
            .send()
            .await?;

        let items = res.items().unwrap_or_default();
        if items.is_empty() {
            return Err(AppError::not_found());
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

        res.item
            .map_or(Err(AppError::not_found()), |v| Ok(Contract::try_from(v)?))
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