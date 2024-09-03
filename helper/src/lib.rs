pub mod client;
mod get_workflow_result;
pub mod parse_activity_result;
pub mod wf_context_ext;

pub use client::get_client;
pub use get_workflow_result::get_workflow_first_result;
pub use parse_activity_result::parse_activity_result;
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
