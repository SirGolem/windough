use crate::{
    printinfo,
    utils::{resource_exists, validate_name, ResourceType},
    verbose, PROJECT_DIRS,
};
use anyhow::{Context, Result};
use std::fs;

pub fn remove(name: String) -> Result<()> {
    validate_name(&name)?;

    let file_path = PROJECT_DIRS.data_dir().join(format!("{}.json", name));
    if !resource_exists(file_path.as_path(), ResourceType::File, false)? {
        if verbose() {
            printinfo!("file does not exist");
        }

        return Ok(());
    }

    fs::remove_file(file_path).with_context(|| "Failed to remove file")?;

    Ok(())
}
