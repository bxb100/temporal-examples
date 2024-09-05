use crate::workflow::looping_workflow;
use helper::core_runtime;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, WorkerConfigBuilder};

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::client::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("continue-as-new")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "continue-as-new");

    worker.register_wf("looping_workflow", looping_workflow);

    worker.run().await?;

    Ok(())
}
