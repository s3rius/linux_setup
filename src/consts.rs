use std::{cell::LazyCell, collections::HashMap, path::PathBuf};

use crate::custom_package::CustomPackage;

// Paru version to install initially before
// installing it from AUR using paru itself.
pub const PARU_VERSION: &str = "v2.0.4";

// Dotfiles to copy in dotfiles folder.
pub const DOTFILES_MAPPING: LazyCell<HashMap<&'static str, &'static str>> = LazyCell::new(|| {
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

// Groups to add the user to when creating.
pub const EXTRA_GROUPS: &'static [&'static str] = &["docker", "wheel", "libvirt"];
// Systemd services to enable.
pub const SERVICES_TO_ENABLE: &'static [&'static str] = &["docker.service"];

/// Packages to install from AUR using AUR package manager like paru.
pub const AUR_PACKAGES: &'static [&'static str] = &[
    // Theming
    "gruvbox-gtk-theme-git",
    "gruvbox-plus-icon-theme-git",
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

// Packages to install from arch linux repossitory
// using pacman.
pub const PACMAN_PACKAGES: &'static [&'static str] = &[
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
    "virt-manager",
    "transmission-gtk",
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
    "marksman",
];

// Custom packages that are not in AUR nor pacman.
// We can install them using custom commands.
pub const CUSTOM_PACKAGES: &'static [CustomPackage] = &[
    CustomPackage::GitPackage {
        repo: "https://github.com/robbyrussell/oh-my-zsh.git",
        git_ref: None,
        build_command: "sh ./tools/install.sh --unattended",
        skip_if: || {
            let path = PathBuf::from(shellexpand::full("~/.oh-my-zsh")?.to_string());
            Ok(path.exists())
        },
    },
    CustomPackage::HttpFile {
        url: "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip",
        install_command: "unzip awscli-exe-linux-x86_64.zip && sudo ./aws/install",
        skip_if: || Ok(PathBuf::from("/usr/local/bin/aws").exists()),
    },
];
