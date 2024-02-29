use crate::{
    printinfo,
    utils::{get_root_dir_path, resource_exists, ResourceType},
    verbose, PROJECT_DIRS,
};
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn clear(all: bool) -> Result<()> {
    if !all {
        let path = PROJECT_DIRS.data_dir();
        if !resource_exists(path, ResourceType::Dir, false)? {
            if verbose() {
                printinfo!("directory does not exist");
            }

            return Ok(());
        }

        fs::remove_dir_all(PROJECT_DIRS.data_dir())
            .with_context(|| "Failed to delete data directory")?;
    } else {
        let path = get_root_dir_path().with_context(|| "Failed to get root directory path")?;
        if !resource_exists(Path::new(&path), ResourceType::Dir, false)? {
            if verbose() {
                printinfo!("directory does not exist");
            }

            return Ok(());
        }

        fs::remove_dir_all(path).with_context(|| "Failed to delete project directory")?;
    }

    Ok(())
}
