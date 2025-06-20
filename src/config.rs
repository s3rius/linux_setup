use crate::custom_package::CustomPackage;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Packages {
    pub paru_version: String,
    pub pacman: Vec<String>,
    pub aur: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Linux {
    // Username to create.
    pub username: String,
    // Groups to add the user to.
    pub groups: Vec<String>,
    // Services to enable.
    pub services: Vec<String>,
    // Where to install the bootloader.
    // Should be mounted to /boot/EFI.
    pub efi_mountpoint: String,
    // Name of the bootloader.
    pub bootloader_id: String,
    // Timezone to set.
    pub timezone: String,
    // Language to set.
    pub locales: Vec<String>,
    // Path to the dotfiles repository.
    pub dotfiles_repo: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub linux: Linux,
    pub packages: Packages,
    pub custom_packages: Vec<CustomPackage>,
}
