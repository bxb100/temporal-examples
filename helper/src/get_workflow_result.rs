use anyhow::anyhow;
use temporal_client::{WfClientExt, WorkflowExecutionResult};
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, Payloads};

pub async fn get_workflow_result<T>(
    client: &impl WfClientExt,
    workflow_id: String,
    run_id: String,
) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    match client
        .get_untyped_workflow_handle(workflow_id, run_id)
        .get_workflow_result(Default::default())
        .await?
    {
        WorkflowExecutionResult::Succeeded(res) => {
            let p = Payload::try_from(Payloads { payloads: res }).map_err(|e| anyhow!("{}", e))?;
            Ok(serde_json::from_slice(&p.data)?)
        }
        tmp @ _ => {
            log::info!("Result: {:?}", tmp);
            Err(anyhow::anyhow!("Workflow did not succeed"))
        }
    }
}
