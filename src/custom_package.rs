use rand::distr::{Alphanumeric, SampleString};

use crate::utils::run_command;

pub enum CustomPackage<'a> {
    GitPackage {
        url: &'a str,
        build_command: &'a str,
    },
    HttpFile {
        url: &'a str,
        install_command: &'a str,
    },
}

impl<'a> CustomPackage<'a> {
    pub fn install(&self) -> anyhow::Result<()> {
        let mut rng = rand::rng();
        let name = Alphanumeric.sample_string(&mut rng, 5);
        let build_dir = format!("/tmp/build_{}", name);

        match self {
            CustomPackage::GitPackage { url, build_command } => {
                println!("Installing custom package from git URL: {}", url);
                run_command("git", ["clone", url, &build_dir], false)?;
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
