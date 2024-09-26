use anyhow::Result;
use helper::util::client::get_client;
use helper::client_ext::ClientExt;
use log::info;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let handle = client
        .start_workflow(
            vec!["Temporal".as_json_payload()?],
            "encryption".to_string(), // task queue
            workflow_id.to_owned(),           // workflow id
            "example".to_string(),            // workflow type
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
