use helper::get_client;
use log::info;
use temporal_client::{WorkflowClientTrait, WorkflowService};
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::DeleteScheduleRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = get_client().await?;

    client
        .delete_schedule(DeleteScheduleRequest {
            namespace: client.namespace().to_string(),
            schedule_id: "sample-schedule".to_string(),
            ..Default::default()
        })
        .await?;

    info!("Schedule is now deleted.");
    Ok(())
}
