use std::{fs, path::Path, ffi::OsStr};

use crate::util::ChangeImageOptions;

use super::{create_gif, format_template_gif_frame, format_template_gif_step};

pub fn create_gif_datapack(options: ChangeImageOptions) {
    let filename = Path::new(&options.path).file_name().and_then(OsStr::to_str).unwrap().to_lowercase().split_once(".").unwrap().0.to_owned();

    fs::create_dir_all(format!("{}/ImageDisplay/data/imagedata/functions/{}_frames", options.datapack_path, filename)).expect("Failed to create directories");

    let path = options.path.clone();

    let res = create_gif(options.clone());
    let mut out = String::from("say ---- CREATING GIF ----\n");
    out += "scoreboard objectives add GIF_CURRENT_FRAME dummy\n";
    out += "scoreboard objectives add GIF_FRAME dummy\n";
    out += &format!("scoreboard players set {} GIF_CURRENT_FRAME 0\n\n", filename);

    for (i, data) in res.iter().enumerate() {
        let mut function_out = String::new();
        for j in 0..options.quality {
            let command_data = format_template_gif_frame(&filename, j, i, data, options.scale / 2.0);
            
            function_out += &command_data;
            function_out += "\n";
        }
        
        
        function_out += &format!("execute as @e[type=text_display,tag={}] if entity @s[tag=frame{}] run scoreboard players set @s GIF_FRAME {}\n", filename, i, i);

        fs::write(format!("{}/ImageDisplay/data/imagedata/functions/{}_frames/frame{}.mcfunction", options.datapack_path, filename, i), function_out).expect("Unable to write file");
        out += &format!("function imagedata:{}_frames/frame{}\n", filename, i);
    }

    out += &format!("\nexecute as @e[type=text_display,tag={},scores={{GIF_FRAME={}}}] run data merge entity @s {{view_range:1f}}\n", filename, 0);
    out += "say ---- CREATED GIF ----\n";

    fs::write(format!("{}/ImageDisplay/data/imagedata/functions/{}/create.mcfunction", options.datapack_path, filename), out).expect("Unable to write file");
    fs::write(format!("{}/ImageDisplay/data/imagedata/functions/{}/step.mcfunction", options.datapack_path, filename), format_template_gif_step(&path)).expect("Unable to write file");
}