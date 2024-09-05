use helper::get_client;
use log::info;
use nanoid::nanoid;
use prost_wkt_types::Duration;
use temporal_client::{WorkflowClientTrait, WorkflowService};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core::protos::temporal::api::common::v1::{Payloads, WorkflowType};
use temporal_sdk_core::protos::temporal::api::enums::v1::ScheduleOverlapPolicy;
use temporal_sdk_core::protos::temporal::api::schedule::v1::{
    schedule_action::Action::StartWorkflow, IntervalSpec, Schedule, ScheduleAction,
    SchedulePolicies, ScheduleSpec,
};
use temporal_sdk_core::protos::temporal::api::taskqueue::v1::TaskQueue;
use temporal_sdk_core::protos::temporal::api::workflow::v1::NewWorkflowExecutionInfo;
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::CreateScheduleRequest;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = get_client().await?;
    let workflow_id = format!("workflow-id-{}", nanoid!());
    let schedule_id = "sample-schedule";

    let _handle = client.create_schedule(
        CreateScheduleRequest {
            namespace: client.namespace().to_string(),
            schedule_id: schedule_id.to_string(),
            schedule: Some(Schedule {
                spec: Some(ScheduleSpec {
                    interval: vec![
                        IntervalSpec {
                            interval: Some(Duration {
                                seconds: 10,
                                nanos: 0,
                            }),
                            phase: None
                        }
                    ],
                    ..Default::default()
                }),
                action: Some(ScheduleAction {
                    action: Some(StartWorkflow(NewWorkflowExecutionInfo {
                        workflow_id: workflow_id.clone(),
                        workflow_type: Some(WorkflowType {
                            name: "reminder".to_string()
                        }),
                        task_queue: Some(TaskQueue {
                            name: "schedules".to_string(),
                            ..Default::default()
                        }),
                        input: Some(Payloads {
                            payloads: vec!["♻️ Dear future self, please take out the recycling tonight. Sincerely, past you ❤️".as_json_payload()?],
                        }),
                        ..Default::default()
                    }))
                }),
                policies: Some(SchedulePolicies {
                    overlap_policy: ScheduleOverlapPolicy::AllowAll as i32,
                    catchup_window: Some(Duration {
                        seconds: 60 * 60 * 24, // 1 day
                        nanos: 0
                    }),
                    ..Default::default()
                }),
                state: None
            }),
            identity: "example-rs-client".to_string(),
            request_id: Uuid::new_v4().to_string(),
            ..Default::default()
        }
    ).await?;

    info!("Started schedule {}.", schedule_id);
    info!(
        r#"
The reminder Workflow will run and log from the Worker every 10 seconds.

You can now run:

  cargo run --bin go-faster / just c go-faster
  cargo run --bin pause / just c pause
  cargo run --bin unpause / just c unpause
  cargo run --bin delete / just c delete"#
    );

    Ok(())
}
