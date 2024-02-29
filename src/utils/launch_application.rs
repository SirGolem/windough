use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Runs a command to launch an application with arguments
///
/// # Arguments
///
/// * `application_path` - A path to the application's executable
/// * `application_args` - A vector of arguments to be passed to the application
///
/// # Errors
///
/// Returns an error if the command process could not be spawned successfully
pub fn launch_application(application_path: &String, application_args: &Vec<String>) -> Result<()> {
    let mut launch_command = Command::new(application_path);
    launch_command.args(application_args);
    launch_command.stderr(Stdio::null());
    launch_command.stdin(Stdio::null());
    launch_command.stdout(Stdio::null());

    launch_command
        .spawn()
        .with_context(|| "Failed to launch application")?;

    Ok(())
}
