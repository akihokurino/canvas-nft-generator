use crate::errors::AppError;

pub mod aws;
pub mod ddb;
pub mod di;
pub mod domain;
pub mod errors;
pub mod ethereum;
pub mod sync;

pub type AppResult<T> = Result<T, AppError>;
