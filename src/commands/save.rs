use crate::{
    data::{WindowData, WindowDataEntry, WindowPosition, WindowSize},
    utils::{
        get_module_paths_from_windows, get_open_windows, resource_exists, validate_name,
        ResourceType,
    },
    PROJECT_DIRS,
};
use anyhow::{bail, Context, Result};
use std::fs;
use winapi::{
    shared::windef::{HWND, RECT},
    um::{
        errhandlingapi::GetLastError,
        winuser::{GetWindowRect, IsIconic, IsZoomed, ShowWindow, SW_MINIMIZE, SW_RESTORE},
    },
};

pub fn save(name: String) -> Result<()> {
    validate_name(&name)?;

    let open_windows = get_open_windows()?;
    let module_paths = get_module_paths_from_windows(&open_windows);

    let mut window_data = Vec::new();

    for i in 0..(open_windows.len()) {
        if open_windows[i].is_null() {
            continue;
        }

        let module_path = match module_paths[i].clone() {
            Some(data) => data,
            None => continue,
        };

        let is_minimized = unsafe { IsIconic(open_windows[i]) == 1 };
        restore_window(open_windows[i], is_minimized); // For properly checking details when not minimized
        let is_maximized = unsafe { IsZoomed(open_windows[i]) == 1 };

        let mut window_rect = RECT {
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        unsafe {
            if GetWindowRect(open_windows[i], &mut window_rect) == 0 {
                minimize_window(open_windows[i], is_minimized); // Restore window to its previous (minimized) state

                bail!(
                    "Failed to get window rect (Win32 error: {})",
                    GetLastError()
                );
            }
        }

        minimize_window(open_windows[i], is_minimized); // Restore window to its previous (minimized) state

        let window_width = window_rect.right - window_rect.left;
        let window_height = window_rect.bottom - window_rect.top;

        window_data.push(WindowDataEntry {
            application_path: module_path,
            application_args: Vec::new(),
            position: WindowPosition {
                top: window_rect.top,
                left: window_rect.left,
            },
            size: WindowSize {
                width: window_width,
                height: window_height,
            },
            minimized: is_minimized,
            maximised: is_maximized,
        });
    }

    let data = WindowData {
        name: name.clone(),
        data: window_data,
    };

    let json_string =
        serde_json::to_string(&data).with_context(|| "Error formatting data as JSON string")?;

    resource_exists(PROJECT_DIRS.data_dir(), ResourceType::Dir, true)?;
    let file_path = PROJECT_DIRS.data_dir().join(format!("{}.json", name));

    fs::write(file_path, json_string).with_context(|| "Error writing data to file")?;

    Ok(())
}

fn restore_window(hwnd: HWND, is_minimized: bool) {
    if is_minimized {
        unsafe { ShowWindow(hwnd, SW_RESTORE) };
    }
}
fn minimize_window(hwnd: HWND, is_minimized: bool) {
    if is_minimized {
        unsafe { ShowWindow(hwnd, SW_MINIMIZE) };
    }
}
