use rand::distr::{Alphanumeric, SampleString};

use crate::utils::run_command;

pub enum CustomPackage<'a> {
    // This variant will clone the repo.
    GitPackage {
        // Git URL.
        repo: &'a str,
        // Optional git ref (branch, tag, commit).
        // If None, it will use the default branch.
        git_ref: Option<&'a str>,
        // Command to run after cloning the repo.
        // This command should be run in the repo directory.
        build_command: &'a str,
        // Function to check if we should skip the installation.
        skip_if: fn() -> anyhow::Result<bool>,
    },
    HttpFile {
        // HTTP URL to download the file from.
        url: &'a str,
        // Command to run after downloading the file.
        install_command: &'a str,
        // Function to check if we should skip the installation.
        skip_if: fn() -> anyhow::Result<bool>,
    },
}

impl<'a> CustomPackage<'a> {
    pub fn install(&self) -> anyhow::Result<()> {
        let mut rng = rand::rng();
        let name = Alphanumeric.sample_string(&mut rng, 5);
        let build_dir = format!("/tmp/build_{}", name);

        match self {
            CustomPackage::GitPackage {
                repo,
                git_ref,
                build_command,
                skip_if,
            } => {
                if skip_if()? {
                    println!("Skipping custom package installation");
                    return Ok(());
                }
                println!("Installing custom package from git URL: {}", repo);
                run_command("git", ["clone", repo, &build_dir], false)?;
                if let Some(git_ref) = git_ref {
                    run_command("git", ["checkout", git_ref], false)?;
                }
                let code = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(build_command)
                    .current_dir(&build_dir)
                    .spawn()?
                    .wait()?;
                if !code.success() {
                    anyhow::bail!("Failed to run build command");
                }
            }
            CustomPackage::HttpFile {
                url,
                install_command,
                skip_if,
            } => {
                if skip_if()? {
                    println!("Skipping custom package installation");
                    return Ok(());
                }
                println!("Installing custom package from HTTP URL: {}", url);
                std::fs::create_dir_all(&build_dir)?;
                let code = std::process::Command::new("wget")
                    .arg(url)
                    .current_dir(&build_dir)
                    .spawn()?
                    .wait()?;
                if !code.success() {
                    anyhow::bail!("Failed to download a file");
                }

                let code = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(install_command)
                    .current_dir(&build_dir)
                    .spawn()?
                    .wait()?;
                if !code.success() {
                    anyhow::bail!("Failed to run install command");
                }
            }
        }
        std::fs::remove_dir_all(&build_dir).ok();
        Ok(())
    }
}
