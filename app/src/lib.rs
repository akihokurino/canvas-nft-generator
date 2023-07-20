use crate::errors::AppError;

pub mod aws;
pub mod errors;

pub type AppResult<T> = Result<T, AppError>;
