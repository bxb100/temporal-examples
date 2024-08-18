use anyhow::Result;
use helper::clients::{get_client, start_workflow};
use log::info;
use nanoid::nanoid;
use temporal_client::{WfClientExt, WorkflowExecutionResult, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());
    let handle = start_workflow(
        &mut client,
        vec!["Temporal".as_json_payload()?],
        "hello-world".to_string(), // task queue
        workflow_id.to_owned(),    // workflow id
        "example".to_string(),     // workflow type
        None,
        WorkflowOptions::default(),
    )
    .await?;

    info!(
        "Started workflow with ID: {workflow_id} and run ID: {}",
        handle.run_id
    );

    let res = client
        .get_untyped_workflow_handle(workflow_id, handle.run_id)
        .get_workflow_result(Default::default())
        .await?;

    if let WorkflowExecutionResult::Succeeded(result) = res {
        info!("payload type is json/plain: {}", result[0].is_json_payload());
        info!("Result: {}", serde_json::from_slice::<String>(&result[0].data)?);
    } else {
        info!("Workflow failed with result: {:?}", res);
    }

    Ok(())
}
