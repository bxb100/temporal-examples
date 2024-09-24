use crate::activities::greet;
use helper::activity_resolution_ext::ActivityResolutionExt;
use helper::wf_context_ext::{ProxyActivityOptions, WfContextExt};
use log::info;
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

pub async fn example(ctx: WfContext) -> WorkflowResult<String> {
    let args = ctx.get_args();

    let input = String::from_json_payload(args.first().unwrap())?;
    info!("Starting workflow with input: {}", input);

    let resolution = ctx.proxy_activity(
        greet,
        ProxyActivityOptions {
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        },
    )(input.as_json_payload()?)
    .await;

    let v = resolution.parse_result::<String>()?;
    info!("Activity completed with: {}", v);
    Ok(WfExitValue::Normal(v))
}
