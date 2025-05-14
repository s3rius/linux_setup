use crate::utils::run_command;

pub enum CustomPackage {
    GitPackage {
        url: &'static str,
        build_command: &'static str,
    },
}

impl CustomPackage {
    pub fn install(&self) -> anyhow::Result<()> {
        match self {
            CustomPackage::GitPackage { url, build_command } => {
                println!("Installing custom package from git URL: {}", url);
                run_command("git", ["clone", url, "/tmp/build"], false)?;
                let code = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(build_command)
                    .current_dir("/tmp/build")
                    .spawn()?
                    .wait()?;
                if !code.success() {
                    anyhow::bail!("Failed to run build command");
                }
            }
        }
        Ok(())
    }
}
