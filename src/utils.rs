use std::{
    ffi::OsStr,
    io::Write,
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
    become_pass: Option<&str>,
) -> anyhow::Result<ExitStatus> {
    let mut cmd = match &become_pass {
        Some(_) => {
            let mut cmd = Command::new("sudo");
            cmd.arg("--stdin");
            cmd.arg("--");
            cmd.arg(command);
            cmd.stdin(Stdio::piped());
            cmd
        }
        None => Command::new(command),
    };

    cmd.args(args);
    let mut child = cmd.spawn()?;
    if let Some(pass) = become_pass {
        println!("Running command as root");
        let mut stdin = child.stdin.as_ref().unwrap();
        stdin.write_all(pass.as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
    }
    let status = child.wait()?;
    Ok(status)
}

pub fn install_pacman_packages<T: IntoIterator<Item = impl ToString>>(
    packages: T,
    become_pass: Option<&str>,
) -> anyhow::Result<()> {
    println!("Installing pacman packages");
    let args = vec!["-Syu", "--noconfirm"]
        .into_iter()
        .map(ToString::to_string)
        .chain(packages.into_iter().map(|item| item.to_string()));
    let status = run_command("pacman", args, become_pass)?;
    if !status.success() {
        anyhow::bail!("Failed to install pacman packages");
    }

    Ok(())
}

pub fn install_aur_packages<T: IntoIterator<Item = impl ToString>>(packages: T) {
    let args = vec!["-Syu", "--noconfirm"]
        .into_iter()
        .map(ToString::to_string)
        .chain(packages.into_iter().map(|item| item.to_string()));
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
        run_command("groupadd", [group.to_string()], None)?;
    }
    Ok(())
}

pub fn update_sudoers() -> anyhow::Result<()> {
    println!("Updating sudoers");
    std::fs::write(
        "/etc/sudoers",
        std::fs::read_to_string("/etc/sudoers")?
            .lines()
            .map(|line| {
                if line.starts_with("%wheel") {
                    "%wheel ALL=(ALL) ALL"
                } else {
                    line
                }
            })
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
        ],
        None,
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
    install_pacman_packages(["grub", "efibootmgr", "os-prober"], None)?;
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
        None,
    )?;
    if !code.success() {
        anyhow::bail!("Failed to install grub");
    }
    let code = run_command("grub-mkconfig", ["-o", "/boot/grub/grub.cfg"], None)?;
    if !code.success() {
        anyhow::bail!("Failed to generate grub config");
    }
    Ok(())
}

pub fn enable_services(services: &[&str]) -> anyhow::Result<()> {
    println!("Enabling services:");
    for service in services {
        let code = run_command("systemctl", ["enable", service], None)?;
        if !code.success() {
            anyhow::bail!("Failed to enable {service}");
        }
        println!(" * Enabled {service}");
    }
    Ok(())
}

pub fn install_network_manager() -> anyhow::Result<()> {
    println!("Installing network manager");
    install_pacman_packages(["networkmanager"], None)?;
    enable_services(&["NetworkManager.service"])?;
    Ok(())
}
