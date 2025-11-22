pub mod capture;
pub mod combiner;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayInfo {
    pub id: u32,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebcamInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub displays: Vec<DisplayInfo>,
    pub webcam: Option<WebcamInfo>,
    pub output_path: String,
    pub fps: u32,
    pub quality: u32, // 1-100
}
