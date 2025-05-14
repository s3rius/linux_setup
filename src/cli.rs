#[derive(clap::Parser)]
pub struct ChrootInstallArgs {
    #[arg(long, default_value = "s3rius")]
    pub username: String,

    #[arg(long, default_value = "x86_64-efi")]
    pub efi_target: String,

    #[arg(long, default_value = "/boot/EFI")]
    pub efi_mountpoint: String,

    #[arg(long, default_value = "I use arch BTW")]
    pub bootloader_id: String,

    #[arg(long, default_value = "Europe/Belgrade")]
    pub timezone: String,

    #[arg(long, default_value = "en_US.UTF-8,ru_RU.UTF-8", value_delimiter = ',')]
    pub locales: Vec<String>,
}

#[derive(clap::Parser)]
pub enum Cli {
    Chroot(ChrootInstallArgs),
    User {},
    Sync {},
}
