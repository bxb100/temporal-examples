pub mod activity_resolution_ext;
pub mod client_ext;
pub mod payload_ext;
pub mod payloads_ext;
pub mod util;
pub mod wf_context_ext;
pub mod worker_ext;

pub use util::client::*;

use std::sync::OnceLock;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::telemetry::construct_filter_string;
use temporal_sdk_core::CoreRuntime;
use temporal_sdk_core_api::telemetry::Logger;

static CORE_RUNTIME: OnceLock<CoreRuntime> = OnceLock::new();

pub fn core_runtime() -> &'static CoreRuntime {
    CORE_RUNTIME.get_or_init(|| {
        let default_level = option_env!("RUST_LOG").unwrap_or("info").parse().unwrap();
        // simple console logging, this is will conflict with `set_global_default` or `set_default` in `tracing`
        // see more in <custom-logger/src/bin/main.rs>
        let telemetry_options = TelemetryOptionsBuilder::default()
            .logging(Logger::Console {
                filter: construct_filter_string(default_level, default_level),
            })
            .build()
            .unwrap();
        CoreRuntime::new_assume_tokio(telemetry_options).unwrap()
    })
}
