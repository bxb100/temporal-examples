use crate::activities::{activity1, activity2, activity3, activity4, activity5};
use crate::workflows::example;
use anyhow::Result;
use helper::core_runtime;
use helper::util::client::get_client;
use helper::worker_ext::WorkerExt;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, WorkerConfigBuilder};

pub async fn start_worker() -> Result<()> {
    let client = get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("dsl-interpreter")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "dsl-interpreter".to_string());

    worker
        .register_act(activity1)
        .register_act(activity2)
        .register_act(activity3)
        .register_act(activity4)
        .register_act(activity5)
        .register_workflow("example", example)
        .run()
        .await
}
