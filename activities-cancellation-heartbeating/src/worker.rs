use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, CoreRuntime};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};

use crate::activities;
use crate::workflows;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::get_client().await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("cancellation-heartbeating")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "cancellation-heartbeating");
    worker.register_activity("fake_progress", activities::fake_progress);

    worker.register_wf(
        "run_cancellable_activity",
        workflows::run_cancellable_activity,
    );
    
    worker.run().await?;

    Ok(())
}
