use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    #[serde(alias = "cod")]
    pub status: usize,

    #[serde(alias = "message")]
    pub message: String,

    #[serde(alias = "found")]
    pub data: Vec<T>,

    #[serde(alias = "notFound")]
    pub not_found: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AsyncApiResponse<T> {
    #[serde(alias = "cod")]
    pub status: usize,

    #[serde(alias = "message")]
    pub message: String,

    #[serde(alias = "correlationId")]
    pub token: T,
}
