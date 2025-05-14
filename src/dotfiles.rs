use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    path::PathBuf,
};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "dotfiles"]
pub struct Dotfiles;

impl Dotfiles {
    pub fn copy(file_mapping: &HashMap<&str, &str>) -> anyhow::Result<()> {
        for (file, target_path) in file_mapping {
            for embeded_file in Self::iter() {
                if embeded_file.starts_with(file) {
                    // This thing removes duplication of a file name at the end of the
                    // target path.
                    // Required for later join.
                    let target_file_path = target_path.strip_suffix(file).unwrap_or(&target_path);
                    let target = PathBuf::from(shellexpand::full(target_file_path)?.to_string())
                        .join(embeded_file.to_string());

                    println!("Copying {} to {}", embeded_file, target.display());

                    if let Some(parent) = target.parent() {
                        std::fs::create_dir_all(parent).ok();
                        if !parent.exists() {
                            anyhow::bail!("Failed to create directory: {}", parent.display());
                        }
                    }
                    let embed_file_data = Self::get(&embeded_file).unwrap();
                    let target_file = std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(&target)?;
                    let mut writer = BufWriter::new(target_file);
                    writer.write_all(&embed_file_data.data)?;
                }
            }
        }
        Ok(())
    }
}
