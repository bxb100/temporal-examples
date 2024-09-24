use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::fmt::Formatter;

/// [ejson](https://github.com/meteor/meteor/blob/263c6e694016acb3fa6077f8780b0d4cc29f72d3/packages/ejson/ejson.js#L161-L173)
pub struct BinarySerde;

impl BinarySerde {
    pub fn serialize<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("$binary", &BASE64_STANDARD.encode(value))?;
        map.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BinaryVisitor;

        impl<'de> Visitor<'de> for BinaryVisitor {
            type Value = Vec<u8>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a struct with a single field named $binary")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some("$binary") = map.next_key()? {
                    let value: String = map.next_value()?;
                    return Ok(BASE64_STANDARD
                        .decode(value.as_bytes())
                        .map_err(|e| Error::custom(e.to_string()))?);
                }

                Err(Error::missing_field("$binary"))
            }
        }

        deserializer.deserialize_map(BinaryVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestBinary {
        #[serde(with = "BinarySerde")]
        v: Vec<u8>,
    }

    #[test]
    fn test_binary() {
        let data = vec![1, 2, 3];
        let json = serde_json::to_string(&TestBinary { v: data.clone() }).unwrap();
        assert_eq!(json, r#"{"v":{"$binary":"AQID"}}"#);
        let de: TestBinary = serde_json::from_str(&json).unwrap();
        assert_eq!(de.v, data);
    }
}
