use crate::activities::greet;
use crate::workflows::example;
use anyhow::Result;
use helper::worker_ext::WorkerExt;
use temporal_sdk::Worker;
use temporal_sdk_core::WorkerConfigBuilder;

pub async fn start_worker() -> Result<()> {
    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("hello-world")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    Worker::single(worker_config)
        .await?
        .register_act(greet)
        .register_workflow("example", example)
        .run()
        .await
}
