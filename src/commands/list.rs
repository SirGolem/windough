use crate::{
    data::WindowData,
    printinfo, printwarning,
    utils::{resource_exists, ResourceType},
    verbose, PROJECT_DIRS,
};
use anyhow::{bail, Context, Result};
use std::fs::{self, DirEntry};

pub fn list() -> Result<()> {
    let path = PROJECT_DIRS.data_dir();
    if !resource_exists(path, ResourceType::Dir, false)? {
        if verbose() {
            printinfo!("data directory does not exist");
        }

        return Ok(());
    }

    let mut names: Vec<String> = Vec::new();
    let mut potentially_missing_entries = false;

    for entry in fs::read_dir(path).with_context(|| "Failed to read directory")? {
        match entry.with_context(|| "failed to get item from directory") {
            Ok(item) => match evaluate_item(item) {
                Ok(data) => {
                    if let Some(name) = data {
                        names.push(name)
                    }
                }
                Err(error) => {
                    if verbose() {
                        printwarning!("{}", error);
                    }

                    potentially_missing_entries = true;
                    continue;
                }
            },
            Err(error) => {
                if verbose() {
                    printwarning!("{}", error);
                }

                potentially_missing_entries = true;
                continue;
            }
        };
    }

    println!("{}", names.join("\n"));
    if potentially_missing_entries {
        printinfo!("some items may be missing from this list - for more details, run this command in verbose mode");
    }

    Ok(())
}

fn evaluate_item(item: DirEntry) -> Result<Option<String>> {
    let metadata = item
        .metadata()
        .with_context(|| "failed to get item metadata")?;
    if !metadata.is_file() {
        return Ok(None);
    }

    let path = item.path();

    let json_string = fs::read_to_string(&path).with_context(|| "error reading data from file")?;
    let window_data: WindowData = serde_json::from_str(&json_string)
        .with_context(|| "error parsing file contents as JSON")?;

    let name_from_path_option = path.file_stem();
    let name_from_path = match name_from_path_option {
        Some(data) => data.to_str(),
        None => bail!("failed to get file stem from path"),
    };
    if Some(window_data.name.as_str()) != name_from_path {
        bail!("'name' property in file does not match expected name");
    }

    Ok(Some(window_data.name))
}
