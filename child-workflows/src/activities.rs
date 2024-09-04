use temporal_sdk::{ActContext, ActivityError};

#[allow(dead_code)]
pub fn greet(_: ActContext, name: String) -> Result<String, ActivityError> {
    Ok(format!("Hello, {}!", name))
}
