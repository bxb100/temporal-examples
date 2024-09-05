use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::init_worker;
use temporal_sdk_core_api::worker::WorkerConfigBuilder;

use crate::activities;
use crate::workflows;
use helper::{core_runtime, get_type_name};

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("cancellation-heartbeating")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "cancellation-heartbeating");
    let (l, r) = get_type_name(activities::fake_progress);
    worker.register_activity(l, r);

    worker.register_wf(
        "run_cancellable_activity",
        workflows::run_cancellable_activity,
    );

    worker.run().await?;

    Ok(())
}
