use reqwest::Client;

use crate::ApiRequest;

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
    pub async fn send(&self, request: Vec<ApiRequest>) -> anyhow::Result<CultResponse> {
        if request.len() >= 500 {
            anyhow::bail!("ANAF Cult API supports maximum number of 500 companies to fetch");
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

        let response = response.json::<CultResponse>().await?;

        tracing::debug!("Response: {:#?}", response);

        Ok(response)
    }
}
