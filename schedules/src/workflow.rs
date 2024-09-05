use crate::activities::*;
use helper::wf_context_ext::{ProxyActivityOptions, WfContextExt};
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};

pub async fn reminder(ctx: WfContext) -> WorkflowResult<()> {
    let text = &ctx.get_args()[0];

    let _handle1 = ctx.proxy_activity(
        add_reminder_to_database,
        ProxyActivityOptions {
            start_to_close_timeout: Some(Duration::from_secs(60)), // 1 minute
            ..Default::default()
        },
    )(text.clone())
    .await;

    let _handle2 = ctx.proxy_activity(
        notify_user,
        ProxyActivityOptions {
            start_to_close_timeout: Some(Duration::from_secs(60)), // 1 minute
            ..Default::default()
        },
    )(text.clone())
    .await;

    Ok(WfExitValue::Normal(()))
}
