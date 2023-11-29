use reqwest::Client;

use crate::balance::BalanceResponse;

use super::{BalanceApiVersion, BalanceRawResponse, BalanceRequest};

/// Balance API
///
/// This API is used to get the balance of a company and / or an NGO.
///
/// >>**Note**: This API is currently unstable and may change in the future.
///
/// # Example
/// ```rust
/// let client = AnafClient::new();
/// let request = BalanceRequest::new(40914732, 2022);
/// let response = client.balance(BalanceApiVersion::V1).send(request).await?;
///
/// dbg!(&response);
/// ```
#[derive(Debug)]
pub struct BalanceApi {
    api_url: String,
    version: BalanceApiVersion,
    client: Client,
}

impl BalanceApi {
    pub fn new(version: BalanceApiVersion, client: Client, api_url: &str) -> Self {
        Self {
            version,
            client,
            api_url: api_url.to_owned(),
        }
    }
}

impl BalanceApi {
    pub async fn send(&self, request: BalanceRequest) -> anyhow::Result<BalanceResponse> {
        tracing::info!(
            "Making ANAF Balance API {version} call",
            version = self.version
        );

        let request_params = serde_qs::to_string(&request)?;
        let url = &format!("{}?{}", self.api_url, request_params);

        tracing::debug!("URL: {:#?}", url);
        tracing::debug!("Request: {:#?}", request);

        let response = self.client.post(url).send().await?;
        let response = response.json::<BalanceRawResponse>().await?;

        tracing::debug!("Response: {:#?}", response);

        Ok(BalanceResponse::from(response))
    }
}
