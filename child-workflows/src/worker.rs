use crate::workflows::*;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{init_worker, CoreRuntime, WorkerConfigBuilder};

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::client::get_client().await?;
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("child-workflows")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "child-workflows");

    worker.register_wf("parent_workflow", parent_workflow);
    worker.register_wf("child_workflow", child_workflow);

    worker.run().await?;

    Ok(())
}
