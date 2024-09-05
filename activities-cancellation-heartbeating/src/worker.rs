use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::init_worker;
use temporal_sdk_core_api::worker::WorkerConfigBuilder;

use crate::activities;
use crate::workflows;
use helper::core_runtime;
use helper::worker_ext::WorkerExt;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("cancellation-heartbeating")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), "cancellation-heartbeating");

    worker
        .register_act(activities::fake_progress)
        .register_workflow(
            "run_cancellable_activity",
            workflows::run_cancellable_activity,
        )
        .run()
        .await?;

    Ok(())
}
