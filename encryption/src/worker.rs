use crate::encryption_codec::EncryptionCodec;
use crate::workflows::example;
use anyhow::Result;
use std::str::FromStr;
use std::sync::Arc;
use temporal_sdk::{sdk_client_options, Codec, Worker};
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::{init_worker, CoreRuntime, Url, WorkerConfigBuilder};
use tokio::sync::OnceCell;

pub async fn start_worker() -> Result<()> {
    let server_options = sdk_client_options(Url::from_str("http://localhost:7233")?).build()?;

    let client = server_options.connect("default", None).await?;

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue("encryption")
        .worker_build_id("temporal-examples-rs")
        .build()?;
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let core_runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
    let core_worker = init_worker(&core_runtime, worker_config, client)?;

    let mut worker = Worker::new_from_core(Arc::new(core_worker), "encryption".to_string());

    worker.set_codec(Codec {
        encode: Arc::new(Box::new(|payload| unsafe {
            let codec = ONCE.get().expect("Codec not initialized");
            codec.encode(vec![payload]).unwrap().pop().unwrap()
        })),
        decode: Arc::new(Box::new(|p| {
            let codec = unsafe { ONCE.get_mut().expect("Codec not initialized") };
            codec.decode(vec![p]).unwrap().pop().unwrap()
        })),
    });

    worker.register_wf("example", example);
    worker.run().await
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
    let codec = EncryptionCodec::create("test-key-id".to_string());
    ONCE.set(codec).expect("Failed to set global codec");
}
