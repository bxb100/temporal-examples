pub mod activity_input;
pub mod client;
mod get_workflow_result;
pub mod parse_activity_result;
pub mod wf_context_ext;

pub use client::get_client;
pub use get_workflow_result::get_workflow_result;
pub use parse_activity_result::parse_activity_result;
use std::sync::OnceLock;
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;

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

use temporal_sdk_core::api::telemetry::TelemetryOptionsBuilder;
use temporal_sdk_core::CoreRuntime;

pub fn core_runtime() -> &'static CoreRuntime {
    static CORE_RUNTIME: OnceLock<CoreRuntime> = OnceLock::new();
    CORE_RUNTIME.get_or_init(|| {
        let telemetry_options = TelemetryOptionsBuilder::default().build().unwrap();
        CoreRuntime::new_assume_tokio(telemetry_options).unwrap()
    })
}
