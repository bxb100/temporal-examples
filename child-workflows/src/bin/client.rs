use helper::{get_client, get_workflow_result};
use log::info;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id = format!("parent-workflow-{}", nanoid!());
    let handle = client
        .start_workflow(
            vec![
                "Alice".as_json_payload()?,
                "Bob".as_json_payload()?,
                "Charlie".as_json_payload()?,
            ],
            "child-workflows".to_owned(), // task queue
            workflow_id.clone(),          // workflow id
            "parent_workflow".to_owned(), // workflow type
            None,
            WorkflowOptions {
                ..Default::default()
            },
        )
        .await?;

    if let Ok(r) = get_workflow_result::<String>(&client, workflow_id, handle.run_id).await {
        assert_eq!(
            r#"I am a child named Alice
I am a child named Bob
I am a child named Charlie"#,
            r
        );
        info!("{r}");
    };

    Ok(())
}
