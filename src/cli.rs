#[derive(clap::Parser)]
pub enum Cli {
    /// Command to run in chroot,
    /// right after the disc layout.
    Chroot,
    /// Command to run on first boot.
    User,
}
