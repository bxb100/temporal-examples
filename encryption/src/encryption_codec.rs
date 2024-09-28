use crate::aes_gcm::{decrypt, encrypt};
use aes_gcm::{Aes256Gcm, Key};
use anyhow::anyhow;
use log::info;
use prost::Message;
use std::collections::HashMap;
use temporal_sdk_core_protos::temporal::api::common::v1::Payload;
use temporal_sdk_core_protos::ENCODING_PAYLOAD_KEY;
use tokio::sync::OnceCell;

pub static mut ONCE: OnceCell<EncryptionCodec> = OnceCell::const_new();

pub async fn init_codec(key_id: String) -> anyhow::Result<()> {
    unsafe {
        ONCE.set(EncryptionCodec::create(key_id).await)?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct EncryptionCodec {
    default_key_id: String,
    keys: HashMap<String, Key<Aes256Gcm>>,
}

const ENCODING: &str = "binary/encrypted";
const METADATA_ENCRYPTION_KEY_ID: &str = "encryption-key-id";

impl EncryptionCodec {
    pub async fn create(key_id: String) -> Self {
        let mut keys = HashMap::new();
        let key = fetch_key(&key_id).await;
        keys.insert(key_id.to_string(), key);
        EncryptionCodec {
            default_key_id: key_id.to_string(),
            keys,
        }
    }

    pub fn encode(&self, payloads: Vec<&Payload>) -> anyhow::Result<Vec<Payload>> {
        let mut res = Vec::new();

        for payload in payloads {
            info!("Encoding payload: {:?}", payload);
            let mut metadata = HashMap::new();
            metadata.insert(
                ENCODING_PAYLOAD_KEY.to_string(),
                ENCODING.as_bytes().to_vec(),
            );
            metadata.insert(
                METADATA_ENCRYPTION_KEY_ID.to_string(),
                self.default_key_id.as_bytes().to_vec(),
            );

            let msg = payload.encode_to_vec();
            let encrypted_data =
                encrypt(msg.as_slice(), self.keys.get(&self.default_key_id).unwrap())?;

            res.push(Payload {
                metadata,
                data: encrypted_data,
            });
        }

        Ok(res)
    }

    pub fn decode(&mut self, payloads: Vec<&Payload>) -> anyhow::Result<Vec<Payload>> {
        let mut res = Vec::new();
        for payload in payloads {
            if !payload
                .metadata
                .get(ENCODING_PAYLOAD_KEY)
                .map(|data| data.as_slice() == ENCODING.as_bytes())
                .unwrap_or_default()
            {
                res.push(payload.clone());
                continue;
            }
            if payload.data.is_empty() {
                return Err(anyhow!("Payload data is missing"));
            }
            let key_id_bytes = payload.metadata.get(METADATA_ENCRYPTION_KEY_ID);
            if key_id_bytes.is_none() {
                return Err(anyhow!(
                    "Unable to decrypt Payload without encryption key id"
                ));
            }
            let key_id_bytes = key_id_bytes.unwrap();
            let key_id = String::from_utf8(key_id_bytes.to_vec())?;
            info!("{:?}", key_id);
            let key = self.keys.get(&key_id).ok_or(anyhow!("Key not found"))?;
            let decrypted_payload_bytes = decrypt(&payload.data, key)?;
            info!("Decrypting payload.data: {:?}", payload.data);

            res.push(Payload::decode(decrypted_payload_bytes.as_slice())?);
        }

        Ok(res)
    }
}

pub async fn fetch_key(_key_id: &str) -> Key<Aes256Gcm> {
    // In production, fetch key from a key management system (KMS). You may want to memoize requests if you'll be decoding
    // Payloads that were encrypted using keys other than defaultKey
    let key = b"test-key-test-key-test-key-test!";
    Key::<Aes256Gcm>::clone_from_slice(key)
}

#[cfg(test)]
mod tests {
    use super::super::aes_gcm;
    use super::*;
    use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

    #[tokio::test]
    async fn test_encryption_codec() {
        let codec = EncryptionCodec::create("".to_string()).await;
        let payload = "Alice: Private message for Bob.".as_json_payload().unwrap();
        let p = codec.encode(vec![&payload]).unwrap();

        let data = &p.first().unwrap().data;

        let data = aes_gcm::decrypt(data, codec.keys.get(&codec.default_key_id).unwrap()).unwrap();
        println!("{:?}", String::from_utf8(data).unwrap());
    }
}
