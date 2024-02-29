use crate::{printwarning, verbose};
use anyhow::{bail, Context, Result};
use std::{ffi::OsString, mem::zeroed, os::windows::ffi::OsStringExt, ptr::null_mut};
use winapi::{
    shared::{
        minwindef::{DWORD, FALSE, MAX_PATH},
        windef::HWND,
    },
    um::{
        errhandlingapi::GetLastError,
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::GetModuleFileNameExW,
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, WCHAR},
        winuser::GetWindowThreadProcessId,
    },
};

/// Given a vector of window handles, attempts to get their corresponding module paths
///
/// # Arguments
///
/// * `hwnds` - A vector of window handles
///
/// # Errors
///
/// The function does not error, but will print a warning (in verbose mode) if a module path could not be obtained and will place `None` in the resulting vector
pub fn get_module_paths_from_windows(hwnds: &Vec<HWND>) -> Vec<Option<String>> {
    let mut module_paths: Vec<Option<String>> = Vec::new();

    for hwnd in hwnds {
        let module_path_result = get_module_path_from_window(hwnd)
            .with_context(|| "failed to get module path from window handle");
        match module_path_result {
            Ok(data) => {
                if !data.to_lowercase().starts_with("c:\\windows")
                    && !data.to_lowercase().starts_with("c:/windows")
                {
                    module_paths.push(Some(data));
                } else {
                    module_paths.push(None);
                }
            }
            Err(error) => {
                if verbose() {
                    printwarning!("{}", error);
                }
                module_paths.push(None);
            }
        }
    }

    module_paths
}

/// Given a window handle, attempts to get its corresponding module path
///
/// # Arguments
///
/// * `hwnd` - A window handle
///
/// # Errors
///
/// Returns an error if any stage of the function fails, which contains the Win32 error code
pub fn get_module_path_from_window(hwnd: &HWND) -> Result<String> {
    unsafe {
        let mut process_id: DWORD = 0;
        if GetWindowThreadProcessId(*hwnd, &mut process_id) == 0 {
            bail!(
                "Failed to get window process ID (Win32 error: {})",
                GetLastError()
            );
        }

        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            FALSE,
            process_id,
        );
        if process_handle.is_null() {
            bail!(
                "Failed to get handle for process {} (Win32 error: {})",
                process_id,
                GetLastError()
            );
        }

        let mut raw_module_path: [WCHAR; MAX_PATH] = zeroed();
        let path_length = GetModuleFileNameExW(
            process_handle,
            null_mut(),
            raw_module_path.as_mut_ptr(),
            raw_module_path.len() as DWORD,
        );
        if path_length == 0 {
            CloseHandle(process_handle);
            bail!(
                "Failed to get module path for process {} (Win32 error: {})",
                process_id,
                GetLastError()
            );
        }

        CloseHandle(process_handle);

        let module_path = OsString::from_wide(&raw_module_path)
            .to_string_lossy()
            .trim_matches(char::from(0))
            .to_string();
        Ok(module_path)
    }
}
