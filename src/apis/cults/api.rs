use reqwest::{Client, StatusCode};

use crate::{ApiError, ApiRequest, Result};

use super::{CultApiVersion, CultResponse};

#[derive(Debug)]
pub struct CultApi {
    api_url: String,
    version: CultApiVersion,
    client: Client,
}

impl CultApi {
    pub fn new(version: CultApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl CultApi {
    pub async fn send(&self, request: Vec<ApiRequest>) -> Result<CultResponse> {
        if request.is_empty() || request.len() >= 500 {
            return Err(ApiError::InvalidRequestError(request.len()));
        }

        tracing::info!(
            "Making ANAF Cult API {version} call",
            version = self.version
        );
        tracing::debug!("URL: {:#?}", self.api_url);
        tracing::debug!("Request: {:#?}", request);

        let response = self
            .client
            .post(&self.api_url)
            .json(&request)
            .send()
            .await?;

        match response.status() {
            StatusCode::SERVICE_UNAVAILABLE => Err(ApiError::ServiceUnavailable),
            StatusCode::OK => {
                let response = response.json::<CultResponse>().await?;
                tracing::debug!("Response: {:#?}", response);
                Ok(response)
            }
            _ => {
                let response = response.text().await?;
                tracing::debug!("Error Response: {:#?}", response);
                Err(ApiError::ApiError(response))
            }
        }
    }
}
