use anyhow::{ensure, Result};
use regex::Regex;

/// Validates that a string is a valid alphanumeric string (including underscores and hyphens) that can be used in a file name
///
/// # Arguments
///
/// * `name` - The string to be validated
///
/// # Errors
///
/// Returns an error if the string is invalid
pub fn validate_name(name: &str) -> Result<()> {
    let pattern = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    ensure!(pattern.is_match(name), "Invalid name: name can only contain alphanumeric characters, underscores (_) and hyphens (-)");
    Ok(())
}
