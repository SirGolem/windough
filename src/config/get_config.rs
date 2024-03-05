use super::ConfigData;
use crate::{
    utils::{resource_exists, ResourceType},
    PROJECT_DIRS,
};
use anyhow::{Context, Result};
use std::fs;

pub fn get_config() -> Result<ConfigData> {
    let dir_path = PROJECT_DIRS.config_dir();
    resource_exists(dir_path, ResourceType::Dir, true)?;
    let file_path = dir_path.join("config.json");
    if !resource_exists(&file_path, ResourceType::File, false)? {
        fs::write(&file_path, "{}").with_context(|| "Failed to create config file")?;
    }

    let json_string =
        fs::read_to_string(file_path).with_context(|| "Error reading config from file")?;
    let config_data: ConfigData = serde_json::from_str(&json_string)
        .with_context(|| "Error parsing file contents as JSON")?;

    Ok(config_data)
}
