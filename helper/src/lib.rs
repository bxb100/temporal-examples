pub mod activity_resolution_ext;
pub mod client_ext;
pub mod util;
pub mod wf_context_ext;
pub mod worker_ext;

pub use util::client::*;

use std::sync::OnceLock;
use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;
use temporal_sdk_core::CoreRuntime;

pub fn get_type_name<T>(t: T) -> (&'static str, T) {
    (T::get_type_name(), t)
}

pub trait TypeName {
    fn get_type_name() -> &'static str;
}

impl<T: ?Sized> TypeName for T {
    fn get_type_name() -> &'static str {
        std::any::type_name::<T>()
    }
}

pub fn payload_into<'a, T>(payload: &'a Payload) -> serde_json::Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    serde_json::from_slice(&payload.data)
}

pub fn core_runtime() -> &'static CoreRuntime {
    static CORE_RUNTIME: OnceLock<CoreRuntime> = OnceLock::new();
    CORE_RUNTIME.get_or_init(|| {
        let telemetry_options = TelemetryOptionsBuilder::default().build().unwrap();
        CoreRuntime::new_assume_tokio(telemetry_options).unwrap()
    })
}
