use crate::PROJECT_DIRS;
use anyhow::{bail, Result};

/// Gets the path to the application's root directory
///
/// # Errors
///
/// Returns an error if conversion of the path to a usable string fails
pub fn get_root_dir_path() -> Result<String> {
    let data_path_option = PROJECT_DIRS.data_dir().to_str();
    let data_path = match data_path_option {
        Some(data) => data,
        None => bail!("Conversion of path to string failed"),
    };
    let path = data_path
        .trim_end_matches("\\data")
        .trim_end_matches("/data")
        .to_string();

    Ok(path)
}
