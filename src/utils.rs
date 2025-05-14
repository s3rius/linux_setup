use std::{
    ffi::OsStr,
    io::Write,
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
};

/// Get password from user.
///
/// Pass type is only used in the prompt.
pub fn get_password(pass_ty: &'static str) -> anyhow::Result<String> {
    print!("Please enter {pass_ty} password: ");
    std::io::stdout().flush()?;
    let password = rpassword::read_password()?;
    print!("Repeat {pass_ty} password: ");
    std::io::stdout().flush()?;
    let password2 = rpassword::read_password()?;
    if password != password2 {
        println!("Passwords do not match!");
        anyhow::bail!("Passwords do not match!");
    }

    return Ok(password);
}

pub fn run_command<T: AsRef<OsStr>>(
    command: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = T>,
    sudo: bool,
) -> anyhow::Result<ExitStatus> {
    let mut cmd = if sudo {
        let mut cmd = Command::new("sudo");
        cmd.arg("--");
        cmd.arg(command);
        cmd
    } else {
        Command::new(command)
    };

    let status = cmd.args(args).spawn()?.wait()?;

    Ok(status)
}

pub fn install_pacman_packages<T: IntoIterator<Item = impl ToString>>(
    packages: T,
    sudo: bool,
) -> anyhow::Result<()> {
    println!("Installing pacman packages");
    let args = vec!["-Syu", "--noconfirm", "--needed"]
        .into_iter()
        .map(ToString::to_string)
        .chain(packages.into_iter().map(|item| item.to_string()));
    let status = run_command("pacman", args, sudo)?;
    if !status.success() {
        anyhow::bail!("Failed to install pacman packages");
    }

    Ok(())
}

pub fn install_aur_packages<T: IntoIterator<Item = impl ToString>>(
    packages: T,
) -> anyhow::Result<()> {
    let args = vec!["-Syu", "--noconfirm", "--needed"]
        .into_iter()
        .map(ToString::to_string)
        .chain(packages.into_iter().map(|item| item.to_string()));
    let status = run_command("paru", args, false)?;
    if !status.success() {
        anyhow::bail!("Failed to install AUR packages");
    }
    Ok(())
}

pub fn uncomment_locales(locales: impl Iterator<Item = impl ToString>) -> anyhow::Result<()> {
    let locale_gen = "/etc/locale.gen";
    let locale_names = locales.map(|l| l.to_string()).collect::<Vec<_>>();
    let contents = std::fs::read_to_string(locale_gen)?
        .lines()
        .map(|line| {
            // Check if the line contains any of the locale names
            // we're after.
            for locale in &locale_names {
                if line.contains(locale) {
                    return line.replace("#", "");
                }
            }
            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n");

    std::fs::write(locale_gen, contents)?;

    if let Some(locale_name) = locale_names.first() {
        std::fs::write("/etc/locale.conf", format!("LANG={locale_name}"))?;
    }
    Ok(())
}

pub fn ch_passwd(user: impl ToString, password: impl ToString) -> anyhow::Result<()> {
    let mut cmd = Command::new("chpasswd").stdin(Stdio::piped()).spawn()?;
    let mut stdin = cmd.stdin.as_ref().unwrap();
    stdin.write_all(format!("{}:{}", user.to_string(), password.to_string()).as_bytes())?;
    stdin.flush()?;
    let code = cmd.wait()?;
    if !code.success() {
        anyhow::bail!("Failed to change password");
    }
    Ok(())
}

pub fn mk_groups(groups: impl Iterator<Item = impl ToString>) -> anyhow::Result<()> {
    for group in groups {
        println!("Creating group {}", group.to_string());
        run_command("groupadd", [group.to_string()], false)?;
    }
    Ok(())
}

pub fn update_sudoers() -> anyhow::Result<()> {
    println!("Updating sudoers");
    let wheel_rule = "%wheel ALL=(ALL) ALL";

    std::fs::write(
        "/etc/sudoers",
        std::fs::read_to_string("/etc/sudoers")?
            .lines()
            .filter(|line| line != &wheel_rule)
            .chain([wheel_rule])
            .collect::<Vec<_>>()
            .join("\n"),
    )?;
    Ok(())
}

pub fn create_user(username: &str, groups: &[&str], shell: &str) -> anyhow::Result<()> {
    println!("Creating user {username}");
    let code = run_command(
        "useradd",
        [
            "--groups",
            groups.join(",").as_str(),
            "--create-home",
            "--shell",
            shell,
            "--user-group",
            username,
        ],
        false,
    )?;
    if !code.success() {
        anyhow::bail!("Failed to create user {username}");
    }
    Ok(())
}

pub fn install_grub(
    efi_target: &str,
    efi_mountpoint: &str,
    bootloader_id: &str,
) -> anyhow::Result<()> {
    install_pacman_packages(["grub", "efibootmgr", "os-prober"], false)?;
    let code = run_command(
        "grub-install",
        [
            "--target",
            efi_target,
            "--efi-directory",
            efi_mountpoint,
            "--bootloader-id",
            bootloader_id,
        ],
        false,
    )?;
    if !code.success() {
        anyhow::bail!("Failed to install grub");
    }
    let code = run_command("grub-mkconfig", ["-o", "/boot/grub/grub.cfg"], false)?;
    if !code.success() {
        anyhow::bail!("Failed to generate grub config");
    }
    Ok(())
}

pub fn enable_services(services: &[&str], sudo: bool) -> anyhow::Result<()> {
    println!("Enabling services:");
    for service in services {
        let code = run_command("systemctl", ["enable", service], sudo)?;
        if !code.success() {
            anyhow::bail!("Failed to enable {service}");
        }
        println!(" * Enabled {service}");
    }
    Ok(())
}

pub fn install_network_manager() -> anyhow::Result<()> {
    println!("Installing network manager");
    install_pacman_packages(["networkmanager"], false)?;
    enable_services(&["NetworkManager.service"], false)?;
    Ok(())
}

pub fn self_install_chroot() -> anyhow::Result<()> {
    run_command("cargo", ["build", "--release"], false)?;
    std::fs::copy(
        PathBuf::from("target/release").join(std::env!("CARGO_BIN_NAME")),
        PathBuf::from("/usr/local/sbin").join(std::env!("CARGO_BIN_NAME")),
    )?;
    Ok(())
}

pub fn self_install_user(repo_url: &str, config_path: &str) -> anyhow::Result<()> {
    let target_dir = shellexpand::full(config_path)?.to_string();
    // Clone the repo to the target directory
    run_command("git", ["clone", repo_url, &target_dir], false)?;

    // Install the binary from there.
    Command::new("cargo")
        .args(["install", "--path", "."])
        .current_dir(&target_dir)
        .spawn()?
        .wait()?;

    // Remove previous chroot binary from the system.
    std::fs::remove_file(PathBuf::from("/usr/local/sbin").join(std::env!("CARGO_BIN_NAME"))).ok();

    Ok(())
}
