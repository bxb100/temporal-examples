use anyhow::Result;
#[cfg(feature = "custom-logger")]
use custom_logger::telemetry::{get_subscriber, init_subscriber};
use custom_logger::worker;

/// using custom subscriber shouldn't set [CoreRuntime](temporal_sdk_core::CoreRuntime) telemetry
///
/// because this conflict with [telemetry_init](temporal_sdk_core::telemetry::telemetry_init) of [new_assume_tokio](temporal_sdk_core::CoreRuntime::new_assume_tokio)
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    #[cfg(feature = "custom-logger")]
    init_subscriber(get_subscriber("custom-logger", "info", std::io::stdout));

    worker::start_worker().await
}
