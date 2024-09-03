use helper::{get_client, get_workflow_result, payload_deserialize};
use log::info;
use nanoid::nanoid;
use temporal_client::{WorkflowClientTrait, WorkflowOptions};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use tokio::join;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let client = get_client().await?;

    let workflow_id1 = format!("workflow-id-{}", nanoid!());
    let handle1 = client.start_workflow(
        vec!["".as_json_payload()?.into()],
        "activities-examples".to_owned(), // task queue
        workflow_id1.clone(),             // workflow id
        "http_workflow".to_owned(),       // workflow type
        None,
        WorkflowOptions {
            ..Default::default()
        },
    );

    let workflow_id2 = format!("workflow-id-{}", nanoid!());
    let handle2 = client.start_workflow(
        vec!["".as_json_payload()?.into()],
        "activities-examples".to_owned(),     // task queue
        workflow_id2.clone(),                 // workflow id
        "async_activity_workflow".to_owned(), // workflow type
        None,
        WorkflowOptions {
            ..Default::default()
        },
    );

    let (f, s) = join!(handle1, handle2);

    if let Ok(r) = get_workflow_result(&client, workflow_id1, f?.run_id).await {
        for p in r {
            info!("http_workflow result: {:?}", payload_deserialize::<String>(&p)?);
        }
    };

    if let Ok(r) = get_workflow_result(&client, workflow_id2, s?.run_id).await {
        for p in r {
            info!("async_activity_workflow result: {:?}", payload_deserialize::<String>(&p)?);
        }
    };

    Ok(())
}
