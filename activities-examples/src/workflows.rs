use crate::activities::*;
use helper::activity_input::*;
use helper::parse_activity_result::ActivityResolutionExt;
use helper::wf_context_ext::{ProxyActivityOptions, WfContextExt};
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::temporal::api::common::v1::RetryPolicy;
use temporal_sdk_core_protos::coresdk::activity_result::ActivityResolution;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

async fn _execution<T>(ctx: &WfContext, t: T, input: Payload) -> ActivityResolution {
    ctx.proxy_activity(
        t,
        ProxyActivityOptions {
            retry_policy: Some(RetryPolicy {
                initial_interval: Some(prost_wkt_types::Duration {
                    seconds: 0,
                    nanos: 50_000_000, // 50ms
                }),
                maximum_attempts: 2,
                ..Default::default()
            }),
            start_to_close_timeout: Some(Duration::from_secs(30)),
            ..Default::default()
        },
    )(input)
    .await
}

pub async fn http_workflow(ctx: WfContext) -> WorkflowResult<String> {
    let act_handler = _execution(&ctx, make_http_request, none().try_into()?).await;

    let answer = act_handler.parse_result::<String>()?;

    Ok(WfExitValue::Normal(format!("The answer is {}", answer)))
}

pub async fn async_activity_workflow(ctx: WfContext) -> WorkflowResult<String> {
    let act_handler = _execution(&ctx, do_something_async, none().try_into()?).await;

    let answer: String = act_handler.parse_result()?;

    Ok(WfExitValue::Normal(format!("The Peon says: {}", answer)))
}
