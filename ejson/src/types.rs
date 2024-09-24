use crate::ejson::*;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub age: f32,
    #[serde(with = "InfNaNSerde")]
    pub hp: f64,
    #[serde(with = "RegexpSerde")]
    pub matcher: Regex,
    #[serde(with = "BinarySerde")]
    pub token: Vec<u8>,
    #[serde(with = "DateTimeSerde", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Res {
    pub success: bool,
    #[serde(with = "DateTimeSerde")]
    pub at: DateTime<Utc>,
}
