use gethostname::gethostname;
use std::{process, str::FromStr};
use temporal_client::{Client, RetryClient};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::Url;

pub static NAMESPACE: &str = "default";

pub async fn get_client() -> Result<RetryClient<Client>, anyhow::Error> {
    let hostname = gethostname().into_string().expect("Failed to get hostname");
    let process_id = process::id();

    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?)
        // if not set, the worker not display
        .identity(format!("{}@{}", process_id, hostname))
        .build()?;

    let client = server_options.connect(NAMESPACE, None).await?;

    Ok(client)
}
