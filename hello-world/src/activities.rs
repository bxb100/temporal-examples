use temporal_sdk::{ActContext, ActivityError};

pub async fn greet(_: ActContext, name: String) -> Result<String, ActivityError> {
    Ok(format!("Hello, {}!", name))
}
