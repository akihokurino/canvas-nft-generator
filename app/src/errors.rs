use aws_sdk_dynamodb::operation::batch_get_item::BatchGetItemError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::scan::ScanError;
use aws_sdk_lambda::operation::invoke::InvokeError;
use aws_sdk_sesv2::operation::send_email::SendEmailError;
use aws_sdk_sns::operation::publish::PublishError;
use aws_sdk_ssm::error::SdkError;
use aws_sdk_ssm::operation::get_parameter::GetParameterError;
use derive_more::Display;
use ethers::prelude::*;
use std::num::ParseFloatError;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
pub enum Kind {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Internal,
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub kind: Kind,
    pub msg: Option<String>,
}

impl AppError {
    pub fn new(kind: Kind, msg: &str) -> Self {
        Self {
            kind,
            msg: Some(msg.to_string()),
        }
    }

    pub fn bad_request(msg: &str) -> Self {
        Self::new(Kind::BadRequest, msg)
    }

    pub fn un_authorized(msg: &str) -> Self {
        Self::new(Kind::Unauthorized, msg)
    }

    pub fn forbidden(msg: &str) -> Self {
        Self::new(Kind::Forbidden, msg)
    }

    pub fn not_found(msg: &str) -> Self {
        Self::new(Kind::NotFound, msg)
    }

    pub fn internal(msg: &str) -> Self {
        Self::new(Kind::Internal, msg)
    }
}

impl From<SdkError<GetParameterError>> for AppError {
    fn from(err: SdkError<GetParameterError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<PutItemError>> for AppError {
    fn from(err: SdkError<PutItemError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<ScanError>> for AppError {
    fn from(err: SdkError<ScanError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<QueryError>> for AppError {
    fn from(err: SdkError<QueryError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<GetItemError>> for AppError {
    fn from(err: SdkError<GetItemError>) -> Self {
        match err {
            SdkError::ServiceError(ref err) if err.err().is_resource_not_found_exception() => {
                AppError::not_found(&err.err().to_string())
            }
            err => AppError::internal(&err.to_string()),
        }
    }
}

impl From<SdkError<BatchGetItemError>> for AppError {
    fn from(err: SdkError<BatchGetItemError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<DeleteItemError>> for AppError {
    fn from(err: SdkError<DeleteItemError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<InvokeError>> for AppError {
    fn from(err: SdkError<InvokeError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::internal(&err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<rustc_hex::FromHexError> for AppError {
    fn from(err: rustc_hex::FromHexError) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<AbiError> for AppError {
    fn from(err: AbiError) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<ContractError<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
    for AppError
{
    fn from(
        e: ContractError<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    ) -> Self {
        match e {
            ContractError::DecodingError(err) => {
                let msg = format!("ethers contract sign error: {:?}", err);
                AppError::internal(&msg)
            }
            ContractError::AbiError(err) => {
                let msg = format!("ethers contract sign error: {:?}", err);
                AppError::internal(&msg)
            }
            ContractError::DetokenizationError(err) => {
                let msg = format!("ethers contract sign error: {:?}", err);
                AppError::internal(&msg)
            }
            ContractError::ConstructorError => {
                let msg =
                    format!("ethers contract sign error: constructor is not defined in the ABI");
                AppError::internal(&msg)
            }
            ContractError::ContractNotDeployed => {
                let msg = format!("ethers contract sign error: Contract was not deployed");
                AppError::internal(&msg)
            }
            _ => {
                let msg = format!("ethers contract sign error");
                AppError::internal(&msg)
            }
        }
    }
}

impl From<ContractError<Provider<Http>>> for AppError {
    fn from(err: ContractError<Provider<Http>>) -> Self {
        let msg = format!("ethers contract call error: {:?}", err);
        AppError::internal(&msg)
    }
}

impl From<ProviderError> for AppError {
    fn from(err: ProviderError) -> Self {
        let msg = format!("ethers transaction error: {:?}", err);
        AppError::internal(&msg)
    }
}

impl From<SdkError<PublishError>> for AppError {
    fn from(err: SdkError<PublishError>) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<ParseFloatError> for AppError {
    fn from(err: ParseFloatError) -> Self {
        AppError::internal(&err.to_string())
    }
}

impl From<SdkError<SendEmailError>> for AppError {
    fn from(err: SdkError<SendEmailError>) -> Self {
        AppError::internal(&err.to_string())
    }
}
