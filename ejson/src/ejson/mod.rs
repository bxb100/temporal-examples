mod binary;
mod date_time;
mod inf_nan;
mod regexp;

pub use binary::BinarySerde;
pub use date_time::DateTimeSerde;
pub use inf_nan::InfNaNSerde;
pub use regexp::RegexpSerde;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use regex::Regex;

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct User {
        id: String,
        age: f32,
        #[serde(with = "InfNaNSerde")]
        hp: f64,
        #[serde(with = "RegexpSerde")]
        matcher: Regex,
        #[serde(with = "BinarySerde")]
        token: Vec<u8>,
        #[serde(with = "DateTimeSerde", rename = "createdAt")]
        created_at: DateTime<Utc>,
    }

    #[test]
    fn test_ejson_deserialization() {
        let json = r#"
{
  "id": "2823dcf6-44a3-4d63-bfa2-7424c899b347",
  "age": 100,
  "hp": {
    "$InfNaN": 1
  },
  "matcher": {
    "$regexp": ".*Stormblessed",
    "$flags": ""
  },
  "token": {
    "$binary": "AQID"
  },
  "createdAt": {
    "$date": 1727201619408
  }
}
        "#;

        let user = serde_json::from_str::<User>(json).unwrap();
        assert_eq!(user.id, "2823dcf6-44a3-4d63-bfa2-7424c899b347");
        assert_eq!(user.age, 100f32);
        assert_eq!(user.hp, f64::INFINITY);
        assert_eq!(user.matcher.as_str(), ".*Stormblessed");
        assert_eq!(user.token, vec![1, 2, 3]);
        assert_eq!(user.created_at.timestamp_millis(), 1727201619408);
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Res {
        success: bool,
        #[serde(with = "DateTimeSerde")]
        at: DateTime<Utc>,
    }

    #[test]
    fn test_ejson_serialization() {
        let json = serde_json::to_string(&Res {
            success: true,
            at: DateTime::from_timestamp_millis(1727201619455).unwrap(),
        })
        .unwrap();

        assert_eq!(json, r#"{"success":true,"at":{"$date":1727201619455}}"#);
    }
}
