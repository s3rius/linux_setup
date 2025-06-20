use clap::Parser;
use cli::Cli;
use config::Config;
use custom_package::CustomPackage;
use utils::{
    ch_passwd, create_user, enable_services, get_password, install_aur_packages, install_grub,
    install_network_manager, install_pacman_packages, mk_groups, run_command, self_install_chroot,
    uncomment_locales, update_sudoers,
};

use crate::utils::instll_ldfm;

mod cli;
mod config;
mod custom_package;
mod utils;
mod wm;

const CONFIG_DATA: &str = include_str!("../Config.toml");

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config: Config = toml::from_str(CONFIG_DATA)?;
    match args {
        Cli::Chroot => chroot_install(&config)?,
        Cli::User => user_install(&config)?,
    }
    Ok(())
}

fn user_install(config: &Config) -> anyhow::Result<()> {
    println!("Installing paru for AUR packages.");
    run_command("git", ["lfs", "install"], false)?;
    run_command("rustup", ["install", "stable"], false)?;

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "arm") {
        "armv7h"
    } else {
        anyhow::bail!("Unsupported architecture");
    };
    // Installing AUR helper.
    CustomPackage::HttpFile {
        url: format!(
            "https://github.com/Morganamilo/paru/releases/download/{version}/paru-{version}-{arch}.tar.zst",
            version=config.packages.paru_version,
            arch=arch,
        ),
        install_command: format!(
            "tar xvf paru-{version}-{arch}.tar.zst && ./paru -Syu --noconfirm paru-bin",
            version=config.packages.paru_version,
            arch=arch,
        ),
        skip_if_exists: Some("/usr/bin/paru".to_string())
    }
        .install()?;

    if let Some(df_repo) = &config.linux.dotfiles_repo {
        instll_ldfm(df_repo)?;
    }

    wm::hypr::install_hyprland()?;

    install_pacman_packages(&config.packages.pacman, true)?;
    install_aur_packages(&config.packages.aur)?;
    for package in &config.custom_packages {
        package.install()?;
    }
    enable_services(config.linux.services.iter(), true)?;

    Ok(())
}

fn chroot_install(config: &Config) -> anyhow::Result<()> {
    if users::get_current_username().unwrap() != "root" {
        anyhow::bail!("You must run this script as root");
    }
    let hostname = format!("{}-pc", config.linux.username);

    let user_password = get_password("user")?;
    let root_password = get_password("root")?;

    install_pacman_packages(["sudo", "rustup", "git-lfs"], false)?;
    run_command("git", ["lfs", "install"], false)?;
    run_command("git", ["lfs", "pull"], false)?;
    install_pacman_packages(config.packages.pacman.iter(), false)?;
    // Setting currene timezone.
    println!("Setting timezone to {}", config.linux.timezone);
    std::fs::remove_file("/etc/localtime").ok();
    std::os::unix::fs::symlink(
        format!("/usr/share/zoneinfo/{}", config.linux.timezone),
        "/etc/localtime",
    )?;
    // Set correct time.
    run_command("hwclock", ["--systohc"], false)?;
    // Set up locales.
    uncomment_locales(config.linux.locales.iter())?;
    run_command::<String>("locale-gen", [], false)?;
    // Update networking essentials.
    std::fs::write("/etc/hostname", hostname)?;
    std::fs::write(
        "/etc/hosts",
        vec!["127.0.0.1\tlocalhost", "::1\tlocalhost"].join("\n"),
    )?;
    run_command("mkinitcpio", ["-p"], false)?;
    // Change root password.
    ch_passwd("root", &root_password)?;

    // Create groups.
    mk_groups(config.linux.groups.iter())?;
    // Create %wheel group.
    update_sudoers()?;
    create_user(
        &config.linux.username,
        config.linux.groups.iter(),
        "/bin/zsh",
    )?;
    ch_passwd(&config.linux.username, &user_password)?;
    install_grub(&config.linux.efi_mountpoint, &config.linux.bootloader_id)?;
    install_network_manager()?;
    self_install_chroot()?;

    Ok(())
}
