use log::info;
use std::time::Duration;
use temporal_sdk::{ActContext, ActivityError};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

pub async fn fake_progress(ctx: ActContext, sleep_interval_ms: u64) -> Result<u64, ActivityError> {
    // allow for resuming from heartbeat
    let starting_point = match ctx.get_heartbeat_details().first() {
        Some(hb) => u64::from_json_payload(hb)?,
        None => 1,
    };

    info!("Starting activity at progress {}", starting_point);

    for progress in starting_point..=100 {
        if ctx.is_cancelled() {
            info!("Activity cancelled");
            return Err(ActivityError::Cancelled { details: None });
        }
        sleep(&ctx, sleep_interval_ms).await;
        info!("Progress {}", progress);
        ctx.record_heartbeat(vec![progress.as_json_payload()?]);
    }
    Ok(100)
}

/// rewrite typescript version to rust
///
/// ```typescript
///   public readonly sleep = (ms: Duration): Promise<void> => {
///     let handle: NodeJS.Timeout;
///     const timer = new Promise<void>((resolve) => {
///       handle = setTimeout(resolve, msToNumber(ms));
///     });
///     return Promise.race([this.cancelled.finally(() => clearTimeout(handle)), timer]);
///   };
/// ```
async fn sleep(ctx: &ActContext, sleep_interval_ms: u64) {
    tokio::select!(
        _ = ctx.cancelled() => {}
        _ = tokio::time::sleep(Duration::from_millis(sleep_interval_ms)) => {}
    );
}
