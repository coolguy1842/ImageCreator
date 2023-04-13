use std::io::Cursor;

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;

use image::{ImageOutputFormat, DynamicImage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub original_width: u32,
    pub original_height: u32,
    pub data: String
}

pub fn image_to_base64(img: &DynamicImage, original_width: u32, original_height: u32) -> ImageData {
    let mut image_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png).unwrap();

    let res_base64 = BASE64.encode(image_data);
    let data = format!("data:image/png;base64,{}", res_base64);

    ImageData {
        width: img.width(),
        height: img.height(),
        original_width,
        original_height,
        data
    }
}