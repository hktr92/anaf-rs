use anaf_api::{vat_payer::VatPayerApiVersion, AnafClient, ApiRequest};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let client = AnafClient::new();
    let now = chrono::Local::now().date_naive();

    let request = vec![ApiRequest::new(49201783, now)];

    let response = client
        .vat_payer(VatPayerApiVersion::V8)
        .send(request)
        .await?;

    dbg!(&response);

    Ok(())
}
