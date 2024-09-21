use helper::get_client;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let _handle = client
        .start_workflow(
            vec![0u32.as_json_payload()?],
            "continue-as-new".to_string(),  // task queue
            workflow_id.to_owned(),         // workflow id
            "looping_workflow".to_string(), // workflow type
            None,
            WorkflowOptions::default(),
        )
        .await?;

    Ok(())
}
