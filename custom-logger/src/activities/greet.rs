use temporal_sdk::{ActContext, ActivityError};
use tracing::info;

pub async fn greet(_: ActContext, name: String) -> Result<String, ActivityError> {
    info!("Log from activity {}", name);
    Ok(format!("Hello, {}!", name))
}
