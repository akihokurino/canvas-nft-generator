use aws_sdk_dynamodb::operation::batch_get_item::BatchGetItemError;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::query::QueryError;
use aws_sdk_dynamodb::operation::scan::ScanError;
use aws_sdk_lambda::operation::invoke::InvokeError;
use aws_sdk_ssm::error::SdkError;
use aws_sdk_ssm::operation::get_parameter::GetParameterError;
use derive_more::Display;

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

    pub fn internal() -> Self {
        Self {
            kind: Kind::Internal,
            msg: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            kind: Kind::NotFound,
            msg: None,
        }
    }

    pub fn un_authorized() -> Self {
        Self {
            kind: Kind::Unauthorized,
            msg: None,
        }
    }
}

impl From<SdkError<GetParameterError>> for AppError {
    fn from(err: SdkError<GetParameterError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<PutItemError>> for AppError {
    fn from(err: SdkError<PutItemError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<ScanError>> for AppError {
    fn from(err: SdkError<ScanError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<QueryError>> for AppError {
    fn from(err: SdkError<QueryError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<GetItemError>> for AppError {
    fn from(err: SdkError<GetItemError>) -> Self {
        match err {
            SdkError::ServiceError(ref err) if err.err().is_resource_not_found_exception() => {
                Self {
                    kind: Kind::NotFound,
                    msg: Some(err.err().to_string()),
                }
            }
            err => Self {
                kind: Kind::Internal,
                msg: Some(err.to_string()),
            },
        }
    }
}

impl From<SdkError<BatchGetItemError>> for AppError {
    fn from(err: SdkError<BatchGetItemError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<DeleteItemError>> for AppError {
    fn from(err: SdkError<DeleteItemError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<SdkError<InvokeError>> for AppError {
    fn from(err: SdkError<InvokeError>) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(err: base64::DecodeError) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(err.to_string()),
        }
    }
}