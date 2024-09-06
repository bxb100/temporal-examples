use serde::Deserialize;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

pub trait PayloadExt {
    fn deserialize<'a, T>(&'a self) -> serde_json::Result<T>
    where
        T: serde::de::Deserialize<'a>;
}

impl PayloadExt for Payload {
    fn deserialize<'a, T>(&'a self) -> serde_json::Result<T>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_slice(&self.data)
    }
}
