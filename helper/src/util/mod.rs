pub mod activity_input;
pub mod client;

pub fn get_mod_simple_name<T>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.split("::").last().unwrap()
}