use exoquant::{optimizer, ditherer, convert_to_indexed, Color};
use image::{GenericImageView, Rgba, imageops::FilterType, DynamicImage};
use imageproc::utils::load_image_or_panic;

use crate::util::ChangeImageOptions;

#[inline]
fn format_rgb(rgb: &Color) -> String {
    return format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b);
}

#[inline]
fn format_rgba(rgb: &Rgba<u8>) -> String {
    return format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
}

fn create_image_simplified(img: DynamicImage, simplify_colors: usize) -> String {
    let mut out = String::from("[{\"text\":\"");
    let mut current_color: &Color;

    let pixels: Vec<Color> = img.pixels().clone().map(|p| Color::new(p.2[0], p.2[1], p.2[2], p.2[3])).collect();
    let (palette, indexed_data) = convert_to_indexed(&pixels,
        img.width() as usize, 
        simplify_colors, 
        &optimizer::KMeans, 
        &ditherer::FloydSteinberg::new());

    current_color = &palette[0];

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

fn create_image_regular(img: DynamicImage) -> String {
    let mut out = String::from("[{\"text\":\"");
    let mut current_color = img.get_pixel(0, 0);

    for (x, y, data) in img.pixels() {
        if x == 0 && y != 0 {      
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

pub fn create_image(options: ChangeImageOptions) -> String {
    let mut img = load_image_or_panic(options.path);

    if options.downscale.enabled && 
        options.downscale.width > 0 && options.downscale.height > 0 && 
        options.downscale.width < img.width() && options.downscale.height < img.height() {
        img = img.resize(options.downscale.width, options.downscale.height, FilterType::CatmullRom);
    }

    if options.simplify.enabled {
        return create_image_simplified(img, options.simplify.colors as usize);
    }

    create_image_regular(img)
}
