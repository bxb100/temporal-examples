pub mod worker;
pub mod workflows;

pub struct SearchAttributesWrapper(pub HashMap<String, Payload>);

use helper::payload_ext::PayloadExt;
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::HashMap;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

/// This is purely for demonstration purposes, we lose all payloads' type information.
impl Serialize for SearchAttributesWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = self.0.clone();
        let mut state = serializer.serialize_map(Some(map.len()))?;

        for (field, payload) in map {
            state.serialize_entry(&field, &payload.deserialize::<serde_json::Value>().unwrap())?;
        }
        state.end()
    }
}
