use crate::data::{WindowPosition, WindowSize};
use anyhow::{bail, Result};
use winapi::{
    shared::windef::HWND,
    um::{
        errhandlingapi::GetLastError,
        winuser::{SetWindowPos, HWND_TOP, SWP_NOZORDER},
    },
};

/// Attempts to reposition and resize a window
///
/// # Arguments
///
/// * `hwnd` - A handle to the target window
/// * `position` - The target position data
/// * `size` - The target size data
///
/// # Errors
///
/// Returns an error if repositioning/resizing fails, which contains the Win32 error code
pub fn reposition_and_resize_window(
    hwnd: HWND,
    position: &WindowPosition,
    size: &WindowSize,
) -> Result<()> {
    unsafe {
        if SetWindowPos(
            hwnd,
            HWND_TOP,
            position.left,
            position.top,
            size.width,
            size.height,
            SWP_NOZORDER,
        ) == 0
        {
            bail!(
                "Failed to reposition/resize window (Win32 error: {})",
                GetLastError()
            );
        }
    }

    Ok(())
}
