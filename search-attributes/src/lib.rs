pub mod worker;
pub mod workflows;

use helper::payload_ext::PayloadExt;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

/// This is purely for demonstration purposes, we lose all payloads' type information.
pub struct SearchAttributesWrapper(pub BTreeMap<String, Payload>);

impl From<HashMap<String, Payload>> for SearchAttributesWrapper {
    fn from(map: HashMap<String, Payload>) -> Self {
        Self(map.into_iter().collect())
    }
}

impl Display for SearchAttributesWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        write!(f, "{s}")
    }
}

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

struct SearchAttributesWrapperMapVisitor;

impl<'de> Visitor<'de> for SearchAttributesWrapperMapVisitor {
    type Value = SearchAttributesWrapper;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("need a map of search attributes")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut search_attributes = BTreeMap::new();
        while let Some((key, value)) = map.next_entry::<String, serde_json::Value>()? {
            search_attributes.insert(
                key,
                value.as_json_payload().map_err(serde::de::Error::custom)?,
            );
        }
        Ok(SearchAttributesWrapper(search_attributes))
    }
}

impl<'de> Deserialize<'de> for SearchAttributesWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(SearchAttributesWrapperMapVisitor)
    }
}
