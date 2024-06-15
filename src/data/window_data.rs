use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WindowData {
    pub name: String,
    pub data: Vec<WindowDataEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct WindowDataEntry {
    pub application_path: String,
    pub application_args: Vec<String>,
    #[serde(default = "resolve_multiple_paths_default")]
    pub resolve_multiple_paths: bool,
    #[serde(default = "path_resolution_index_default")]
    pub path_resolution_index: usize,
    #[serde(default = "launch_default")]
    pub launch: bool,
    #[serde(default = "reposition_default")]
    pub reposition: bool,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub minimized: bool,
    pub maximised: bool,
}

pub const fn resolve_multiple_paths_default() -> bool {
    false
}
pub const fn path_resolution_index_default() -> usize {
    0
}
pub const fn launch_default() -> bool {
    true
}
pub const fn reposition_default() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
pub struct WindowPosition {
    pub top: i32,
    pub left: i32,
}

#[derive(Serialize, Deserialize)]
pub struct WindowSize {
    pub width: i32,
    pub height: i32,
}
