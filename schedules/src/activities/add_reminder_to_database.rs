use temporal_sdk::{ActContext, ActivityError};
use tracing::info;

pub async fn add_reminder_to_database(_: ActContext, _: String) -> Result<(), ActivityError> {
    info!("Adding reminder record to the database");
    Ok(())
}
