use tracing::info;

use crate::activities::fake_progress;
use helper::activity_resolution_ext::ActivityResolutionExt;
use helper::wf_context_ext::*;
use std::time::Duration;
use temporal_sdk::{WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::workflow_commands::ActivityCancellationType;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

pub async fn run_cancellable_activity(ctx: WfContext) -> WorkflowResult<u64> {
    let fake_progress = ctx.proxy_activity(
        fake_progress,
        ProxyActivityOptions {
            cancellation_type: ActivityCancellationType::WaitCancellationCompleted,
            heartbeat_timeout: Some(Duration::from_secs(3)),
            start_to_close_timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        },
    );

    let mut fake_progress_handle = fake_progress(1000u64.as_json_payload()?);

    let cancel_handle = ctx.cancelled();

    let exit = tokio::select!(
        _ = cancel_handle => {
            info!("Workflow cancelled");
            WfExitValue::Cancelled
        },
        res = &mut fake_progress_handle => {
            info!("Activity completed with {:?}", res);
            WfExitValue::Normal(
                res.parse_result()?
            )
        }
    );

    // wait for the activity complete, and update the activity FSM status
    if let WfExitValue::Cancelled = exit {
        let _ = fake_progress_handle.cancel(&ctx);
        // No need FuseFuture;
        // Is the correct way to use after `select!`?
        fake_progress_handle.await;
    }

    Ok(exit)
}
