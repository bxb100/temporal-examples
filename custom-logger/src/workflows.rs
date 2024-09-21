use crate::activities::greet;
use helper::activity_resolution_ext::ActivityResolutionExt;
use helper::payloads_ext::PayloadsExt;
use helper::wf_context_ext::{ProxyActivityOptions, WfContextExt};
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::AsJsonPayloadExt;
use tracing::info;

pub async fn example(ctx: WfContext) -> WorkflowResult<String> {
    let input = ctx.get_args().first_input::<String>();
    info!("Starting workflow with input: {:?}", input);

    let resolution = ctx.proxy_activity(
        greet,
        ProxyActivityOptions {
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        },
    )("Temporal".as_json_payload()?)
    .await;

    let v = resolution.parse_result::<String>()?;
    info!("Greeted {}", v);
    Ok(WfExitValue::Normal(v))
}
