use reqwest::Client;

use crate::ApiRequest;

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
    pub async fn send(&self, request: Vec<ApiRequest>) -> anyhow::Result<FarmerResponse> {
        if request.len() >= 500 {
            anyhow::bail!("ANAF Farmer API supports maximum number of 500 companies to fetch");
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

        let response = response.json::<FarmerResponse>().await?;

        tracing::debug!("Response: {:#?}", response);

        Ok(response)
    }
}
