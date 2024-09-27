use crate::types::EncryptionPayload;
use crate::worker;
use log::info;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

pub async fn example(ctx: WfContext) -> WorkflowResult<EncryptionPayload> {
    let codec = unsafe { worker::ONCE.get_mut().unwrap() };
    let payloads = codec.decode(ctx.get_args().iter().collect()).await?;

    let message = serde_json::from_slice::<String>(&payloads[0].data)?;
    info!("Decoded message: {:?}", message);

    let msg = format!("{message}\nBob: Hi Alice, I'm Workflow Bob.");

    let payload = msg.as_json_payload()?;
    let encoded = codec.encode(vec![&payload])?;

    let res = encoded
        .first()
        .map(|p| EncryptionPayload {
            metadata: p.metadata.clone(),
            data: p.data.clone(),
        })
        .unwrap();

    Ok(WfExitValue::Normal(res))
}
