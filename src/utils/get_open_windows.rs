use anyhow::{bail, Result};
use std::mem::zeroed;
use winapi::{
    shared::{
        minwindef::{BOOL, LPARAM, TRUE},
        ntdef::WCHAR,
        windef::HWND,
    },
    um::{
        errhandlingapi::GetLastError,
        winuser::{EnumWindows, GetWindowTextW, IsWindowVisible},
    },
};

/// Attempts to get handles to all currently open windows (that meet the set criteria)
///
/// # Errors
///
/// Returns an error if enumeration of windows fails, which contains the Win32 error code
pub fn get_open_windows() -> Result<Vec<HWND>> {
    let mut hwnds: Vec<HWND> = Vec::new();

    unsafe {
        if EnumWindows(
            Some(enum_windows_callback),
            &mut hwnds as *mut Vec<HWND> as LPARAM,
        ) == 0
        {
            bail!(
                "Failed to enumerate windows (Win32 error: {})",
                GetLastError()
            );
        }
    }

    Ok(hwnds)
}

/// Checks a window handle against set criteria, and then adds it to the vector of window handles
///
/// This function is only intended to be used within the `EnumWindows` function
///
/// # Arguments
///
/// * `hwnd` - A window handle
/// * `raw_hwnds` - A raw representation of the vector of window handles
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, raw_hwnds: LPARAM) -> BOOL {
    let mut hwnds = Box::from_raw(raw_hwnds as *mut Vec<HWND>);

    let mut title: [WCHAR; 2] = zeroed();

    if IsWindowVisible(hwnd) != 0
        && GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32) != 0
    {
        hwnds.push(hwnd);
    }

    Box::into_raw(hwnds);

    TRUE
}
