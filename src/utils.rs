use std::{
    ffi::OsStr,
    io::Write,
    process::{Command, ExitStatus, Stdio},
};

use rand::distr::{Alphanumeric, SampleString};

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

pub fn run_as_user<T: AsRef<OsStr>>(
    user: &str,
    command: impl AsRef<OsStr>,
    args: impl IntoIterator<Item = T>,
) -> anyhow::Result<ExitStatus> {
    let home_dir = format!("/home/{user}");
    let status = Command::new("runuser")
        .env_clear()
        .envs([
            ("PATH", "/usr/local/bin:/usr/bin:/bin"),
            ("HOME", &home_dir),
            ("USER", &user),
        ])
        .current_dir(format!("/home/{}", user))
        .args(["-u", user])
        .arg("--")
        .arg(command)
        .args(args)
        .spawn()?
        .wait()?;

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
    let wheel_rule = "%wheel ALL=(ALL:ALL) NOPASSWD: ALL";

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

pub fn create_user(
    username: &str,
    groups: impl Iterator<Item = impl ToString>,
    shell: &str,
) -> anyhow::Result<()> {
    println!("Creating user {username}");
    let code = run_command(
        "useradd",
        [
            "--groups",
            groups
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .join(",")
                .as_str(),
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

pub fn install_grub(efi_mountpoint: &str, bootloader_id: &str) -> anyhow::Result<()> {
    install_pacman_packages(["grub", "efibootmgr", "os-prober"], false)?;
    let target = match std::env::consts::ARCH {
        "x86" => "i386-efi",
        "x86_64" => "x86_64-efi",
        "arm" => "arm-efi",
        "aarch64" => "arm64-efi",
        "riscv32" => "riscv32-efi",
        "riscv64" => "riscv64-efi",
        "loongarch64" => "loongarch64-efi",
        _ => anyhow::bail!("Unsupported CPU architecture for GRUB."),
    };
    let code = run_command(
        "grub-install",
        [
            "--target",
            target,
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

pub fn enable_services(
    services: impl Iterator<Item = impl ToString>,
    sudo: bool,
) -> anyhow::Result<()> {
    println!("Enabling services:");
    for service in services {
        let service_name = service.to_string();
        let code = run_command("systemctl", ["enable", &service_name], sudo)?;
        if !code.success() {
            anyhow::bail!("Failed to enable {service_name}");
        }
        println!(" * Enabled {service_name}");
    }
    Ok(())
}

pub fn install_network_manager() -> anyhow::Result<()> {
    println!("Installing network manager");
    install_pacman_packages(["networkmanager"], false)?;
    enable_services(["NetworkManager.service"].iter(), false)?;
    Ok(())
}

pub fn install_paru(username: &str) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let name = Alphanumeric.sample_string(&mut rng, 5);
    let build_folder = format!("/tmp/{name}");
    run_as_user(
        username,
        "git",
        [
            "clone",
            "https://aur.archlinux.org/paru.git",
            build_folder.as_str(),
        ],
    )?;
    let home_dir = format!("/home/{username}");
    Command::new("runuser")
        .env_clear()
        .envs([
            (
                "PATH",
                format!("{home_dir}/.cargo/bin:/usr/local/bin:/usr/bin:/bin"),
            ),
            ("HOME", home_dir),
            ("USER", username.to_string()),
        ])
        .current_dir(build_folder)
        .args(["-u", username, "--", "makepkg", "-si", "--noconfirm"])
        .spawn()?
        .wait()?;
    run_command("paru", ["-Scc", "--noconfirm"], false)?;
    Ok(())
}
