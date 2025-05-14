#[derive(clap::Parser)]
pub struct ChrootInstallArgs {
    /// Username for the default user.
    /// PC name is going to be derived from it.
    #[arg(long, default_value = "s3rius")]
    pub username: String,

    /// bootloader target efi.
    #[arg(long, default_value = "x86_64-efi")]
    pub efi_target: String,

    /// Where to install the bootloader.
    #[arg(long, default_value = "/boot/EFI")]
    pub efi_mountpoint: String,

    /// Name of the bootloader.
    #[arg(long, default_value = "I use arch BTW")]
    pub bootloader_id: String,

    /// Timezone to set.
    #[arg(long, default_value = "Europe/Belgrade")]
    pub timezone: String,

    /// Supported locales.
    /// The first one is going to be used as a default.
    #[arg(long, default_value = "en_US.UTF-8,ru_RU.UTF-8", value_delimiter = ',')]
    pub locales: Vec<String>,
}

#[derive(clap::Parser)]
pub struct UserArgs {
    /// Repository URL for the dotfiles. It's going to be used
    /// to clone the dotfiles repository in .config/linux_setup.
    #[arg(long, default_value = std::env!("CARGO_PKG_REPOSITORY"))]
    pub repo_url: String,

    /// Where git repo is going to be cloned.
    /// Used by `sync` command to update your dotfiles.
    ///
    /// Defeaults to $HOME/.config/linux_setup.
    #[arg(long, default_value = "$HOME/.config/linux_setup")]
    pub configs_path: Option<String>,
}

#[derive(clap::Parser)]
pub enum Cli {
    /// Command to run in chroot,
    /// right after the disc layout.
    Chroot(ChrootInstallArgs),
    /// Command to run on first boot.
    User(UserArgs),
    /// Sync files from your PC with repo's dotfiles,
    /// accroding to the `DOTFILES_MAPPING`.
    Sync,
}
