use std::path::PathBuf;

#[derive(clap::Parser)]
pub enum Cli {
    /// Command to run in chroot,
    /// right after the disc layout.
    Chroot,
    /// Command to run on first boot.
    User,
    /// Sync files from your PC with repo's dotfiles,
    /// accroding to the `DOTFILES_MAPPING`.
    Sync {
        #[arg(short, long, default_value = "false")]
        commit: bool,
        #[arg(short, long, default_value = "false")]
        push: bool,
    },
    Add {
        /// Add a file to the repo.
        /// This command will copy the file to the repo.
        /// and add it to the git index.
        #[arg()]
        file: PathBuf,
    },
    /// Pull files from the repo.
    /// and apply them to your system.
    ///
    /// This command will install pacman packages, aur packages
    /// and will copy dotfiles.
    Pull,
    /// Apply files from the repo. Used by pull, but can also be used
    /// to apply files from the repo manually.
    Apply,
}
