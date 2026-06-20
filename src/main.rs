use config::Config;
use utils::{
    ch_passwd, create_user, get_password, install_grub, install_network_manager,
    install_pacman_packages, mk_groups, run_command, uncomment_locales, update_sudoers,
};

use crate::utils::{install_paru, run_as_user};

mod config;
mod utils;

const CONFIG_DATA: &str = include_str!("../Config.toml");

fn main() -> anyhow::Result<()> {
    let config: Config = toml::from_str(CONFIG_DATA)?;
    chroot_install(&config)?;
    user_install(&config)?;
    Ok(())
}

fn user_install(config: &Config) -> anyhow::Result<()> {
    println!("Installing paru for AUR packages.");
    run_as_user(&config.linux.username, "git", ["lfs", "install"])?;
    run_as_user(&config.linux.username, "rustup", ["install", "stable"])?;
    install_paru(&config.linux.username)?;
    if let Some(dotfiles) = &config.linux.dotfiles_repo {
        run_as_user(
            &config.linux.username,
            "chezmoi",
            ["init", &dotfiles, "--apply", "--git-lfs"],
        )?;
    }
    Ok(())
}

fn chroot_install(config: &Config) -> anyhow::Result<()> {
    if users::get_current_username().unwrap() != "root" {
        anyhow::bail!("You must run this script as root");
    }
    let hostname = format!("{}-pc", config.linux.username);

    let user_password = get_password("user")?;
    let root_password = get_password("root")?;

    install_pacman_packages(["sudo", "rustup", "git-lfs", "chezmoi", "zsh"], false)?;
    run_command("git", ["lfs", "install"], false)?;
    run_command("git", ["lfs", "pull"], false)?;
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
    std::fs::write(
        "/etc/vconsole.conf",
        format!("KEYMAP={}", config.linux.vconsole_keymap),
    )?;
    std::fs::write("/etc/hostname", hostname)?;
    std::fs::write(
        "/etc/hosts",
        vec!["127.0.0.1\tlocalhost", "::1\tlocalhost"].join("\n"),
    )?;
    run_command("mkinitcpio", ["-P"], false)?;
    // Change root password.
    ch_passwd("root", &root_password)?;

    // Create groups.
    mk_groups(
        config
            .linux
            .groups
            .iter()
            .chain([String::from("wheel")].iter()),
    )?;
    // Create %wheel group.
    update_sudoers()?;
    create_user(
        &config.linux.username,
        config
            .linux
            .groups
            .iter()
            .chain([String::from("wheel")].iter()),
        "/bin/zsh",
    )?;
    ch_passwd(&config.linux.username, &user_password)?;
    install_grub(&config.linux.efi_mountpoint, &config.linux.bootloader_id)?;
    install_network_manager()?;

    Ok(())
}
