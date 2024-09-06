use helper::core_runtime;
use helper::worker_ext::WorkerExt;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::init_worker;
use temporal_sdk_core_api::worker::WorkerConfigBuilder;

use crate::activities::*;
use crate::workflows::*;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::util::client::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("activities-examples")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "activities-examples");

    worker
        .register_act(make_http_request)
        .register_act(do_something_async)
        .register_workflow("http_workflow", http_workflow)
        .register_workflow("async_activity_workflow", async_activity_workflow)
        .run()
        .await?;

    Ok(())
}
