use async_graphql::{ErrorExtensions, FieldError};

pub struct Error {
    pub code: String,
    pub message: String,
}

impl Error {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl From<app::errors::AppError> for Error {
    fn from(v: app::errors::AppError) -> Self {
        match v.kind {
            app::errors::Kind::BadRequest => Self::new("BadRequest", v.msg.unwrap_or_default()),
            app::errors::Kind::Unauthorized => Self::new("UnAuthorized", v.msg.unwrap_or_default()),
            app::errors::Kind::Forbidden => Self::new("Forbidden", v.msg.unwrap_or_default()),
            app::errors::Kind::NotFound => Self::new("NotFound", v.msg.unwrap_or_default()),
            app::errors::Kind::Internal => Self::new("Internal", v.msg.unwrap_or_default()),
        }
    }
}

impl From<async_graphql::Error> for Error {
    fn from(v: async_graphql::Error) -> Self {
        Self::new("Internal", v.message)
    }
}

impl From<Error> for FieldError {
    fn from(v: Error) -> Self {
        Self::new(v.message).extend_with(|_, err| err.set("code", v.code.to_string()))
    }
}
