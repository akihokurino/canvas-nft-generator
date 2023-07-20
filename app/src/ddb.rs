use crate::domain::time::LocalDateTime;
use crate::errors::{AppError, Kind};
use crate::AppResult;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Local, TimeZone};
use std::collections::HashMap;

pub mod contract;
pub mod token;

pub trait PrimaryKey: Sized {
    fn typename() -> String;
    fn from(from: String) -> Self;

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
