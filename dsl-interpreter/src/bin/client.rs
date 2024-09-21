use anyhow::Result;
use helper::client_ext::ClientExt;
use helper::util::client::get_client;
use nanoid::nanoid;
use temporal_client::{ WorkflowClientTrait, WorkflowExecutionResult, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let handle = client
        .start_workflow(
            vec!["Temporal".as_json_payload()?],
            "dsl-interpreter".to_string(), // task queue
            workflow_id.to_owned(),        // workflow id
            "example".to_string(),         // workflow type
            None,
            WorkflowOptions::default(),
        )
        .await?;

    info!(
        "Started workflow with ID: {workflow_id} and run ID: {}",
        handle.run_id
    );

    let res = client
        .get_workflow_result::<String>(workflow_id, handle.run_id)
        .await?;

    info!("Result: {}", res);

    Ok(())
}
