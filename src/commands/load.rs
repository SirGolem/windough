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
use glob::{glob, Pattern};
use std::{cmp::min, fs, iter, path::PathBuf, thread::sleep, time::Duration};
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
        let matching_paths: Vec<PathBuf> = match glob(&window.application_path) {
            Ok(paths) => paths.filter_map(Result::ok).collect(),
            Err(err) => {
                if verbose() {
                    printwarning!(
                        "failed to convert 'application_path' value to pattern for matching: {}",
                        err.msg
                    );
                }
                continue;
            }
        };

        if matching_paths.is_empty() {
            if verbose() {
                printwarning!(
                    "no files found matching 'application_path' value - skipping entry\nPath: {}",
                    &window.application_path
                );
            }
            continue;
        }

        let paths_to_launch = match window.resolve_multiple_paths {
            true => matching_paths,
            false => {
                let mut paths_to_launch_vector = Vec::new();
                paths_to_launch_vector.push(
                    matching_paths[min(window.path_resolution_index, matching_paths.len() - 1)]
                        .clone(),
                );
                paths_to_launch_vector
            }
        };

        for path_to_launch in paths_to_launch {
            if window.launch
                && !running_module_paths.contains(&Some(path_to_launch.display().to_string()))
            {
                match launch_application(
                    &path_to_launch.display().to_string(),
                    &window.application_args,
                ) {
                    Ok(_) => (),
                    Err(error) => {
                        if verbose() {
                            printwarning!("{:?}", error);
                        }
                    }
                }
            }
        }
    }

    let mut retry_attempts = 0;
    // For windows included in window_data
    let mut windows_to_retry = Vec::from_iter(iter::repeat(true).take(window_data.data.len()));
    // For other windows (not in window_data)
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

            let mut window_data_index_option = None;
            for (index, data) in window_data.data.iter().enumerate() {
                let pattern = match Pattern::new(&data.application_path) {
                    Ok(pattern) => pattern,
                    Err(err) => {
                        if verbose() {
                            printwarning!("failed to convert 'application_path' value to pattern for matching: {}", err.msg);
                        }
                        continue;
                    }
                };

                if pattern.matches(&module_path) {
                    window_data_index_option = Some(index);
                    break;
                }
            }
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

            if !windows_to_retry[window_data_index] {
                continue;
            }

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
