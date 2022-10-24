use crate::global::structs::Errors;
use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead},
    Aes256GcmSiv, KeyInit, Nonce,
};
use sha2::{Digest, Sha384};
use std::error::Error;

// AES GCM SIV

pub fn decipher_slice(
    ciphertext: &str,
    key: &str,
    nounce: &[u8],
) -> Result<String, Box<dyn Error>> {
    let key = GenericArray::from_iter(hex::decode(key)?.into_iter());
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(nounce);

    match cipher.decrypt(nonce, hex::decode(ciphertext)?.as_ref()) {
        Ok(decrypted) => Ok(String::from_utf8(decrypted)?),
        Err(_) => Err(Box::new(Errors::DecrpytionError)),
    }
}

pub fn cipher_slice(plaintext: &str, key: &str, nounce: &[u8]) -> Result<String, Box<dyn Error>> {
    let key = GenericArray::from_iter(hex::decode(key)?.into_iter());
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(nounce);

    match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(encrypted) => Ok(hex::encode(encrypted)),
        Err(_) => Err(Box::new(Errors::EncryptionError)),
    }
}

pub fn decipher(
    ciphertext: &str,
    key: &str,
    mut nounce: Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    nounce.resize(12, 0);
    decipher_slice(ciphertext, key, nounce.as_slice())
}

pub fn cipher(plaintext: &str, key: &str, mut nounce: Vec<u8>) -> Result<String, Box<dyn Error>> {
    nounce.resize(12, 0);
    cipher_slice(plaintext, key, nounce.as_slice())
}

// SHA384

pub fn sha384(plaintext: &str) -> Vec<u8> {
    let mut hasher = Sha384::new();
    hasher.update(plaintext);
    hasher.finalize().to_vec()
}
