use crate::encryption_codec::EncryptionCodec;
use crate::workflows::example;
use anyhow::Result;
use helper::core_runtime;
use helper::util::client::get_client;
use helper::worker_ext::WorkerExt;
use std::sync::Arc;
use temporal_sdk::Worker;
use temporal_sdk_core::{init_worker, WorkerConfigBuilder};
use tokio::sync::OnceCell;

pub async fn start_worker() -> Result<()> {
    let client = get_client().await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("encryption")
        .worker_build_id("temporal-examples-rs")
        .build()?;

    let core_worker = init_worker(core_runtime(), worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "encryption".to_string());

    worker
        .register_workflow("example", Box::new(example))
        .run()
        .await
}

pub static mut ONCE: OnceCell<EncryptionCodec> = OnceCell::const_new();

/// `WfContext` not exist `Dataconverter` and `app_data`
/// 
/// todo: using serde is the best way
///
/// # SAFETY
/// 
/// This function is unsafe because it uses a mutable static variable.
pub async unsafe fn init_codec() {
    let codec = EncryptionCodec::create("test-key-id".to_string()).await;
    ONCE.set(codec).expect("Failed to set global codec");
}
