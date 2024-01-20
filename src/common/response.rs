use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<T> {
    #[serde(alias = "cod")]
    pub status: usize,

    #[serde(alias = "message")]
    pub message: String,

    #[serde(alias = "found")]
    pub data: Vec<T>,

    #[serde(alias = "notFound")]
    pub not_found: Vec<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AsyncApiResponse<T> {
    #[serde(alias = "cod")]
    pub status: usize,

    #[serde(alias = "message")]
    pub message: String,

    #[serde(alias = "correlationId")]
    pub token: T,
}
