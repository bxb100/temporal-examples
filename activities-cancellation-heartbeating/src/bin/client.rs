use helper::get_client;
use log::{info, warn};
use std::time::Duration;
use temporal_client::{WfClientExt, WorkflowExecutionResult, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core::WorkflowClientTrait;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = "cancellation-heartbeating-0".to_string();
    let handle = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "cancellation-heartbeating".to_owned(), // task queue
            workflow_id.clone(),                    // workflow id
            "run_cancellable_activity".to_owned(),  // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    info!(
        "Started workflow_id: {}, run_id: {}",
        workflow_id, handle.run_id
    );

    tokio::time::sleep(Duration::from_secs(40)).await;

    info!("Cancelling workflow");
    let cancel_handle = client
        .cancel_workflow_execution(
            workflow_id.clone(),
            Some(handle.run_id.clone()),
            "Try and cancel".to_string(),
            None,
        )
        .await?;

    info!("Cancelled workflow successfully, {:?}", cancel_handle);
    
    match client
        .get_untyped_workflow_handle(workflow_id, handle.run_id)
        .get_workflow_result(Default::default())
        .await?
    {
        WorkflowExecutionResult::Cancelled(_) => {
            warn!("Workflow was cancelled");
        }
        WorkflowExecutionResult::Failed(failure) => {
            warn!("Workflow failed with failure: {:?}", failure);
        }
        ext => {
           info!("Result: {:?}", ext); 
        }
    };

    Ok(())
}
