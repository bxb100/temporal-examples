use crate::workflow::looping_workflow;
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
        .task_queue("continue-as-new")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "continue-as-new");

    worker.register_wf("looping_workflow", looping_workflow);

    worker.run().await?;

    Ok(())
}
