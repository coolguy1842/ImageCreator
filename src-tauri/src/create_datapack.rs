mod gif;
mod format_value;
mod image;

use std::{fs};

use crate::util::ChangeImageOptions;

use self::{image::create_image_datapack, format_value::format_value, gif::create_gif_datapack};

const PACK_FORMAT: i8 = 12;

#[tauri::command]
pub fn create_datapack(options: ChangeImageOptions) -> bool {
    let path_lower = options.path.to_lowercase();
    let path_split: Vec<&str> = path_lower.split(".").collect();

    let filename = path_split[0].to_owned().replace(" ", "_");
    let extension = path_split[1].to_owned();

    fs::create_dir_all(format!("{}/ImageDisplay/data/imagedata/functions/{}", options.datapack_path, filename)).expect("Failed to create directories");

    let pack_meta = serde_json::json!({
        "pack": {
            "pack_format": PACK_FORMAT,
            "description": "Creates Images/GIFs"
        }
    });


    fs::write(format!("{}/ImageDisplay/pack.mcmeta", options.datapack_path), format_value(pack_meta)).expect("Unable to write pack.mcmeta");
    
    if extension != "gif" {
        create_image_datapack(options);
    }
    else {
        create_gif_datapack(options);
    }

    return true;
}