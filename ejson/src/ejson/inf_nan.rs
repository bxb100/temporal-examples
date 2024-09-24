use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserializer, Serializer};
use std::fmt::Formatter;

/// [ejson](https://github.com/meteor/meteor/blob/263c6e694016acb3fa6077f8780b0d4cc29f72d3/packages/ejson/ejson.js#L139-L159)
pub struct InfNaNSerde;

impl InfNaNSerde {
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if value.is_finite() {
            return serializer.serialize_f64(*value);
        }
        // NaN, Inf, -Inf.
        let mut inf_nan = serializer.serialize_struct("InfNaN", 1)?;
        let value = *value;
        if value.is_nan() {
            inf_nan.serialize_field("$InfNaN", &0)?;
        } else if value == f64::INFINITY {
            inf_nan.serialize_field("$InfNaN", &1)?;
        } else if value == f64::NEG_INFINITY {
            inf_nan.serialize_field("$InfNaN", &-1)?;
        } else {
            return Err(serde::ser::Error::custom("invalid value for $InfNaN"));
        }
        inf_nan.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct F64Visitor;

        impl<'de> Visitor<'de> for F64Visitor {
            type Value = f64;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("f64 or a struct with a single field named $InfNaN")
            }

            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some("$InfNaN") = map.next_key()? {
                    let value = map.next_value()?;
                    return match value {
                        0 => Ok(f64::NAN),
                        1 => Ok(f64::INFINITY),
                        -1 => Ok(f64::NEG_INFINITY),
                        _ => Err(Error::custom("invalid value for $InfNaN")),
                    };
                }

                Err(Error::missing_field("$InfNaN"))
            }
        }
        deserializer.deserialize_any(F64Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::InfNaNSerde;

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct TestInfNaNStruct {
        #[serde(with = "InfNaNSerde")]
        v: f64,
    }

    #[test]
    fn test_inf_nan() {
        let json = r#"{"v":{"$InfNaN":0}}"#;

        let a = serde_json::from_str::<TestInfNaNStruct>(json).unwrap();
        assert!(a.v.is_nan());
        let new_struct = serde_json::to_string(&TestInfNaNStruct { v: f64::NAN }).unwrap();
        assert_eq!(json, new_struct);

        let json = r#"{"v":{"$InfNaN":1}}"#;
        let a = serde_json::from_str::<TestInfNaNStruct>(json).unwrap();
        assert_eq!(a.v, f64::INFINITY);
        let new_struct = serde_json::to_string(&TestInfNaNStruct { v: f64::INFINITY }).unwrap();
        assert_eq!(json, new_struct);

        let json = r#"{"v":{"$InfNaN":-1}}"#;
        let a = serde_json::from_str::<TestInfNaNStruct>(json).unwrap();
        assert_eq!(a.v, f64::NEG_INFINITY);
        let new_struct = serde_json::to_string(&TestInfNaNStruct {
            v: f64::NEG_INFINITY,
        })
        .unwrap();
        assert_eq!(json, new_struct);
    }

    #[test]
    fn test_normal_situation() {
        let a = TestInfNaNStruct { v: 100f64 };
        let json = serde_json::to_string(&a).unwrap();
        println!("{json}");

        let obj = serde_json::from_str::<TestInfNaNStruct>(&json).unwrap();
        assert_eq!(obj.v, 100f64);
    }

    #[test]
    fn test_inf_nan_invalid() {
        let json = r#"{"v":{"$InfNaN":2}}"#;
        let a = serde_json::from_str::<TestInfNaNStruct>(json);
        assert!(a.is_err());
    }
}
