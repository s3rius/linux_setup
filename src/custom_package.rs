use std::path::PathBuf;

use rand::distr::{Alphanumeric, SampleString};

use crate::utils::run_command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum CustomPackage {
    // This variant will clone the repo.
    Git {
        // Git URL.
        repo: String,
        // Optional git ref (branch, tag, commit).
        // If None, it will use the default branch.
        git_ref: Option<String>,
        // Command to run after cloning the repo.
        // This command should be run in the repo directory.
        build_command: String,
        // Function to check if we should skip the installation.
        skip_if_exists: Option<String>,
    },
    HttpFile {
        // HTTP URL to download the file from.
        url: String,
        // Command to run after downloading the file.
        install_command: String,
        // Function to check if we should skip the installation.
        skip_if_exists: Option<String>,
    },
}

impl CustomPackage {
    fn skip_if_exists(&self) -> anyhow::Result<bool> {
        let if_exists = match self {
            CustomPackage::Git { skip_if_exists, .. } => skip_if_exists,
            CustomPackage::HttpFile { skip_if_exists, .. } => skip_if_exists,
        };
        Ok(if_exists
            .as_ref()
            .map(|a| shellexpand::full(a))
            .transpose()?
            .map(|path| PathBuf::from(path.to_string()).exists())
            .unwrap_or(false))
    }

    pub fn install(&self) -> anyhow::Result<()> {
        let mut rng = rand::rng();
        let name = Alphanumeric.sample_string(&mut rng, 5);
        let build_dir = format!("/tmp/build_{}", name);

        if self.skip_if_exists()? {
            println!("Skipping custom package installation");
            return Ok(());
        }

        match self {
            CustomPackage::Git {
                repo,
                git_ref,
                build_command,
                skip_if_exists: _,
            } => {
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
                skip_if_exists: _,
            } => {
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
