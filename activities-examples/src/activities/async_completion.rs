use helper::{activity_input::ActivityInput, get_client};
use log::info;
use std::time::Duration;
use temporal_client::WorkflowClientTrait;
use temporal_sdk::{ActContext, ActExitValue, ActivityError};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core_protos::TaskToken;
use tokio::time::timeout;

pub async fn do_something_async(
    ctx: ActContext,
    _input: ActivityInput<String>,
) -> Result<ActExitValue<()>, ActivityError> {
    let task_token = ctx.get_info().task_token.to_vec();

    tokio::spawn(timeout(
        Duration::from_millis(1000),
        do_some_work(task_token),
    ));

    Ok(ActExitValue::WillCompleteAsync)
}

async fn do_some_work(task_token: Vec<u8>) -> anyhow::Result<()> {
    info!("Starting activity at thread {:?}", std::thread::current());

    let client = get_client().await?;

    client
        .complete_activity_task(
            TaskToken(task_token),
            Some("Job's done!".as_json_payload()?.into()),
        )
        .await?;

    Ok(())
}
