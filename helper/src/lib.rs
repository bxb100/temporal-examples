use serde::de;
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;

pub mod clients;
pub mod parse_activity_result;

pub trait PayloadExt {
    fn to_json<'a, T>(&'a self) -> anyhow::Result<T>
    where
        T: de::Deserialize<'a>;

    fn to_str(&self) -> anyhow::Result<String>;
}

impl PayloadExt for Payload {
    fn to_json<'a, T>(&'a self) -> anyhow::Result<T>
    where
        T: de::Deserialize<'a>,
    {
        Ok(serde_json::from_slice::<T>(&self.data)?)
    }

    fn to_str(&self) -> anyhow::Result<String> {
        // use `String::from_utf8(self.data.clone())` not trim `"`
        self.to_json::<String>()
    }
}
