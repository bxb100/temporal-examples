use log::info;
use temporal_sdk::{ActContext, ActivityError};

pub async fn notify_user(_: ActContext, text: String) -> Result<(), ActivityError> {
    info!("Notifying user {}", text);
    Ok(())
}
