use crate::payload_ext::PayloadExt;
use anyhow::anyhow;
use core::future::Future;
use serde::de::DeserializeOwned;
use temporal_client::{WfClientExt, WorkflowExecutionResult};
use temporal_sdk_core::protos::temporal::api::common::v1::{Payload, Payloads};

pub trait ClientExt {
    fn get_workflow_result<Rs>(
        &self,
        workflow_id: String,
        run_id: String,
    ) -> impl Future<Output = anyhow::Result<Rs>> + Send
    where
        Rs: DeserializeOwned;
}

impl<T> ClientExt for T
where
    T: WfClientExt + Clone + Sized + Sync,
{
    fn get_workflow_result<Rs>(
        &self,
        workflow_id: String,
        run_id: String,
    ) -> impl Future<Output = anyhow::Result<Rs>> + Send
    where
        Rs: DeserializeOwned,
    {
        async {
            match self
                .get_untyped_workflow_handle(workflow_id, run_id)
                .get_workflow_result(Default::default())
                .await?
            {
                WorkflowExecutionResult::Succeeded(res) => {
                    let mut p = Payload::try_from(Payloads { payloads: res })
                        .map_err(|e| anyhow!("{}", e))?;
                    if let Some(codec) = self.get_codec() {
                        p = (codec.decode)(&p);
                    }
                    Ok(p.deserialize()?)
                }
                other => Err(anyhow::anyhow!("Workflow did not succeed {:?}", other)),
            }
        }
    }
}
