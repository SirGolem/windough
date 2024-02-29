use anyhow::{bail, Result};
use std::mem::{size_of, size_of_val, zeroed};
use winapi::{
    shared::minwindef::DWORD,
    um::{errhandlingapi::GetLastError, psapi::EnumProcesses},
};

/// Attempts to get process IDs of all running processes
///
/// # Errors
///
/// Returns an error if enumeration of processes fails, which contains the Win32 error code
pub fn get_running_process_ids() -> Result<Vec<DWORD>> {
    unsafe {
        let mut process_ids: [DWORD; 1024] = zeroed();
        let mut process_ids_bytes_needed = 0;

        if EnumProcesses(
            process_ids.as_mut_ptr(),
            size_of_val(&process_ids) as DWORD,
            &mut process_ids_bytes_needed,
        ) == 0
        {
            bail!(
                "Failed to enumerate processes (Win32 error: {})",
                GetLastError()
            );
        }

        let process_count = process_ids_bytes_needed as usize / size_of::<DWORD>();
        Ok(process_ids[0..process_count].to_vec())
    }
}
