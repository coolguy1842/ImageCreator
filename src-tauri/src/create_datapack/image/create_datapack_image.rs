use std::{fs, path::Path, ffi::OsStr};

use crate::util::ChangeImageOptions;

use super::{create_image, format_template_image};

pub fn create_image_datapack(options: ChangeImageOptions) {
    let filename = Path::new(&options.path).file_name().and_then(OsStr::to_str).unwrap().to_lowercase().split_once(".").unwrap().0.to_owned().replace(" ", "_");

    let res = &create_image(options.clone());
    let mut out = String::new();

    for i in 0..options.quality {
        out += &format_template_image(&filename, i, res, options.scale / 2.0);
        out += "\n";
    }

    fs::write(format!("{}/ImageDisplay/data/imagedata/functions/{}/create.mcfunction", options.datapack_path, filename), out).expect("Unable to write file");
}