use temporal_sdk::{ActContext, ActivityError};

pub async fn greet(_ctx: ActContext, payload: Option<String>) -> Result<String, ActivityError> {
    match payload {
        Some(name) => Ok(format!("Hello, {}!", name)),
        None => Err(ActivityError::Cancelled {
            details: Some("No name provided".into()),
        }),
    }
}
