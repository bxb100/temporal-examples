use temporal_client::{WfClientExt, WorkflowExecutionResult};

pub async fn get_workflow_first_result<T>(
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
            if let Some(p) = res.first() {
                return Ok(serde_json::from_slice(&p.data)?);
            }
            unreachable!()
        }
        tmp @ _ => {
            log::info!("Result: {:?}", tmp);
            Err(anyhow::anyhow!("Workflow did not succeed"))
        }
    }
}
