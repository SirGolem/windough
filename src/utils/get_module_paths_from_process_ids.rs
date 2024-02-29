use crate::{printwarning, verbose};
use anyhow::{bail, Context, Result};
use std::{ffi::OsString, mem::zeroed, os::windows::ffi::OsStringExt, ptr::null_mut};
use winapi::{
    shared::{minwindef::DWORD, ntdef::WCHAR},
    um::{
        errhandlingapi::GetLastError,
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::GetModuleFileNameExW,
        winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

/// Given a vector of process IDs, attempts to get their corresponding module paths
///
/// # Arguments
///
/// * `process_ids` - A vector of process IDs
///
/// # Errors
///
/// The function does not error, but will print a warning (in verbose mode) if a module path could not be obtained and will place `None` in the resulting vector
pub fn get_module_paths_from_process_ids(process_ids: Vec<DWORD>) -> Vec<Option<String>> {
    let mut module_paths: Vec<Option<String>> = Vec::new();

    for process_id in process_ids {
        let module_path_result = get_module_path_from_process_id(process_id)
            .with_context(|| "failed to get module path from process ID");
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

/// Given a process ID, attempts to get its corresponding module path
///
/// # Arguments
///
/// * `process_id` - A process ID
///
/// # Errors
///
/// Returns an error if any stage of the function fails, which contains the Win32 error code
pub fn get_module_path_from_process_id(process_id: DWORD) -> Result<String> {
    unsafe {
        let process_handle =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if process_handle.is_null() {
            bail!(
                "Failed to get handle for process {} (Win32 error: {})",
                process_id,
                GetLastError()
            );
        }

        let mut module_path_raw: [WCHAR; 512] = zeroed();

        if GetModuleFileNameExW(
            process_handle,
            null_mut(),
            module_path_raw.as_mut_ptr(),
            module_path_raw.len() as DWORD,
        ) == 0
        {
            CloseHandle(process_handle);
            bail!(
                "Failed to get module path (PID: {}) (Win32 error: {})",
                process_id,
                GetLastError()
            );
        }

        let module_path = OsString::from_wide(&module_path_raw)
            .to_string_lossy()
            .trim_end_matches('\0')
            .to_string();
        Ok(module_path)
    }
}
