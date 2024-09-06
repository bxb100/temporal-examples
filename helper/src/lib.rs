pub mod activity_resolution_ext;
pub mod client_ext;
pub mod payload_ext;
pub mod util;
pub mod wf_context_ext;
pub mod worker_ext;

pub use util::client::*;

use std::sync::OnceLock;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::CoreRuntime;

pub fn core_runtime() -> &'static CoreRuntime {
    static CORE_RUNTIME: OnceLock<CoreRuntime> = OnceLock::new();
    CORE_RUNTIME.get_or_init(|| {
        let telemetry_options = TelemetryOptionsBuilder::default().build().unwrap();
        CoreRuntime::new_assume_tokio(telemetry_options).unwrap()
    })
}
