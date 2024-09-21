use crate::activities::greet;

use crate::workflows::example;
use anyhow::Result;
use helper::core_runtime;
use helper::util::client::get_client;
use helper::worker_ext::WorkerExt;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{init_worker, CoreRuntime, WorkerConfigBuilder};

pub async fn start_worker() -> Result<()> {
    let client = get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("custom-logger")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = if cfg!(feature = "custom-logger") {
        let telemetry_options = TelemetryOptionsBuilder::default().build()?;
        let core_runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
        init_worker(&core_runtime, worker_config, client)?
    } else {
        init_worker(core_runtime(), worker_config, client)?
    };

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "custom-logger".to_string());

    worker
        .register_act(greet)
        .register_workflow("example", example)
        .run()
        .await
}
