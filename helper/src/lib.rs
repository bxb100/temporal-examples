pub mod client;
pub mod parse_activity_result;
pub mod wf_context_ext;

pub use client::get_client;
pub use parse_activity_result::parse_activity_result;

pub fn get_type_name<T>(_: T) -> String {
    T::get_type_name()
}

pub trait TypeName {
    fn get_type_name() -> String;
}

impl<T> TypeName for T {
    fn get_type_name() -> String {
        std::any::type_name::<T>().to_string()
    }
}
