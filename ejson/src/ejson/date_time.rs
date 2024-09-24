use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer};
use std::fmt::Formatter;

/// [ejson](https://github.com/meteor/meteor/blob/263c6e694016acb3fa6077f8780b0d4cc29f72d3/packages/ejson/ejson.js#L99-L110)
pub struct DateTimeSerde;

impl DateTimeSerde {
    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        // we use timestamp_millis() according to the ejson spec
        // so attention not to use DateTime PartialEq
        map.serialize_entry("$date", &value.timestamp_millis())?;
        map.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = DateTime<Utc>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a struct with a single field named $date")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some("$date") = map.next_key()? {
                    let timestamp: i64 = map.next_value()?;
                    return Ok(DateTime::from_timestamp_millis(timestamp).unwrap());
                }

                Err(Error::missing_field("$date"))
            }
        }
        deserializer.deserialize_map(DateVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct TestDateTime {
        #[serde(with = "DateTimeSerde")]
        v: DateTime<Utc>,
    }

    #[test]
    fn test_date_time() {
        let now = Utc::now();
        let json = serde_json::to_string(&TestDateTime { v: now }).unwrap();
        println!("{}", json);
        let test = serde_json::from_str::<TestDateTime>(&json).unwrap();
        assert_eq!(test.v.timestamp_millis(), now.timestamp_millis());
    }
}
