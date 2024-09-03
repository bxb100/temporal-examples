use temporal_client::{WfClientExt, WorkflowExecutionResult};
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;

pub async fn get_workflow_result(
    client: &impl WfClientExt,
    workflow_id: String,
    run_id: String,
) -> anyhow::Result<Vec<Payload>> {
    match client
        .get_untyped_workflow_handle(workflow_id, run_id)
        .get_workflow_result(Default::default())
        .await?
    {
        WorkflowExecutionResult::Succeeded(res) => Ok(res),
        tmp @ _ => {
            log::info!("Result: {:?}", tmp);
            Err(anyhow::anyhow!("Workflow did not succeed"))
        }
    }
}
