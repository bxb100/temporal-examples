use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionPayload {
    pub metadata: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,
}
