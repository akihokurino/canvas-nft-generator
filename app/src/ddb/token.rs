use crate::ddb::{
    condition_eq, condition_sk_type, AttributeStringValue, AttributeValueResolver, MustPresent,
    PrimaryKey,
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
            ipfs_hash: v.get_map("ipfsHash", |v| {
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
                Some(
                    <WalletAddress as Into<String>>::into(self.owner_address).to_attribute_value(),
                ),
            ),
            (
                "ipfsHash".to_string(),
                Some(self.ipfs_hash.to_attribute_value()),
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

    // pub async fn get_by_contract_with_pager(
    //     &self,
    //     address: &ContractId,
    //     key: &PagingKey,
    //     limit: &Option<i32>,
    // ) -> AppResult<(Vec<Token>, PagingKey)> {
    //     let res = self
    //         .cli
    //         .query()
    //         .set_key_conditions(Some(HashMap::from([
    //             ("pk".to_string(), condition_eq(address.to_attribute_value())),
    //             ("sk".to_string(), condition_sk_type::<TokenId>()),
    //         ])))
    //         .set_exclusive_start_key(key.val.to_owned())
    //         .limit(limit.unwrap_or(20))
    //         .table_name(TABLE_NAME)
    //         .send()
    //         .await?;
    //
    //     Ok((
    //         res.items
    //             .unwrap_or_default()
    //             .into_iter()
    //             .map(|v| Token::try_from(v))
    //             .collect::<AppResult<Vec<_>>>()?,
    //         PagingKey::from(res.last_evaluated_key),
    //     ))
    // }
    //
    // pub async fn get_stock_by_contract_with_pager(
    //     &self,
    //     wallet_address: WalletAddress,
    //     address: &ContractId,
    //     key: &PagingKey,
    //     limit: &Option<i32>,
    // ) -> AppResult<(Vec<Token>, PagingKey)> {
    //     let res = self
    //         .cli
    //         .query()
    //         .set_key_conditions(Some(HashMap::from([
    //             ("pk".to_string(), condition_eq(address.to_attribute_value())),
    //             ("sk".to_string(), condition_sk_type::<TokenId>()),
    //         ])))
    //         .set_filter_expression(Some("attribute_not_exists(priceEth)".to_string()))
    //         .filter_expression("ownerAddress = :ownerAddress")
    //         .expression_attribute_values(
    //             ":ownerAddress",
    //             <WalletAddress as Into<String>>::into(wallet_address).to_attribute_value(),
    //         )
    //         .set_exclusive_start_key(key.val.to_owned())
    //         .limit(limit.unwrap_or(20))
    //         .table_name(TABLE_NAME)
    //         .send()
    //         .await?;
    //
    //     Ok((
    //         res.items
    //             .unwrap_or_default()
    //             .into_iter()
    //             .map(|v| Token::try_from(v))
    //             .collect::<AppResult<Vec<_>>>()?,
    //         PagingKey::from(res.last_evaluated_key),
    //     ))
    // }
    //
    // pub async fn get_sell_order_by_contract_with_pager(
    //     &self,
    //     wallet_address: WalletAddress,
    //     address: &ContractId,
    //     key: &PagingKey,
    //     limit: &Option<i32>,
    // ) -> AppResult<(Vec<Token>, PagingKey)> {
    //     let res = self
    //         .cli
    //         .query()
    //         .set_key_conditions(Some(HashMap::from([
    //             ("pk".to_string(), condition_eq(address.to_attribute_value())),
    //             ("sk".to_string(), condition_sk_type::<TokenId>()),
    //         ])))
    //         .set_filter_expression(Some("attribute_exists(priceEth)".to_string()))
    //         .filter_expression("ownerAddress = :ownerAddress")
    //         .expression_attribute_values(
    //             ":ownerAddress",
    //             <WalletAddress as Into<String>>::into(wallet_address).to_attribute_value(),
    //         )
    //         .set_exclusive_start_key(key.val.to_owned())
    //         .limit(limit.unwrap_or(20))
    //         .table_name(TABLE_NAME)
    //         .send()
    //         .await?;
    //
    //     Ok((
    //         res.items
    //             .unwrap_or_default()
    //             .into_iter()
    //             .map(|v| Token::try_from(v))
    //             .collect::<AppResult<Vec<_>>>()?,
    //         PagingKey::from(res.last_evaluated_key),
    //     ))
    // }

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

        res.item
            .map_or(Err(AppError::not_found()), |v| Ok(Token::try_from(v)?))
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
