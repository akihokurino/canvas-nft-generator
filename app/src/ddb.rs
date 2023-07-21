use crate::domain::time::LocalDateTime;
use crate::errors::{AppError, Kind};
use crate::AppResult;
use aws_sdk_dynamodb::types::{AttributeValue, ComparisonOperator, Condition};
use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod contract;
pub mod token;

pub trait PrimaryKey: Sized {
    fn typename() -> String;
    fn raw(&self) -> String;
    fn from(from: String) -> Self;

    fn to_attribute_value(&self) -> AttributeValue {
        AttributeValue::S(format!("{}#{}", Self::typename(), self.raw()))
    }

    fn try_from_attribute_string(v: String) -> Result<Self, String> {
        v.strip_prefix(format!("{}#", Self::typename()).as_str())
            .ok_or_else(|| "invalid format".to_string())
            .map(|v| Self::from(v.to_string()))
    }

    fn try_from_attribute_value(av: &AttributeValue) -> Result<Self, String> {
        let raw = av
            .as_s()
            .map_err(|_| "not a string".to_string())
            .map(Clone::clone)?;
        Self::try_from_attribute_string(raw)
    }

    fn key_tuples(&self) -> Vec<(String, AttributeValue)> {
        vec![
            ("pk".to_string(), self.to_attribute_value()),
            ("sk".to_string(), "#".to_string().to_attribute_value()),
        ]
    }

    fn key_map(&self) -> HashMap<String, AttributeValue> {
        self.key_tuples().into_iter().collect()
    }
}

pub trait AttributeStringValue: Sized {
    fn to_attribute_string(&self) -> String;
    fn try_from_attribute_string(v: String) -> Result<Self, String>;

    fn to_attribute_value(&self) -> AttributeValue {
        AttributeValue::S(self.to_attribute_string())
    }

    fn try_from_attribute_value(av: &AttributeValue) -> Result<Self, String> {
        Self::try_from_attribute_string(
            av.as_s()
                .map_err(|_| "not a string".to_string())
                .map(Clone::clone)?,
        )
    }
}

impl AttributeStringValue for String {
    fn to_attribute_string(&self) -> String {
        self.clone()
    }

    fn try_from_attribute_string(v: String) -> Result<Self, String> {
        Ok(v)
    }
}

impl AttributeStringValue for LocalDateTime {
    fn to_attribute_string(&self) -> String {
        self.to_rfc3339()
    }

    fn try_from_attribute_string(v: String) -> Result<Self, String> {
        Self::parse_from_rfc3339(&v)
    }
}

pub trait ParseFromRfc3339<T> {
    fn parse_from_rfc3339(s: &str) -> Result<T, String>;
}

impl ParseFromRfc3339<Self> for LocalDateTime {
    fn parse_from_rfc3339(s: &str) -> Result<Self, String> {
        DateTime::parse_from_rfc3339(s)
            .map_err(|e| e.to_string())
            .map(|dt| Local.from_local_datetime(&dt.naive_local()).unwrap())
    }
}

pub trait AttributeValueResolver {
    fn get_map<T, F: FnOnce(Option<&AttributeValue>) -> Result<T, String>>(
        &self,
        key: &str,
        f: F,
    ) -> AppResult<T>;
}

impl AttributeValueResolver for HashMap<String, AttributeValue> {
    fn get_map<T, F: FnOnce(Option<&AttributeValue>) -> Result<T, String>>(
        &self,
        key: &str,
        f: F,
    ) -> AppResult<T> {
        f(self.get(key))
            .map_err(|err| AppError::new(Kind::Internal, format!("{}: {}", key, err).as_str()))
    }
}

pub trait MustPresent<T> {
    fn must_present(self) -> Result<T, String>;
}

impl<T> MustPresent<T> for Option<T> {
    fn must_present(self) -> Result<T, String> {
        self.ok_or_else(|| format!("missing field"))
    }
}

#[allow(unused)]
pub fn condition_sk_type<T: PrimaryKey>() -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::BeginsWith)
        .attribute_value_list(AttributeValue::S(format!("{}#", T::typename())))
        .build()
}

#[allow(unused)]
pub fn condition_eq(v: AttributeValue) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Eq)
        .attribute_value_list(v)
        .build()
}

#[allow(unused)]
pub fn condition_contains(v: impl Into<String>) -> Condition {
    Condition::builder()
        .comparison_operator(ComparisonOperator::Contains)
        .attribute_value_list(v.into().to_attribute_value())
        .build()
}

#[derive(Default)]
pub struct PagingKey {
    pub val: Option<HashMap<String, AttributeValue>>,
}

impl PagingKey {
    pub fn from(val: Option<HashMap<String, AttributeValue>>) -> Self {
        Self { val }
    }

    pub fn decode(from: Option<String>) -> AppResult<Self> {
        if from.is_none() {
            return Ok(Self { val: None });
        }

        let bytes = base64::decode::<String>(from.unwrap())?;
        let json = bytes.iter().map(|&s| s as char).collect::<String>();
        let tmp = serde_json::from_str::<HashMap<String, EncodableAttributeValue>>(json.as_str())?;
        let mut val: HashMap<String, AttributeValue> = HashMap::new();
        for (k, v) in tmp {
            match v {
                EncodableAttributeValue::S(rv) => {
                    val.insert(k.to_owned(), AttributeValue::S(rv));
                }
                EncodableAttributeValue::N(rv) => {
                    val.insert(k.to_owned(), AttributeValue::N(rv));
                }
            }
        }
        Ok(Self { val: Some(val) })
    }

    pub fn encode(&self) -> Option<String> {
        if self.val.is_none() {
            return None;
        }

        let mut tmp: HashMap<String, EncodableAttributeValue> = HashMap::new();
        for (k, v) in self.val.to_owned().unwrap() {
            if v.is_s() {
                tmp.insert(
                    k.to_owned(),
                    EncodableAttributeValue::S(v.as_s().unwrap().to_owned()),
                );
            }
            if v.is_n() {
                tmp.insert(
                    k.to_owned(),
                    EncodableAttributeValue::N(v.as_n().unwrap().to_owned()),
                );
            }
        }
        let json = serde_json::to_string(&tmp).unwrap();
        Some(base64::encode::<String>(json))
    }
}

#[derive(Serialize, Deserialize)]
pub enum EncodableAttributeValue {
    S(String),
    N(String),
}
