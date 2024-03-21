use crate::{
    data::WindowData,
    printwarning,
    utils::{
        get_module_path_from_window, get_module_paths_from_windows, get_open_windows,
        launch_application, reposition_and_resize_window, resource_exists, validate_name,
        ResourceType,
    },
    verbose, CONFIG, PROJECT_DIRS,
};
use anyhow::{ensure, Context, Result};
use std::{fs, iter, thread::sleep, time::Duration};
use winapi::{
    shared::windef::HWND,
    um::winuser::{
        IsZoomed, PostMessageW, ShowWindow, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, WM_CLOSE,
    },
};

pub fn load(name: String, close_others: bool, minimize_others: bool) -> Result<()> {
    validate_name(&name)?;

    let file_path = PROJECT_DIRS.data_dir().join(format!("{}.json", name));
    ensure!(
        resource_exists(&file_path, ResourceType::File, false)?,
        "File does not exist"
    );
    let json_string =
        fs::read_to_string(file_path).with_context(|| "Error reading data from file")?;

    let window_data: WindowData = serde_json::from_str(&json_string)
        .with_context(|| "Error parsing file contents as JSON")?;

    ensure!(
        window_data.name == name,
        "'name' property in file does not match expected name"
    );

    let initial_open_windows = get_open_windows()?;
    let running_module_paths = get_module_paths_from_windows(&initial_open_windows);

    // Launch Applications
    for window in &window_data.data {
        if window.launch && !running_module_paths.contains(&Some(window.application_path.clone())) {
            match launch_application(&window.application_path, &window.application_args) {
                Ok(_) => (),
                Err(error) => {
                    if verbose() {
                        printwarning!("{:?}", error);
                    }
                }
            }
        }
    }

    let mut retry_attempts = 0;
    let mut windows_to_retry = Vec::from_iter(iter::repeat(true).take(window_data.data.len()));
    let mut windows_to_ignore: Vec<HWND> = Vec::new();

    while retry_attempts < CONFIG.retry_count && windows_to_retry.contains(&true) {
        sleep(Duration::from_millis(CONFIG.retry_interval as u64));

        let open_windows = get_open_windows()?;

        // Reposition & Resize Windows
        for hwnd in open_windows {
            if windows_to_ignore.contains(&hwnd) {
                continue;
            }

            let module_path = get_module_path_from_window(&hwnd)
                .with_context(|| "Failed to get module path from window handle")?;
            if module_path.to_lowercase().starts_with("c:\\windows")
                || module_path.to_lowercase().starts_with("c:/windows")
            {
                continue;
            }

            let window_data_index_option = window_data
                .data
                .iter()
                .position(|data| module_path == data.application_path);
            let window_data_index = match window_data_index_option {
                Some(data) => data,
                None => {
                    unsafe {
                        if close_others {
                            PostMessageW(hwnd, WM_CLOSE, 0, 0);
                        } else if minimize_others {
                            ShowWindow(hwnd, SW_MINIMIZE);
                        }
                    }

                    windows_to_ignore.push(hwnd);
                    continue;
                }
            };

            let window = &window_data.data[window_data_index];
            windows_to_retry[window_data_index] = false;

            if !window.reposition {
                continue;
            }

            unsafe {
                // Restore window to visible position
                ShowWindow(hwnd, SW_RESTORE);
                // Un-maximize window so it can be moved properly
                if IsZoomed(hwnd) != 0 {
                    ShowWindow(hwnd, SW_RESTORE);
                }
            }

            reposition_and_resize_window(hwnd, &window.position, &window.size)?;

            unsafe {
                if window.maximised {
                    ShowWindow(hwnd, SW_MAXIMIZE);
                }
                if window.minimized {
                    ShowWindow(hwnd, SW_MINIMIZE);
                }
            }
        }

        retry_attempts += 1;
    }

    Ok(())
}
