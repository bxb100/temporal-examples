use regex::Regex;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt::Formatter;

/// [ejson](https://github.com/meteor/meteor/blob/263c6e694016acb3fa6077f8780b0d4cc29f72d3/packages/ejson/ejson.js#L112-L138)
pub struct RegexpSerde;

impl RegexpSerde {
    pub fn serialize<S>(value: &Regex, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("$regexp", value.as_str())?;
        map.serialize_entry("$flags", "")?;
        map.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Fields {
            Regexp,
            Flags,
        }

        impl<'de> Deserialize<'de> for Fields {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldsVisitor;

                impl<'de> Visitor<'de> for FieldsVisitor {
                    type Value = Fields;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("`$regexp` or `$flags`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match v {
                            "$regexp" => Ok(Fields::Regexp),
                            "$flags" => Ok(Fields::Flags),
                            _ => Err(Error::custom("invalid field")),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldsVisitor)
            }
        }

        struct RegexpVisitor;

        impl<'de> Visitor<'de> for RegexpVisitor {
            type Value = Regex;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a struct with $regexp and $flags fields")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut regexp: Option<&str> = None;
                let mut flags: Option<&str> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::Regexp => {
                            if regexp.is_some() {
                                return Err(Error::duplicate_field("$regexp"));
                            }
                            regexp = Some(map.next_value()?);
                        }
                        Fields::Flags => {
                            if flags.is_some() {
                                return Err(Error::duplicate_field("$flags"));
                            }
                            flags = Some(map.next_value()?);
                        }
                    }
                }

                let regexp = regexp.ok_or_else(|| Error::missing_field("$regexp"))?;
                // todo: how to convert javascript regex to rust regex?
                flags.ok_or_else(|| Error::missing_field("$flags"))?;

                Regex::new(regexp).map_err(|e| Error::custom(e.to_string()))
            }
        }

        let fields = &["$regexp", "$flags"];
        deserializer.deserialize_struct("Regexp", fields, RegexpVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::RegexpSerde;
    use regex::Regex;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestRegexpStruct {
        #[serde(with = "RegexpSerde")]
        v: Regex,
    }

    #[test]
    fn test_regex() {
        let a = TestRegexpStruct {
            v: Regex::new(".*Stormblessed").unwrap()
        };

        let json = serde_json::to_string(&a).unwrap();
        assert_eq!(r#"{"v":{"$regexp":".*Stormblessed","$flags":""}}"#, json);
    }
}
