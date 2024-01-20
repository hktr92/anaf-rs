use reqwest::{Client, StatusCode};

use crate::{ApiError, ApiRequest, Result};

use super::{VatPayerApiVersion, VatPayerAsyncResponse, VatPayerAsyncToken, VatPayerResponse};

#[derive(Debug)]
pub struct VatPayerAsyncApi {
    api_url: String,
    version: VatPayerApiVersion,
    client: Client,
}

impl VatPayerAsyncApi {
    pub fn new(version: VatPayerApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl VatPayerAsyncApi {
    pub async fn send(&self, request: Vec<ApiRequest>) -> Result<VatPayerAsyncResponse> {
        if request.is_empty() || request.len() >= 500 {
            return Err(ApiError::InvalidRequestError(request.len()));
        }

        tracing::info!(
            "Making ANAF VatPayer Async API {version} call",
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
                let response = response.json::<VatPayerAsyncResponse>().await?;
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

    // TODO: finish the implementation
    pub async fn fetch(&self, token: VatPayerAsyncToken) -> anyhow::Result<VatPayerResponse> {
        // TODO: implement retry logic: min 2 seconds, min 10 seconds between retries

        let url = format!("{}?id={}", self.api_url, token);
        tracing::info!(
            "Making ANAF VatPayer Async API {version} call",
            version = self.version
        );
        tracing::debug!("URL: {:#?}", self.api_url);
        tracing::debug!("Token: {:#?}", token);

        let response = self.client.get(&url).send().await?;
        let response = response.json::<VatPayerResponse>().await?;

        tracing::debug!("Response: {:#?}", response);

        Ok(response)
    }
}
