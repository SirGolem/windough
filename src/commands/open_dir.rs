use crate::{
    utils::{get_root_dir_path, resource_exists, ResourceType},
    PROJECT_DIRS,
};
use anyhow::{bail, Context, Result};
use std::{
    ffi::OsString,
    os::windows::ffi::OsStrExt,
    path::Path,
    ptr::{null, null_mut},
};
use winapi::{
    shared::basetsd::INT_PTR,
    um::{errhandlingapi::GetLastError, shellapi::ShellExecuteW, winuser::SW_NORMAL},
};

pub fn open_dir(root: bool, data: bool, config: bool) -> Result<()> {
    let path: String;

    if root {
        path = get_root_dir_path().with_context(|| "Failed to get root directory path")?;
    } else if data {
        let data_path_option = PROJECT_DIRS.data_dir().to_str();
        path = match data_path_option {
            Some(data) => data.to_string(),
            None => bail!("Conversion of path to string failed"),
        };
    } else if config {
        let config_path_option = PROJECT_DIRS.config_dir().to_str();
        path = match config_path_option {
            Some(data) => data.to_string(),
            None => bail!("Conversion of path to string failed"),
        };
    } else {
        path = get_root_dir_path().with_context(|| "Failed to get root directory path")?;
    }

    resource_exists(Path::new(&path), ResourceType::Dir, true)?;

    let mut path_osstring = OsString::from(path);
    path_osstring.push(OsString::from("\0"));
    let path_wide: Vec<u16> = path_osstring.encode_wide().collect();

    unsafe {
        let return_code = ShellExecuteW(
            null_mut(),
            null(),
            path_wide.as_ptr(),
            null(),
            null(),
            SW_NORMAL,
        ) as INT_PTR;

        if return_code <= 32 {
            bail!(
                "Failed to open directory (code: {}) (Win32 error: {})",
                return_code,
                GetLastError()
            );
        }
    }

    Ok(())
}
