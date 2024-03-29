use crate::ddb::{
    condition_eq, condition_sk_type, condition_start_from, AttributeStringValue,
    AttributeValueResolver, Cursor, EntityWithCursor, MustPresent, PrimaryKey,
};
use crate::domain::contract::{ContractId, WalletAddress};
use crate::domain::time::LocalDateTime;
use crate::domain::token::{Token, TokenId};
use crate::errors::AppError;
use crate::AppResult;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use std::str::FromStr;

const TABLE_NAME: &str = "cng-contract";

impl PrimaryKey for TokenId {
    fn typename() -> String {
        "token".to_string()
    }

    fn raw(&self) -> String {
        self.to_string()
    }

    fn from(from: String) -> Self {
        TokenId::new(from)
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Token {
    type Error = AppError;

    fn try_from(v: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Token {
            address: v.get_map("pk", |v| {
                v.must_present()
                    .and_then(ContractId::try_from_attribute_value)
            })?,
            token_id: v.get_map("sk", |v| {
                v.must_present().and_then(TokenId::try_from_attribute_value)
            })?,
            work_id: v.get_map("workId", |v| {
                v.must_present().and_then(String::try_from_attribute_value)
            })?,
            owner_address: v.get_map("ownerAddress", |v| {
                let raw = v
                    .must_present()
                    .and_then(String::try_from_attribute_value)?;
                Ok(WalletAddress::from(raw))
            })?,
            ipfs_image_hash: v.get_map("ipfsImageHash", |v| {
                v.must_present().and_then(String::try_from_attribute_value)
            })?,
            name: v.get_map("name", |v| {
                v.must_present().and_then(String::try_from_attribute_value)
            })?,
            description: v.get_map("description", |v| {
                v.must_present().and_then(String::try_from_attribute_value)
            })?,
            price_eth: v.get_map("priceEth", |v| {
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

impl Into<HashMap<String, AttributeValue>> for Token {
    fn into(self) -> HashMap<String, AttributeValue> {
        [
            ("pk".to_string(), Some(self.address.to_attribute_value())),
            ("sk".to_string(), Some(self.token_id.to_attribute_value())),
            (
                "workId".to_string(),
                Some(self.work_id.to_attribute_value()),
            ),
            (
                "ownerAddress".to_string(),
                Some(self.owner_address.to_string().to_attribute_value()),
            ),
            (
                "ipfsImageHash".to_string(),
                Some(self.ipfs_image_hash.to_attribute_value()),
            ),
            ("name".to_string(), Some(self.name.to_attribute_value())),
            (
                "description".to_string(),
                Some(self.description.to_attribute_value()),
            ),
            (
                "priceEth".to_string(),
                self.price_eth.map(|v| AttributeValue::N(v.to_string())),
            ),
            (
                "createdAt".to_string(),
                Some(self.created_at.to_attribute_value()),
            ),
            (
                "glk".to_string(),
                Some(TokenId::typename().to_attribute_value()),
            ),
        ]
        .into_iter()
        .flat_map(|(k, v)| v.map(|v| (k.to_string(), v)))
        .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Repository {
    pub cli: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(cli: aws_sdk_dynamodb::Client) -> Self {
        Self { cli }
    }

    pub async fn get_all_by_contract(&self, address: &ContractId) -> AppResult<Vec<Token>> {
        let res = self
            .cli
            .query()
            .set_key_conditions(Some(HashMap::from([
                ("pk".to_string(), condition_eq(address.to_attribute_value())),
                ("sk".to_string(), condition_sk_type::<TokenId>()),
            ])))
            .table_name(TABLE_NAME)
            .send()
            .await?;

        res.items
            .unwrap_or_default()
            .into_iter()
            .map(|v| Token::try_from(v))
            .collect::<AppResult<Vec<_>>>()
    }

    pub async fn get_by_contract_with_pager(
        &self,
        address: &ContractId,
        cursor: Option<Cursor>,
        limit: i32,
        forward: bool,
    ) -> AppResult<Vec<EntityWithCursor<Token>>> {
        let mut q = self
            .cli
            .query()
            .index_name("pk-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "pk".to_string(),
                condition_eq(address.to_attribute_value()),
            )])))
            .filter_expression("glk = :glk")
            .expression_attribute_values(":glk", TokenId::typename().to_attribute_value())
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
            .map(|v| EntityWithCursor::new(v, |v| Token::try_from(v), "createdAt"))
            .collect::<AppResult<Vec<EntityWithCursor<Token>>>>()
    }

    pub async fn get_stock_by_contract_with_pager(
        &self,
        address: &ContractId,
        wallet_address: &WalletAddress,
        cursor: Option<Cursor>,
        limit: i32,
        forward: bool,
    ) -> AppResult<Vec<EntityWithCursor<Token>>> {
        let mut q = self
            .cli
            .query()
            .index_name("pk-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "pk".to_string(),
                condition_eq(address.to_attribute_value()),
            )])))
            .filter_expression(
                "glk = :glk AND ownerAddress = :ownerAddress AND attribute_not_exists(priceEth)",
            )
            .expression_attribute_values(":glk", TokenId::typename().to_attribute_value())
            .expression_attribute_values(
                ":ownerAddress",
                wallet_address.to_string().to_attribute_value(),
            )
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
            .map(|v| EntityWithCursor::new(v, |v| Token::try_from(v), "createdAt"))
            .collect::<AppResult<Vec<EntityWithCursor<Token>>>>()
    }

    pub async fn get_sell_order_by_contract_with_pager(
        &self,
        address: &ContractId,
        wallet_address: &WalletAddress,
        cursor: Option<Cursor>,
        limit: i32,
        forward: bool,
    ) -> AppResult<Vec<EntityWithCursor<Token>>> {
        let mut q = self
            .cli
            .query()
            .index_name("pk-createdAt-index")
            .set_key_conditions(Some(HashMap::from([(
                "pk".to_string(),
                condition_eq(address.to_attribute_value()),
            )])))
            .filter_expression(
                "glk = :glk AND ownerAddress = :ownerAddress AND attribute_exists(priceEth)",
            )
            .expression_attribute_values(":glk", TokenId::typename().to_attribute_value())
            .expression_attribute_values(
                ":ownerAddress",
                wallet_address.to_string().to_attribute_value(),
            )
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
            .map(|v| EntityWithCursor::new(v, |v| Token::try_from(v), "createdAt"))
            .collect::<AppResult<Vec<EntityWithCursor<Token>>>>()
    }

    pub async fn get_by_ipfs_image_hash(
        &self,
        address: &ContractId,
        ipfs_image_hash: String,
    ) -> AppResult<Token> {
        let res = self
            .cli
            .query()
            .index_name("pk-ipfsImageHash-index")
            .set_key_conditions(Some(HashMap::from([
                ("pk".to_string(), condition_eq(address.to_attribute_value())),
                (
                    "ipfsImageHash".to_string(),
                    condition_eq(ipfs_image_hash.to_attribute_value()),
                ),
            ])))
            .limit(1)
            .table_name(TABLE_NAME)
            .send()
            .await?;

        let items = res.items().unwrap_or_default();
        if items.is_empty() {
            return Err(AppError::not_found("データが存在しません"));
        }

        Ok(Token::try_from(items.first().unwrap().to_owned())?)
    }

    pub async fn get(&self, address: &ContractId, token_id: &TokenId) -> AppResult<Token> {
        let res = self
            .cli
            .get_item()
            .table_name(TABLE_NAME)
            .set_key(Some(HashMap::from([
                ("pk".to_string(), address.to_attribute_value()),
                ("sk".to_string(), token_id.to_attribute_value()),
            ])))
            .send()
            .await?;

        res.item.map_or(
            Err(AppError::not_found("データが存在しません")),
            |v| Ok(Token::try_from(v)?),
        )
    }

    pub async fn put(&self, item: Token) -> AppResult<()> {
        self.cli
            .put_item()
            .table_name(TABLE_NAME)
            .set_item(Some(item.into()))
            .send()
            .await?;

        Ok(())
    }
}
