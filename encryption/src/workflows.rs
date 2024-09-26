use crate::worker;
use log::info;
use prost::Message;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;

pub async fn example(ctx: WfContext) -> WorkflowResult<Vec<u8>> {
    let args = ctx.get_args();
    // let data = &args[0].data;
    info!("Received message: {:?}", args[0]);

    // let encrypted_payload = Payload::decode(data.as_slice())?;

    let codec = unsafe { worker::ONCE.get_mut().unwrap() };
    let payloads = codec.decode(vec![&args[0]]).await?;

    let message = serde_json::from_slice::<String>(&payloads[0].data)?;
    info!("Decoded message: {:?}", message);

    let msg = format!("{message}\nBob: Hi Alice, I'm Workflow Bob.");

    let payload = msg.as_json_payload()?;
    let encoded = codec.encode(vec![&payload])?;
    Ok(WfExitValue::Normal(encoded[0].encode_to_vec()))
}
