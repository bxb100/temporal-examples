pub mod client;
pub mod parse_activity_result;
pub mod wf_context_ext;

pub use client::get_client;
pub use parse_activity_result::parse_activity_result;

pub fn get_type_name<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}
