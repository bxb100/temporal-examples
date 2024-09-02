use log::info;

use helper::parse_activity_result;
use std::time::Duration;
use temporal_sdk::CancellableFuture;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core_protos::coresdk::workflow_commands::ActivityCancellationType;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

pub async fn run_cancellable_activity(ctx: WfContext) -> WorkflowResult<u64> {
    let fake_progress_handle = ctx.activity(ActivityOptions {
        activity_type: "fake_progress".to_string(),
        cancellation_type: ActivityCancellationType::WaitCancellationCompleted,
        input: 1000u64.as_json_payload()?,
        heartbeat_timeout: Some(Duration::from_secs(3)),
        start_to_close_timeout: Some(Duration::from_secs(60)),
        ..Default::default()
    });

    let cancel_handle = ctx.cancelled();

    tokio::pin!(fake_progress_handle);

    let exit = tokio::select!(
        _ = cancel_handle => {
            info!("Workflow cancelled");
            WfExitValue::Cancelled
        },
        res = &mut fake_progress_handle => {
            info!("Activity completed with {:?}", res);
            WfExitValue::Normal(
                 parse_activity_result(&res)?
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
