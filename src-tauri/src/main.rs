// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod util;

use std::path::Path;

mod preview;
mod create_datapack;

use preview::{modify_preview_image};

use regex::Regex;
use util::{image_to_base64, ImageData, ChangeImageOptions};
use create_datapack::create_datapack;

#[tauri::command]
fn change_image(options: ChangeImageOptions) -> ImageData {
    let extension_regex = Regex::new(r"(png|gif|jpe?g|jpg|jfif|bmp|tiff|ico|webp|avif|pnm|pbm|pgm|ppm|pam|dds|dxt1|dxt3|dxt5|tga|openexr|farbfeld)").unwrap();
    
    let path = Path::new(&options.path);
    if !path.is_file() || !extension_regex.is_match(&path.extension().unwrap().to_str().unwrap()) {
        return ImageData {
            width: 0,
            height: 0,
            original_width: 0,
            original_height: 0,
            data: String::new()
        };
    }

    let img = modify_preview_image(options);
    let base64 = image_to_base64(&img, img.width(), img.height());

    return base64;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![change_image, create_datapack])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
