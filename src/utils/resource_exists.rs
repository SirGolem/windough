use anyhow::{anyhow, Context, Result};
use std::{fs, io::ErrorKind, path::Path};

pub enum ResourceType {
    Dir,
    File,
}

/// Checks that a filesystem resource exists
///
/// # Arguments
///
/// * `path` - A path to the resource
/// * `resource_type` - The expected type of the resource
/// * `create` - Whether to create the resource if there is nothing already at this path
///
/// # Errors
///
/// Returns an error if metadata could not be obtained or creation of a resource fails
pub fn resource_exists(path: &Path, resourse_type: ResourceType, create: bool) -> Result<bool> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(match resourse_type {
            ResourceType::Dir => metadata.is_dir(),
            ResourceType::File => metadata.is_file(),
        }),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                if create {
                    match resourse_type {
                        ResourceType::Dir => fs::create_dir_all(path)
                            .with_context(|| "Failed to create directories")?,
                        ResourceType::File => {
                            fs::write(path, "").with_context(|| "Failed to create file")?
                        }
                    }

                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(anyhow!(error)),
        },
    }
}
