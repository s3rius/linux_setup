use crate::utils::{enable_services, install_aur_packages, install_pacman_packages};

const HYPRLAND_PACKAGES: &'static [&'static str] = &[
    "hyprland",
    "hyprpaper",
    "brightnessctl",
    "waybar",
    "swaync",
    "wofi",
    "sddm",
    "otf-font-awesome",
    "qt6",
    "qt6ct",
    "xdg-desktop-portal-hyprland",
    "pipewire",
    "pipewire-v4l2",
    "pipewire-pulse",
    "wireplumber",
    "grim",
    "slurp",
    "wl-clipboard",
    "gtk-engine-murrine",
    "network-manager-applet",
    "noto-fonts",
    "noto-fonts-cjk",
    "noto-fonts-emoji",
    "noto-fonts-extra",
    "hyprlock",
    "udiskie",
    "cpio",
    "uwsm",
];

const PARU_PACKAGES: &'static [&'static str] = &["qt6gtk2"];

pub fn install_hyprland() -> anyhow::Result<()> {
    install_pacman_packages(HYPRLAND_PACKAGES, true)?;
    install_aur_packages(PARU_PACKAGES)?;
    enable_services(&["sddm.service"], true)?;
    Ok(())
}
