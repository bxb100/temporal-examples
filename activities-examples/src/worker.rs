use helper::{core_runtime, get_type_name};
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::init_worker;
use temporal_sdk_core_api::worker::WorkerConfigBuilder;

use crate::activities::*;
use crate::workflows::*;

pub async fn start_worker() -> Result<(), Box<dyn std::error::Error>> {
    let client = helper::client::get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("activities-examples")
        .worker_build_id("core-worker")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "activities-examples");

    let (l, r) = get_type_name(make_http_request);
    worker.register_activity(l, r);
    let (l, r) = get_type_name(do_something_async);
    worker.register_activity(l, r);

    worker.register_wf("http_workflow", http_workflow);
    worker.register_wf("async_activity_workflow", async_activity_workflow);

    worker.run().await?;

    Ok(())
}
