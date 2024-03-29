use crate::errors::AppError;

pub mod application;
pub mod aws;
pub mod ddb;
pub mod di;
pub mod domain;
pub mod errors;
pub mod ethereum;
pub mod internal_api;
pub mod ipfs;
pub mod open_sea;
pub mod sync;

pub type AppResult<T> = Result<T, AppError>;
