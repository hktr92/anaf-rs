#[cfg(feature = "balance_api")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anaf_api::{
        balance::{BalanceApiVersion, BalanceRequest},
        AnafClient,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let client = AnafClient::new();

    let request = BalanceRequest::new(40914732, 2022);
    let response = client.balance(BalanceApiVersion::V1).send(request).await?;

    dbg!(&response);

    Ok(())
}

#[cfg(not(feature = "balance_api"))]
fn main() {
    eprintln!("Please enable the `balance_api` feature to run this example.");
}
