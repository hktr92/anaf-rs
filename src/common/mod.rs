use thiserror::Error;

mod request;
mod response;

pub use request::*;
pub use response::*;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("ANAF API is under maintenance")]
    ServiceUnavailable,

    #[error("ANAF API returned an error: {0}")]
    ApiError(String),

    #[error("ANAF API supports fetching between 1 and 500 companies, got {0}.")]
    InvalidRequestError(usize),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Query String error: {0}")]
    QueryStringError(#[from] serde_qs::Error),
}
