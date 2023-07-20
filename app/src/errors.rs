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

    pub fn default() -> Self {
        Self {
            kind: Kind::Internal,
            msg: None,
        }
    }

    pub fn auth_error() -> Self {
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
