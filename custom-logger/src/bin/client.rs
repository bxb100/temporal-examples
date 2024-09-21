use anyhow::Result;
use helper::util::client::get_client;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let _handle = client
        .start_workflow(
            vec![],
            "custom-logger".to_string(), // task queue
            workflow_id.to_owned(),      // workflow id
            "example".to_string(),       // workflow type
            None,
            WorkflowOptions::default(),
        )
        .await?;

    Ok(())
}
