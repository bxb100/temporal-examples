use helper::get_client;
use log::info;
use nanoid::nanoid;
use std::time::Duration;
use temporal_client::{WfClientExt, WorkflowOptions};
use temporal_sdk_core::WorkflowClientTrait;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("workflow-id-{}", nanoid!());
    let handle = client
        .start_workflow(
            vec!["".as_json_payload()?.into()],
            "activities-cancellation-heartbeating".to_owned(), // task queue
            workflow_id.to_owned(),                            // workflow id
            "run_cancellable_activity".to_owned(),             // workflow type
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
    info!("Sleeping 30s to allow workflow to run");
    tokio::time::sleep(Duration::from_secs(30)).await;

    info!("Requesting cancellation");
    let _cancel_handle = client
        .cancel_workflow_execution(
            workflow_id.clone(),
            Some(handle.run_id.clone()),
            "Try and cancel".to_string(),
            None,
        )
        .await?;

    // TODO: check in Rust
    // this is typescript version:
    // `if (err instanceof WorkflowFailedError && err.cause instanceof CancelledFailure)`
    match client
        .get_untyped_workflow_handle(workflow_id, handle.run_id)
        .get_workflow_result(Default::default())
        .await
    {
        Ok(res) => {
            info!("Workflow completed with result: {:?}", res);
            Ok(())
        }
        Err(e) => {
            info!("Workflow failed with error: {:?}", e);
            Err(e)
        }
    }
}
