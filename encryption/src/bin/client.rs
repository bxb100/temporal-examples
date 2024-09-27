use anyhow::{anyhow, Result};
use encryption::worker::{init_codec, ONCE};
use log::info;
use nanoid::nanoid;
use std::str::FromStr;
use std::sync::Arc;
use temporal_client::{
    Codec, WfClientExt, WorkflowClientTrait, WorkflowExecutionResult, WorkflowOptions,
};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, Payloads};
use temporal_sdk_core::Url;
use temporal_sdk_core_protos::coresdk::FromJsonPayloadExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    unsafe {
        init_codec().await;
    };

    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let mut client = server_options.connect("default", None).await?;

    client.get_client_mut().set_codec(Codec {
        encode: Arc::new(Box::new(|payload| unsafe {
            let codec = ONCE.get().expect("Codec not initialized");
            codec.encode(vec![payload]).unwrap().pop().unwrap()
        })),
        decode: Arc::new(Box::new(|p| {
            let codec = unsafe { ONCE.get_mut().expect("Codec not initialized") };
            codec.decode(vec![p]).unwrap().pop().unwrap()
        })),
    });

    let workflow_id = format!("workflow-{}", nanoid!());

    let handle = client
        .start_workflow(
            vec!["Alice: Private message for Bob.".as_json_payload()?],
            "encryption".to_string(), // task queue
            workflow_id.to_owned(),   // workflow id
            "example".to_string(),    // workflow type
            None,
            WorkflowOptions::default(),
        )
        .await?;

    info!(
        "Started workflow with ID: {workflow_id} and run ID: {}",
        handle.run_id
    );

    let res: String = match client
        .get_untyped_workflow_handle(workflow_id, handle.run_id)
        .get_workflow_result(Default::default())
        .await?
    {
        WorkflowExecutionResult::Succeeded(res) => {
            let mut p =
                Payload::try_from(Payloads { payloads: res }).map_err(|e| anyhow!("{}", e))?;
            if let Some(codec) = client.get_codec() {
                p = (codec.decode)(&p);
            }
            Ok(String::from_json_payload(&p)?)
        }
        other => Err(anyhow::anyhow!("Workflow did not succeed {:?}", other)),
    }?;

    info!("Result: {:?}", res);

    Ok(())
}
