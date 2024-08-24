use anyhow::anyhow;
use log::{info, warn};
use rand::Rng;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use temporal_sdk::{ActContext, ActivityError};
use temporal_sdk_core::protos::coresdk::{AsJsonPayloadExt, FromJsonPayloadExt};

/// slightly modify the temporal-samples-rust version
pub async fn fake_progress(
    ctx: ActContext,
    sleep_interval_ms: Option<u64>,
) -> Result<u64, ActivityError> {
    let sleep_interval_ms = sleep_interval_ms.unwrap_or(1000);

    // allow for resuming from heartbeat
    let starting_point = match ctx.get_heartbeat_details().first() {
        Some(hb) => u64::from_json_payload(hb)?,
        None => 1,
    };

    info!("Starting activity at progress {}", starting_point);

    let progress = Arc::new(AtomicU64::new(starting_point));
    let progress_clone = progress.clone();
    let ctx_clone = ctx.clone();
    // newly spawn very expensive execution first
    tokio::task::spawn(async move {
        for p in starting_point..=100 {
            if ctx_clone.is_cancelled() {
                return;
            }
            let random = { rand::thread_rng().gen_range(1..=30u64) };
            tokio::time::sleep(Duration::from_secs(random)).await;
            progress_clone.store(p, Ordering::Relaxed);
        }
    });
    // then get the progress and loop to send heartbeat
    tokio::task::yield_now().await;
    loop {
        sleep(&ctx, sleep_interval_ms).await;

        let progress = progress.load(Ordering::Relaxed);
        info!("Progress {}", progress);

        // while the spawn worker done or cancelled, the loop will be stopped
        if let Err(e) = heartbeat(&ctx, progress) {
            warn!("{}", e);
            return Err(ActivityError::Cancelled {
                details: Some(progress.as_json_payload()?),
            });
            // FIXME: why not return Ok
            // return Ok(progress);
        }
        if progress >= 100 {
            return Ok(progress);
        }
    }
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

fn heartbeat(ctx: &ActContext, progress: u64) -> Result<(), anyhow::Error> {
    if ctx.is_cancelled() {
        return Err(anyhow!("Fake progress activity cancelled"));
    }

    ctx.record_heartbeat(vec![progress
        .as_json_payload()
        .expect("failed to serialize progress")]);

    Ok(())
}
