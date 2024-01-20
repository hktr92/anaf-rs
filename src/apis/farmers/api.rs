use reqwest::{Client, StatusCode};

use crate::{ApiError, ApiRequest, Result};

use super::{FarmerApiVersion, FarmerResponse};

#[derive(Debug)]
pub struct FarmerApi {
    api_url: String,
    version: FarmerApiVersion,
    client: Client,
}

impl FarmerApi {
    pub fn new(version: FarmerApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl FarmerApi {
    pub async fn send(&self, request: Vec<ApiRequest>) -> Result<FarmerResponse> {
        if request.is_empty() || request.len() >= 500 {
            return Err(ApiError::InvalidRequestError(request.len()));
        }

        tracing::info!(
            "Making ANAF Farmer API {version} call",
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
                let response = response.json::<FarmerResponse>().await?;
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
