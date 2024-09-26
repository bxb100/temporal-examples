use aes_gcm::aead::consts::U12;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::anyhow;

/// TAG_LENGTH_BYTES = 16 means
pub fn encrypt(data: &[u8], key: &Key<Aes256Gcm>) -> anyhow::Result<Vec<u8>> {
    let nonce: Nonce<U12> = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);

    let mut encrypt_data = cipher.encrypt(&nonce, data).map_err(|e| anyhow!("{e}"))?;
    let mut res = nonce.to_vec();
    res.append(&mut encrypt_data);
    Ok(res)
}

pub fn decrypt(data: &[u8], key: &Key<Aes256Gcm>) -> anyhow::Result<Vec<u8>> {
    let (nonce, cipher_text) = data.split_at(12);
    let cipher = Aes256Gcm::new(key);
    cipher
        .decrypt(GenericArray::from_slice(nonce), cipher_text.as_ref())
        .map_err(|e| anyhow!("{e}"))
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};
    use aes_gcm::{
        aead::{KeyInit, OsRng},
        Aes256Gcm, Key,
    };
    use base64::prelude::BASE64_STANDARD;
    use base64::Engine;
    use temporal_sdk_core_protos::temporal::api::common::v1::Payload;

    #[test]
    fn ase_gcm_usage() -> anyhow::Result<()> {
        let key = Aes256Gcm::generate_key(OsRng);

        let ciphertext = encrypt(b"Alice: Private message for Bob.", &key)?;
        let plaintext = decrypt(&ciphertext, &key)?;

        assert_eq!(plaintext, b"Alice: Private message for Bob.");

        Ok(())
    }

    use temporal_sdk_core_protos::coresdk::FromJsonPayloadExt;
    use prost::Message;
    
    #[test]
    fn test_decrypt() -> anyhow::Result<()> {
        let data = "AMhwQ0cyyvbrvqu0FG1kM2AsHehe5Q3VoThOUpiyqva/ybgdOREhFg5fX+Q5MXlCm58H0CBLqJQG1jGt3SsdY1TxQk3XzHM6OnZDB829AlCzt1FmPsQN";
        let data = BASE64_STANDARD.decode(data)?;
        let key = Key::<Aes256Gcm>::from_slice(b"test-key-test-key-test-key-test!");

        let plaintext = decrypt(&data, key)?;

        let payload = Payload::decode(plaintext.as_slice())?;
        println!("{:?}", payload);
        print!("{:?}", String::from_json_payload(&payload)?);

        Ok(())
    }
}
