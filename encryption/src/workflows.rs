use log::info;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};

pub async fn example(ctx: WfContext) -> WorkflowResult<String> {
    let burrow = ctx.decode_payload(&ctx.get_args()[0])?;

    let message = serde_json::from_slice::<String>(&burrow.data)?;
    info!("Decoded message: {:?}", message);

    let msg = format!("{message}\nBob: Hi Alice, I'm Workflow Bob.");

    Ok(WfExitValue::Normal(msg))
}
