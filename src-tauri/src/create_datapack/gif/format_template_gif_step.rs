use std::{path::Path, ffi::OsStr, fs::File};

use image::{codecs::gif::GifDecoder, AnimationDecoder};

const GIF_STEP_TEMPLATE: &str = 
"execute as @e[type=minecraft:text_display,tag=(filename)] if score @s GIF_FRAME = (filename) GIF_CURRENT_FRAME run data merge entity @s {view_range:0f}
scoreboard players add (filename) GIF_CURRENT_FRAME 1
execute if score (filename) GIF_CURRENT_FRAME matches (frames) run scoreboard players set (filename) GIF_CURRENT_FRAME 0
execute as @e[type=minecraft:text_display,tag=(filename)] if score @s GIF_FRAME = (filename) GIF_CURRENT_FRAME run data merge entity @s {view_range:1f}";

pub fn format_template_gif_step(path: &str) -> String {
    let file_full_name = Path::new(&path).file_name().and_then(OsStr::to_str).unwrap().to_lowercase();
    let file_full_name_split: Vec<&str> = file_full_name.split(".").collect();

    let filename = file_full_name_split[0].to_owned();

    
    let file_in = File::open(path).unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");
    

    let mut out = String::from(GIF_STEP_TEMPLATE);
    out = out.replace("(filename)", &filename);
    out = out.replace("(frames)", &frames.len().to_string());

    return out;
}