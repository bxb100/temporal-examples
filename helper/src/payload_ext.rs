use serde::Deserialize;
use temporal_sdk_core_protos::coresdk::FromJsonPayloadExt;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

pub trait PayloadExt {
    /// see [FromJsonPayloadExt::from_json_payload](FromJsonPayloadExt::from_json_payload)
    fn deserialize<T>(&self) -> anyhow::Result<T>
    where
        T: for<'de> serde::de::Deserialize<'de>;
}

impl PayloadExt for Payload {
    fn deserialize<T>(&self) -> anyhow::Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        // serde_json::from_slice(&self.data)
        Ok(T::from_json_payload(self)?)
    }
}
