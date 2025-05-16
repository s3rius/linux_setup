use std::{cell::LazyCell, collections::HashMap, fs::read_to_string, io::Write, path::PathBuf};

use clap::Parser;
use cli::{ChrootInstallArgs, Cli, UserArgs};
use custom_package::CustomPackage;
use dotfiles::Dotfiles;
use utils::{
    ch_passwd, create_user, enable_services, get_password, git_commit, git_pull, git_push,
    install_aur_packages, install_grub, install_network_manager, install_pacman_packages,
    install_self_bin, mk_groups, run_command, self_install_chroot, self_install_user,
    uncomment_locales, update_sudoers,
};

mod cli;
mod custom_package;
mod dotfiles;
mod utils;
mod wm;

const PARU_VERSION: &str = "v2.0.4";
const DOTFILES_MAPPING: LazyCell<HashMap<&'static str, &'static str>> = LazyCell::new(|| {
    let mut mapping = HashMap::new();
    mapping.insert(".zshrc", "~/.zshrc");
    mapping.insert(".zshenv", "~/.zshenv");
    mapping.insert("kitty", "~/.config/kitty");
    mapping.insert(".zfunc", "~/.zfunc");
    mapping.insert("hypr", "~/.config/hypr");
    mapping.insert("nvim", "~/.config/nvim");
    mapping.insert("wallpapers", "~/Pictures/wallpapers");
    mapping.insert(".gitconfig", "~/.gitconfig");
    mapping
});

const EXTRA_GROUPS: &'static [&'static str] = &["docker", "wheel"];

const AUR_PACKAGES: &'static [&'static str] = &[
    // Randoms
    "autojump-rs",
    "zen-browser-bin",
    // Fonts
    "ttf-symbola",
    "ttf-ubraille",
    "nerd-fonts",
    // Lang servers.
    "terraform-ls-bin",
    "helm-ls-bin",
];
const PACMAN_PACKAGES: &'static [&'static str] = &[
    // Top programs
    "neovim",
    "bat",
    "kitty",
    "starship",
    "docker",
    "lsd",
    "mise",
    "usage",
    "kubectl",
    "sd",
    "fd",
    "ripgrep",
    "tig",
    "thunderbird",
    // Shell shit
    "zsh",
    "zsh-autosuggestions",
    "zsh-syntax-highlighting",
    // Fonts
    "ttf-fira-code",
    "ttf-font-awesome",
    "ttf-iosevka-nerd",
    "ttf-ubuntu-font-family",
    "otf-font-awesome",
    "opendesktop-fonts",
    // Audio
    "bluez",
    "bluez-libs",
    "bluez-utils",
    "pavucontrol",
    "playerctl",
    "blueman",
    // Random stuff
    "acpi",
    "base-devel",
    "libldac",
    "noto-fonts-emoji",
    "gnome-keyring",
    "linux-headers",
    "tk",
    "os-prober",
    "wget",
    "zip",
    "unzip",
    "feh",
    "curl",
    "mpv",
    "java-runtime-common",
    "man-db",
    "kvantum",
    // Lang servers
    "rust-analyzer",
    "lua-language-server",
    "pyright",
    "typescript-language-server",
    "vue-language-server",
    "vue-typescript-plugin",
    "ruff",
    "yaml-language-server",
    "texlab",
    "gopls",
];

const CUSTOM_PACKAGES: &'static [CustomPackage] = &[
    CustomPackage::GitPackage {
        url: "https://github.com/robbyrussell/oh-my-zsh.git",
        build_command: "sh ./tools/install.sh --unattended",
        skip_if: || {
            let path = PathBuf::from(shellexpand::full("~/.oh-my-zsh")?.to_string());
            Ok(path.exists())
        },
    },
    CustomPackage::GitPackage {
        url: "https://github.com/Fausto-Korpsvart/Gruvbox-GTK-Theme.git",
        build_command: "sh themes/install.sh --tweaks black --libadwaita && cp -r ./icons ~/.local/share",
        skip_if: || {
            let path = PathBuf::from(
                shellexpand::full("~/.themes/Gruvbox-Dark")?.to_string(),
            );
            Ok(path.exists())
        },
    },
    CustomPackage::GitPackage {
        url: "https://github.com/vinceliuice/Colloid-kde.git",
        build_command: "sh install.sh",
        skip_if: || {
            let path = PathBuf::from(shellexpand::full("~/.config/Kvantum/Colloid")?.to_string());
            Ok(path.exists())
        },
    },
];
const SERVICES_TO_ENABLE: &'static [&'static str] = &["docker.service"];

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Chroot(args) => chroot_install(args)?,
        Cli::User(args) => user_install(args)?,
        Cli::Sync { commit, push } => sync_files(commit, push)?,
        Cli::Pull => pull()?,
        Cli::Apply => apply()?,
    }
    Ok(())
}

fn user_install(args: UserArgs) -> anyhow::Result<()> {
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
        url: format!("https://github.com/Morganamilo/paru/releases/download/{PARU_VERSION}/paru-{PARU_VERSION}-{arch}.tar.zst").as_str(),
        install_command: format!("tar xvf paru-{PARU_VERSION}-{arch}.tar.zst && ./paru -Syu --noconfirm paru-bin").as_str(),
    }
        .install()?;

    wm::hypr::install_hyprland()?;

    apply()?;

    self_install_user(&args.repo_url, &args.configs_path)?;

    Ok(())
}

pub fn pull() -> anyhow::Result<()> {
    let main_folder = env!("CARGO_MANIFEST_DIR");
    git_pull(main_folder)?;
    install_self_bin(main_folder)?;
    run_command(std::env!("CARGO_BIN_NAME"), ["apply"], false)?;

    Ok(())
}

pub fn apply() -> anyhow::Result<()> {
    install_pacman_packages(PACMAN_PACKAGES, true)?;
    install_aur_packages(AUR_PACKAGES)?;
    for package in CUSTOM_PACKAGES {
        package.install()?;
    }
    Dotfiles::copy(&DOTFILES_MAPPING)?;
    enable_services(SERVICES_TO_ENABLE, true)?;
    Ok(())
}

fn sync_files(commit: bool, push: bool) -> anyhow::Result<()> {
    let main_folder = env!("CARGO_MANIFEST_DIR");
    let dotfiles_folder = PathBuf::from(format!("{main_folder}/dotfiles"));
    std::fs::remove_dir_all(&dotfiles_folder).ok();
    std::fs::create_dir_all(&dotfiles_folder).ok();
    println!("Syncing dotfiles for {dotfiles_folder:?}");
    for (local_path, sys_path) in DOTFILES_MAPPING.iter() {
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
    // Remove intree from nvim init.
    let init_contents = read_to_string(dotfiles_folder.join("nvim/init.lua"))?;
    let mut nvim_init = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(dotfiles_folder.join("nvim/init.lua"))?;
    for line in init_contents.lines() {
        if line.contains("intree") {
            println!("Removing intree from nvim init");
            continue;
        }
        nvim_init.write(line.as_bytes())?;
        nvim_init.write(b"\n")?;
    }

    if commit {
        git_commit(main_folder, "Sync dotfiles")?;
    }
    if push {
        git_push(main_folder)?;
    }

    Ok(())
}

fn chroot_install(args: ChrootInstallArgs) -> anyhow::Result<()> {
    if users::get_current_username().unwrap() != "root" {
        anyhow::bail!("You must run this script as root");
    }
    let hostname = format!("{}-pc", args.username);

    let user_password = get_password("user")?;
    let root_password = get_password("root")?;

    install_pacman_packages(["sudo", "rustup", "git-lfs"], false)?;
    run_command("git", ["lfs", "install"], false)?;
    run_command("git", ["lfs", "pull"], false)?;
    install_pacman_packages(PACMAN_PACKAGES, false)?;
    // Setting currene timezone.
    println!("Setting timezone to {}", args.timezone);
    std::fs::remove_file("/etc/localtime").ok();
    std::os::unix::fs::symlink(
        format!("/usr/share/zoneinfo/{}", args.timezone),
        "/etc/localtime",
    )?;
    // Set correct time.
    run_command("hwclock", ["--systohc"], false)?;
    // Set up locales.
    uncomment_locales(args.locales.iter())?;
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
    mk_groups(EXTRA_GROUPS.iter())?;
    // Create %wheel group.
    update_sudoers()?;
    create_user(&args.username, EXTRA_GROUPS, "/bin/zsh")?;
    ch_passwd(&args.username, &user_password)?;
    install_grub(&args.efi_target, &args.efi_mountpoint, &args.bootloader_id)?;
    install_network_manager()?;
    self_install_chroot()?;

    Ok(())
}
