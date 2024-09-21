use helper::get_client;
use temporal_client::{WorkflowClientTrait, WorkflowService};
use temporal_sdk_core::protos::temporal::api::schedule::v1::SchedulePatch;
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::PatchScheduleRequest;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut client = get_client().await?;

    client
        .patch_schedule(PatchScheduleRequest {
            namespace: client.namespace().to_string(),
            schedule_id: "sample-schedule".to_string(),
            patch: Some(SchedulePatch {
                pause: "Pause By Rust SDK".to_string(),
                ..Default::default()
            }),
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        })
        .await?;

    info!("Schedule is now paused.");
    Ok(())
}
