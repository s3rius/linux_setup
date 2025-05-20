use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, ser::SerializeMap};

use crate::{
    custom_package::CustomPackage,
    utils::{expand_path, path_shrink},
};

fn default_repo_url() -> String {
    std::env!("CARGO_PKG_REPOSITORY").to_string()
}

pub fn from_path<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let path = String::deserialize(deserializer)?;
    let expanded = expand_path(&path).map_err(|err| {
        serde::de::Error::custom(format!("Cannot expand given path ({path}): {err}"))
    })?;
    Ok(expanded.to_string())
}

pub fn to_shrinked_path<S>(val: &String, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let path = path_shrink(&PathBuf::from(val)).map_err(|err| {
        serde::ser::Error::custom(format!("Cannot shrink given path ({val}): {err}"))
    })?;
    serializer.serialize_str(path.display().to_string().as_str())
}

pub fn from_path_hashmap<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let mut paths = HashMap::<String, String>::deserialize(deserializer)?;
    for (_, value) in paths.iter_mut() {
        let expanded = expand_path(&value).map_err(|err| {
            serde::de::Error::custom(format!("Cannot expand given path ({value}): {err}"))
        })?;
        *value = expanded.to_string();
    }
    Ok(paths)
}

pub fn to_shrinked_path_hashmap<S>(
    val: &HashMap<String, String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    let mut mapper = serializer.serialize_map(Some(val.len()))?;
    for (key, value) in val.iter() {
        let path = path_shrink(&PathBuf::from(value)).map_err(|err| {
            serde::ser::Error::custom(format!("Cannot shrink given path ({value}): {err}"))
        })?;
        mapper.serialize_entry(key, path.display().to_string().as_str())?;
    }
    mapper.end()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Packages {
    pub paru_version: String,
    pub pacman: Vec<String>,
    pub aur: Vec<String>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Linux {
    // Username to create.
    pub username: String,
    // Groups to add the user to.
    pub groups: Vec<String>,
    // Services to enable.
    pub services: Vec<String>,
    // Where to install the bootloader.
    // Should be mounted to /boot/EFI.
    pub efi_mountpoint: String,
    // Name of the bootloader.
    pub bootloader_id: String,
    // Timezone to set.
    pub timezone: String,
    // Language to set.
    pub locales: Vec<String>,
    #[serde(default = "default_repo_url")]
    pub repo_url: String,
    #[serde(deserialize_with = "from_path", serialize_with = "to_shrinked_path")]
    pub configs_path: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub linux: Linux,
    #[serde(
        deserialize_with = "from_path_hashmap",
        serialize_with = "to_shrinked_path_hashmap"
    )]
    pub dotfiles: HashMap<String, String>,
    pub packages: Packages,
    pub custom_packages: Vec<CustomPackage>,
}

impl Config {
    pub fn dump(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let mut document = toml_edit::ser::to_document(self)?;
        document.set_implicit(true);
        for (_, value) in document.iter_mut() {
            // We iterate over inline tables to uninline them
            // and sort their values.
            if let Some(inline) = value.as_inline_table_mut() {
                // Convert inline table to a normal table
                let mut table = inline.clone().into_table();
                table.sort_values();
                table.set_dotted(false);
                table.fmt();
                // Sort values inside the table's keys.
                for (_, table_val) in table.iter_mut() {
                    if let Some(arr) = table_val.as_array_mut() {
                        // Also, apply some formatting to it.
                        arr.set_trailing_comma(true);
                        arr.sort_by(|a, b| a.to_string().cmp(&b.to_string()));
                        for item in arr.iter_mut() {
                            item.decor_mut().set_prefix("\n");
                        }
                        arr.set_trailing("\n");
                    };
                }
                *value = toml_edit::Item::Table(table);
            }
            // Convert top-level inlined arrays
            // to an array of tables.
            //
            // It's an implicit array where each table is
            // defined under its own header.
            if let Some(array) = value.as_array_mut() {
                let mut table_array = toml_edit::ArrayOfTables::new();
                for item in array.iter() {
                    if let Some(inline) = item.as_inline_table() {
                        table_array.push(inline.clone().into_table())
                    }
                }
                *value = toml_edit::Item::ArrayOfTables(table_array);
            }
        }
        document.set_implicit(true);

        std::fs::write(path, document.to_string())?;
        Ok(())
    }
}
