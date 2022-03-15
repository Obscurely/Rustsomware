use dirs;
use mountpoints;
use std::fs;
use std::str;

const NOT_IMPORTANT_DIRS: [&str; 22] = [
    "Microsoft",
    "PerfLogs",
    "Program Files",
    "Program Files (x86)",
    "ProgramData",
    "Recovery",
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
    // Creates vars with the paths of the common folder used in windows for data storage.
    // Creates the documents path var.
    let documents_path = match dirs::document_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Creates the picture path var.
    let pictures_path = match dirs::picture_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Creates the videos path var.
    let video_path = match dirs::video_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Creates the audio path var.
    let audio_path = match dirs::audio_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Creates the downloads path var.
    let downloads_path = match dirs::download_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Creates the desktop path var.
    let desktop_path = match dirs::desktop_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => path_str.to_owned(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    // Clones the vars because we are adding the previous ones to the vector.
    // (could just add to the vector after, but is easier to understand this way. IMO)
    let documents_path_clone = documents_path.clone();
    let pictures_path_clone = pictures_path.clone();
    let video_path_clone = video_path.clone();
    let audio_path_clone = audio_path.clone();
    let downloads_path_clone = downloads_path.clone();
    let desktop_path_clone = desktop_path.clone();

    // Adds the found paths to the Vector.
    vec.push(documents_path);
    vec.push(pictures_path);
    vec.push(video_path);
    vec.push(audio_path);
    vec.push(desktop_path);
    vec.push(downloads_path);

    // Adds all the remaining folders in the home folder (that weren't yet added)
    match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(home_path) => match fs::read_dir(home_path) {
                Ok(read) => {
                    for path in read.filter_map(|file| file.ok()) {
                        let path = path.path();
                        if path.is_dir() {
                            let path = path.display().to_string();
                            let path_split: Vec<&str> = path.split("\\").collect();
                            let path_last_part = path_split.last().unwrap_or(&"Documents");
                            if path_last_part == &"Documents" && &path == &documents_path_clone {
                                continue;
                            } else if path_last_part == &"Pictures" && &path == &pictures_path_clone
                            {
                                continue;
                            } else if path_last_part == &"Videos" && &path == &video_path_clone {
                                continue;
                            } else if path_last_part == &"Music" && &path == &audio_path_clone {
                                continue;
                            } else if path_last_part == &"Desktop" && &path == &desktop_path_clone {
                                continue;
                            } else if path_last_part == &"Downloads"
                                && &path == &downloads_path_clone
                            {
                                continue;
                            } else if path_last_part == &"AppData" {
                                continue;
                            }

                            vec.push(path);
                        }
                    }
                }
                Err(_) => (),
            },
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

    // Add other users paths
    match dirs::home_dir() {
        Some(home_path) => match home_path.file_name() {
            Some(current_user) => {
                let current_user = current_user.to_str().unwrap();
                let to_replace = String::from("\\") + &current_user;
                match fs::read_dir(home_path.to_str().unwrap().replace(&to_replace, "")) {
                    Ok(read) => {
                        for folder in read.filter_map(|file| file.ok()) {
                            let folder = folder.path();
                            println!("{}", &folder.display().to_string());
                            if folder.is_dir() {
                                let folder = folder.display().to_string();
                                let folder_split: Vec<&str> = folder.split("\\").collect();
                                let folder_last_part = folder_split.last().unwrap();
                                // Excluding Public dir as well since it's already added to the vector.
                                if folder_last_part == &current_user
                                    || folder_last_part == &"Public"
                                {
                                    continue;
                                }

                                match fs::read_dir(folder) {
                                    Ok(read) => {
                                        for f in read.filter_map(|file| file.ok()) {
                                            let f = f.path();
                                            if f.is_dir() {
                                                let folder_name =
                                                    f.file_name().unwrap().to_str().unwrap();
                                                if &folder_name != &"AppData" {
                                                    // Excluding the AppData folder since we want to delete that.
                                                    vec.push(f.display().to_string())
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => (),
                                }
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
            None => (),
        },
        None => (),
    }
}

fn add_possible_important_folder_paths_on_drive_windows(
    drive_mount_path: &String,
    vec: &mut Vec<String>,
) {
    let drive_dirs = match fs::read_dir(drive_mount_path) {
        Ok(dirs) => dirs,
        Err(_) => return,
    };

    for path in drive_dirs.filter_map(|file| file.ok()) {
        let path = path.path();

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

    for path in drive_dirs.filter_map(|file| file.ok()) {
        let path = path.path();

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

    // AppData folder for the other users, the one that we skipped in the above function because we want to delete the files in it.
    // Add other users paths
    match dirs::home_dir() {
        Some(home_path) => match home_path.file_name() {
            Some(current_user) => {
                let current_user = current_user.to_str().unwrap();
                let to_replace = String::from("\\") + &current_user;
                match fs::read_dir(home_path.to_str().unwrap().replace(&to_replace, "")) {
                    Ok(read) => {
                        for folder in read.filter_map(|file| file.ok()) {
                            let folder = folder.path();
                            println!("{}", &folder.display().to_string());
                            if folder.is_dir() {
                                let folder = folder.display().to_string();
                                let folder_split: Vec<&str> = folder.split("\\").collect();
                                let folder_last_part = folder_split.last().unwrap();
                                if folder_last_part == &current_user
                                    || folder_last_part == &"Public"
                                {
                                    continue;
                                }

                                match fs::read_dir(folder) {
                                    Ok(read) => {
                                        for f in read.filter_map(|file| file.ok()) {
                                            let f = f.path();
                                            if f.is_dir() {
                                                let folder_name =
                                                    f.file_name().unwrap().to_str().unwrap();
                                                if &folder_name == &"AppData" {
                                                    vec.push(f.display().to_string())
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => (),
                                }
                            }
                        }
                    }
                    Err(_) => (),
                }
            }
            None => (),
        },
        None => (),
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

pub fn get_users_windows() -> Vec<String> {
    let mut users: Vec<String> = vec![];
    match fs::read_dir("C:\\Users") {
        Ok(read) => {
            for folder in read.filter_map(|file| file.ok()) {
                let folder = folder.path();
                if folder.is_dir() {
                    users.push(folder.display().to_string());
                }
            }
        }
        Err(_) => (),
    }

    users
}
