use crate::types::{Res, User};
use chrono::Utc;
use log::info;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::FromJsonPayloadExt;

pub async fn example(ctx: WfContext) -> WorkflowResult<Res> {
    let args = ctx.get_args();

    let user = User::from_json_payload(args.first().unwrap())?;
    info!("Starting workflow with input: {:?}", user);

    let success = user.created_at < Utc::now()
        && user.hp > 50f64
        && user.matcher.is_match("Kaladin Stormblessed")
        && user.token == vec![1, 2, 3];

    Ok(WfExitValue::Normal(Res {
        success,
        at: Utc::now(),
    }))
}
