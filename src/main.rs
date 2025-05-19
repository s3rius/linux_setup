use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use config::Config;
use custom_package::CustomPackage;
use dotfiles::Dotfiles;
use utils::{
    ch_passwd, create_user, enable_services, get_password, git_commit, git_pull, git_push,
    install_aur_packages, install_grub, install_network_manager, install_pacman_packages,
    install_self_bin, mk_groups, path_shrink, run_command, self_install_chroot, self_install_user,
    uncomment_locales, update_sudoers,
};

mod cli;
mod config;
mod custom_package;
mod dotfiles;
mod utils;
mod wm;

const CONFIG_DATA: &str = include_str!("../Config.toml");

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let config: Config = toml::from_str(CONFIG_DATA)?;
    match args {
        Cli::Chroot => chroot_install(&config)?,
        Cli::User => user_install(&config)?,
        Cli::Sync { commit, push } => sync_files(commit, push, &config)?,
        Cli::Pull => pull(&config)?,
        Cli::Apply => apply(&config)?,
        Cli::Add { file } => add_to_dotfiles_mapping(&file, &config)?,
    }
    Ok(())
}

fn add_to_dotfiles_mapping(file: &PathBuf, config: &Config) -> anyhow::Result<()> {
    let mut config = config.clone();
    let file_name = file
        .file_name()
        .expect("Should be a path to a file")
        .display()
        .to_string();
    let file_path = path_shrink(file)?.display().to_string();
    config.dotfiles.insert(file_name, file_path);
    config.dump(PathBuf::from(&config.linux.configs_path).join("Config.toml"))?;
    sync_files(false, false, &config)?;
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

    wm::hypr::install_hyprland()?;

    apply(&config)?;

    self_install_user(&config)?;

    Ok(())
}

pub fn pull(config: &Config) -> anyhow::Result<()> {
    git_pull(&config.linux.configs_path)?;
    install_self_bin(&config.linux.configs_path)?;
    run_command(std::env!("CARGO_BIN_NAME"), ["apply"], false)?;

    Ok(())
}

pub fn apply(config: &Config) -> anyhow::Result<()> {
    install_pacman_packages(&config.packages.pacman, true)?;
    install_aur_packages(&config.packages.aur)?;
    for package in &config.custom_packages {
        package.install()?;
    }
    Dotfiles::copy(&config.dotfiles)?;
    enable_services(config.linux.services.iter(), true)?;
    Ok(())
}

fn sync_files(commit: bool, push: bool, config: &Config) -> anyhow::Result<()> {
    let main_folder = env!("CARGO_MANIFEST_DIR");
    let dotfiles_folder = PathBuf::from(format!("{main_folder}/dotfiles"));
    std::fs::remove_dir_all(&dotfiles_folder).ok();
    std::fs::create_dir_all(&dotfiles_folder).ok();
    println!("Syncing dotfiles for {dotfiles_folder:?}");
    for (local_path, sys_path) in config.dotfiles.iter() {
        let sys_path = PathBuf::from(shellexpand::full(sys_path)?.to_string());
        let mut target_path = dotfiles_folder.join(local_path);
        if target_path.exists() {
            if target_path.is_dir() {
                std::fs::remove_dir_all(&target_path).ok();
            } else {
                std::fs::remove_file(&target_path).ok();
            }
        }
        println!(
            "Copying {} to {}",
            sys_path.display(),
            target_path.display()
        );
        if !sys_path.exists() {
            println!("Source file does not exist: {}", sys_path.display());
            continue;
        }
        if sys_path.is_dir() {
            target_path.pop();
        }
        run_command(
            "cp",
            [
                "-r",
                sys_path.display().to_string().as_str(),
                target_path.display().to_string().as_str(),
            ],
            false,
        )?;
    }
    // Cleanup private things.
    for file in dotfiles_folder.join(".zfunc").read_dir()?.flatten() {
        if file.file_name().to_string_lossy().starts_with("_") {
            println!("Removing private file: {}", file.path().display());
            std::fs::remove_file(file.path())?;
        }
    }
    std::fs::remove_file(dotfiles_folder.join("kitty/kitty.conf.bak")).ok();
    std::fs::remove_file(dotfiles_folder.join("nvim/lua/config/intree.lua")).ok();

    if commit {
        git_commit(main_folder, "Sync dotfiles")?;
    }
    if push {
        git_push(main_folder)?;
    }

    install_self_bin(&config.linux.configs_path)?;

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
