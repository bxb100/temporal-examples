use crate::workflows::*;
use helper::core_runtime;
use helper::worker_ext::WorkerExt;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, WorkerConfigBuilder};

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::util::client::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("child-workflows")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "child-workflows");

    worker
        .register_workflow("parent_workflow", parent_workflow)
        .register_workflow("child_workflow", child_workflow)
        .run()
        .await?;

    Ok(())
}
