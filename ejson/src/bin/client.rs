use anyhow::Result;
use ejson::types::{Res, User};
use helper::client_ext::ClientExt;
use helper::util::client::get_client;
use log::info;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let user = User {
        id: nanoid!(),
        age: 100f32,
        hp: f64::INFINITY,
        matcher: regex::Regex::new(".*Stormblessed")?,
        token: vec![1, 2, 3],
        created_at: chrono::Utc::now(),
    };
    let workflow_id = format!("example-user-{}", user.id);

    let handle = client
        .start_workflow(
            vec![user.as_json_payload()?],
            "ejson".to_string(),    // task queue
            workflow_id.to_owned(), // workflow id
            "example".to_string(),  // workflow type
            None,
            WorkflowOptions::default(),
        )
        .await?;

    info!("Started workflow {}", workflow_id);

    let res = client
        .get_workflow_result::<Res>(workflow_id, handle.run_id)
        .await?;

    info!("Result: {:?}", res);

    Ok(())
}
