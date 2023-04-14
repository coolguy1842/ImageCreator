use std::{fs::File};

use exoquant::{ditherer, optimizer, Color, convert_to_indexed};
use image::{codecs::gif::GifDecoder, AnimationDecoder, imageops::resize, Rgba, ImageBuffer};

use crate::util::ChangeImageOptions;

#[inline]
fn format_rgb(rgb: &Color) -> String {
    return format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b);
}

#[inline]
fn format_rgba(rgb: &Rgba<u8>) -> String {
    return format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
}

fn create_gif_simplified(img: ImageBuffer<Rgba<u8>, Vec<u8>>, simplify_colors: usize) -> String {
    let mut out = String::from("[{\"text\":\"");

    let pixels: Vec<Color> = img.pixels().clone().map(|p| Color::new(p.0[0], p.0[1], p.0[2], p.0[3])).collect();
    let (palette, indexed_data) = convert_to_indexed(&pixels,
        img.width() as usize, 
        simplify_colors, 
        &optimizer::KMeans,
        &ditherer::FloydSteinberg::new());
    
    let mut current_color = &palette[0];

    for (i, pallete_idx) in indexed_data.iter().enumerate() {
        if i % img.width() as usize == 0 && i != 0 {    
            out += "\\\\n";
        }

        let data = &palette[*pallete_idx as usize];

        if img.get_pixel(i as u32 % img.width(), i as u32 / img.width()).0[3] == 255 {
            if data != current_color {
                out += &format!("\",\"color\":\"{}\"}},{{\"text\":\"", format_rgb(current_color));
                current_color = data;
            }

            out += "⬛";
        }
        else {
            out += "  ";
        }
    }

    return format!("{}\",\"color\":\"{}\"}}]", out, format_rgb(&current_color)); 
}

fn create_gif_regular(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let mut out = String::from("[{\"text\":\"");
    let mut current_color = img.get_pixel(0, 0);

    for (i, data) in img.pixels().enumerate() {
        if i % img.width() as usize == 0 && i != 0 {
            out += "\\\\n";
        }

        if data[3] == 255 {
            if data != current_color {
                out += &format!("\",\"color\":\"{}\"}},{{\"text\":\"", format_rgba(&current_color));
                current_color = data;
            }

            out += "⬛";
        }
        else {
            out += "  ";
        }
    }  

    return format!("{}\",\"color\":\"{}\"}}]", out, format_rgba(&current_color));
}

pub fn create_gif(options: ChangeImageOptions) -> Vec<String> {    
    let mut out_data = vec![];

    let file_in = File::open(options.path).unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");

    for frame in frames {
        let buf = frame.buffer();
        let img: ImageBuffer<Rgba<u8>, Vec<u8>>;

        let out: String;

        if options.downscale.enabled && 
            options.downscale.width > 0 && options.downscale.height > 0 && 
            options.downscale.width < buf.width() && options.downscale.height < buf.height() {
            img = resize(buf, options.downscale.width, options.downscale.height, image::imageops::FilterType::CatmullRom);
        }
        else {
            img = buf.clone();
        }

        if options.simplify.enabled {
            out = create_gif_simplified(img, options.simplify.colors as usize);
        }
        else {
            out = create_gif_regular(img);
        }
    
        out_data.push(out);
    }

    return out_data;
}
