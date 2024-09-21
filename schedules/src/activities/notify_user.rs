use temporal_sdk::{ActContext, ActivityError};
use tracing::info;

pub async fn notify_user(_: ActContext, text: String) -> Result<(), ActivityError> {
    info!("Notifying user {}", text);
    Ok(())
}
