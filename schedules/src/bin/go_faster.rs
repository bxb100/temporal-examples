use chrono::Duration;
use helper::get_client;
use temporal_client::{WorkflowClientTrait, WorkflowService};
use temporal_sdk_core::protos::temporal::api::schedule::v1::{IntervalSpec, Schedule};
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::{
    DescribeScheduleRequest, UpdateScheduleRequest,
};
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let mut client = get_client().await?;
    let schedule_id = "sample-schedule";

    let response = client
        .describe_schedule(DescribeScheduleRequest {
            namespace: client.namespace().to_string(),
            schedule_id: schedule_id.to_string(),
        })
        .await?
        .into_inner();

    info!("Got schedule conflict token: {:?}", response.conflict_token);

    let mut schedule = response.schedule;
    if let Some(Schedule {
        spec: Some(ref mut spec),
        ..
    }) = schedule
    {
        spec.interval = vec![IntervalSpec {
            interval: Some(Duration::seconds(5).into()),
            phase: None,
        }];
    }

    client
        .update_schedule(UpdateScheduleRequest {
            namespace: client.namespace().to_string(),
            schedule_id: schedule_id.to_string(),
            schedule,
            conflict_token: response.conflict_token,
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        })
        .await?;

    info!("Schedule is now triggered every 5 seconds.");
    Ok(())
}
