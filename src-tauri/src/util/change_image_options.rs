use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimplifyOptions {
    pub enabled: bool,
    pub colors: u8
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DownscaleOptions {
    pub enabled: bool,
    pub width: u32,
    pub height: u32
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeImageOptions {
    pub path: String,
    pub datapack_path: String,
    pub simplify: SimplifyOptions,
    pub downscale: DownscaleOptions,
    pub scale: f32,
    pub quality: i8
}