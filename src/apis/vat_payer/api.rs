use reqwest::Client;

use crate::ApiRequest;

use super::{VatPayerApiVersion, VatPayerResponse};

#[derive(Debug)]
pub struct VatPayerApi {
    api_url: String,
    version: VatPayerApiVersion,
    client: Client,
}

impl VatPayerApi {
    pub fn new(version: VatPayerApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl VatPayerApi {
    pub async fn send(&self, request: Vec<ApiRequest>) -> anyhow::Result<VatPayerResponse> {
        if request.len() >= 500 {
            anyhow::bail!("ANAF VatPayer API supports maximum number of 500 companies to fetch");
        }

        tracing::info!(
            "Making ANAF VatPayer API {version} call",
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

        let response = response.json::<VatPayerResponse>().await?;

        tracing::debug!("Response: {:#?}", response);

        Ok(response)
    }
}
