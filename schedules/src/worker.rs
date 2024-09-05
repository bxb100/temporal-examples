use crate::activities::{add_reminder_to_database, notify_user};
use crate::workflow::reminder;
use helper::worker_ext::WorkerExt;
use helper::{core_runtime, get_client};
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, WorkerConfigBuilder};

pub async fn start_worker() -> anyhow::Result<()> {
    let client = get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("schedules")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "schedules".to_string());

    worker
        .register_act(add_reminder_to_database)
        .register_act(notify_user)
        .register_workflow("reminder", reminder)
        .run()
        .await
}
