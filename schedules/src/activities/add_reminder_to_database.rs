use log::info;
use temporal_sdk::{ActContext, ActivityError};

pub async fn add_reminder_to_database(_: ActContext, _: String) -> Result<(), ActivityError> {
    info!("Adding reminder record to the database");
    Ok(())
}
