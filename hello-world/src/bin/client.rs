use anyhow::Result;
use helper::client::get_client;
use log::info;
use nanoid::nanoid;
use temporal_client::{WfClientExt, WorkflowClientTrait, WorkflowExecutionResult, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let handle = client
        .start_workflow(
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
        let payload = &result[0];

        assert!(payload.is_json_payload());
        assert_eq!("Hello, Temporal!", String::from_json_payload(payload)?);

        info!("Result: {}", String::from_json_payload(payload)?);
    } else {
        info!("Workflow failed with result: {:?}", res);
    }

    Ok(())
}
