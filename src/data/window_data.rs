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
    pub launch: bool,
    pub reposition: bool,
    pub position: WindowPosition,
    pub size: WindowSize,
    pub minimized: bool,
    pub maximised: bool,
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
