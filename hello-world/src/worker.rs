use crate::activities::greet;
use crate::workflows::example;
use anyhow::Result;
use helper::clients::get_client;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{init_worker, CoreRuntime, WorkerConfigBuilder};

pub async fn start_worker() -> Result<()> {
    let client = get_client().await?;

    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("hello-world")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "hello-world".to_string());

    worker.register_activity("activities", greet);
    worker.register_wf("example", example);

    worker.run().await
}
