use crate::ddb::{AttributeStringValue, AttributeValueResolver, MustPresent, PrimaryKey};
use crate::domain::contract::{Contract, ContractId, Network, Schema};
use crate::domain::time::LocalDateTime;
use crate::domain::wallet::WalletAddress;
use crate::errors::AppError;
use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use std::str::FromStr;

impl PrimaryKey for ContractId {
    fn typename() -> String {
        "contract".to_string()
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
