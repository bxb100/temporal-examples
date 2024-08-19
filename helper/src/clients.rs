use gethostname::gethostname;
use std::{process, str::FromStr};
use temporal_client::{tonic, Client, RetryClient, WorkflowOptions, WorkflowService};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::protos::coresdk::IntoPayloadsExt;
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, WorkflowType};
use temporal_sdk_core::protos::temporal::api::enums::v1::TaskQueueKind;
use temporal_sdk_core::protos::temporal::api::taskqueue::v1::TaskQueue;
use temporal_sdk_core::protos::temporal::api::workflowservice::v1::{
    StartWorkflowExecutionRequest, StartWorkflowExecutionResponse,
};
use temporal_sdk_core::Url;
use uuid::Uuid;

pub static NAMESPACE: &str = "default";

pub async fn get_client() -> Result<RetryClient<Client>, anyhow::Error> {
    let hostname = gethostname().into_string().expect("Failed to get hostname");
    let process_id = process::id();

    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?)
        // if not set, the worker not display
        .identity(format!("{}@{}", process_id, hostname))
        .build()?;

    let client = server_options.connect(NAMESPACE, None).await?;

    Ok(client)
}

/// this copy from [temporal_client::WorkflowClientTrait]
///
/// FIXME: remove this when Jetbrains fix the issue: #RUST-15459
pub async fn start_workflow(
    client: &mut RetryClient<Client>,
    input: Vec<Payload>,
    task_queue: String,
    workflow_id: String,
    workflow_type: String,
    request_id: Option<String>,
    options: WorkflowOptions,
) -> Result<StartWorkflowExecutionResponse, tonic::Status> {
    Ok(client
        .start_workflow_execution(StartWorkflowExecutionRequest {
            // it should use the namespace from the client
            // `client.namespace().to_string()` from [temporal_client::WorkflowClientTrait]
            // again this is a workaround for the issue: #RUST-15459
            namespace: NAMESPACE.to_string(),
            input: input.into_payloads(),
            workflow_id,
            workflow_type: Some(WorkflowType {
                name: workflow_type,
            }),
            task_queue: Some(TaskQueue {
                name: task_queue,
                kind: TaskQueueKind::Unspecified as i32,
                normal_name: "".to_string(),
            }),
            request_id: request_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            workflow_id_reuse_policy: options.id_reuse_policy as i32,
            workflow_execution_timeout: options.execution_timeout.and_then(|d| d.try_into().ok()),
            workflow_run_timeout: options.run_timeout.and_then(|d| d.try_into().ok()),
            workflow_task_timeout: options.task_timeout.and_then(|d| d.try_into().ok()),
            search_attributes: options.search_attributes.map(|d| d.into()),
            cron_schedule: options.cron_schedule.unwrap_or_default(),
            request_eager_execution: options.enable_eager_workflow_start,
            retry_policy: options.retry_policy,
            ..Default::default()
        })
        .await?
        .into_inner())
}
