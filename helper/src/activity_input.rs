use serde::{de::DeserializeOwned, Deserialize, Serialize};
use temporal_sdk_core::protos::temporal::api::common::v1::Payload;
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct ActivityInput<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<T>,
}

pub fn none() -> ActivityInput<String> {
    ActivityInput::<String> { input: None }
}

impl<T: Serialize + DeserializeOwned> ActivityInput<T> {
    pub fn new(input: T) -> Self {
        Self { input: Some(input) }
    }
}

impl<T> TryInto<Payload> for ActivityInput<T>
where
    T: Serialize,
    ActivityInput<T>: Serialize,
{
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Payload, Self::Error> {
        self.as_json_payload()
    }
}
