use anyhow::Result;
use dsl_interpreter::dsl::types::Dsl;
use helper::util::client::get_client;
use log::info;
use nanoid::nanoid;
use std::env;
use std::fs;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("{:?}", env::current_dir()?);

    // parse DSL configuration
    let args = env::args().collect::<Vec<String>>();
    let default_file = "workflow1.yaml".to_string();
    let file = args.get(1).unwrap_or(&default_file);
    let dsl = serde_yml::from_str::<Dsl>(&fs::read_to_string(file)?)?;

    let client = get_client().await?;

    let workflow_id = format!("workflow-{}", nanoid!());

    let handle = client
        .start_workflow(
            vec![dsl.as_json_payload()?],
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

    Ok(())
}
