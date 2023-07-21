use app::ddb::ParseFromRfc3339;
use app::domain;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType};
use async_graphql_value::ConstValue;
use derive_more::{From, Into};

pub mod contract;
pub mod wallet;

#[derive(Clone, Debug, From, Into)]
pub struct DateTime(pub domain::time::LocalDateTime);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: ConstValue) -> InputValueResult<Self> {
        if let ConstValue::String(v) = value {
            Ok(Self(domain::time::LocalDateTime::parse_from_rfc3339(&v)?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> ConstValue {
        ConstValue::String(self.0.to_rfc3339())
    }
}
