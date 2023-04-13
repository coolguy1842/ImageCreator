use image::{DynamicImage, imageops::FilterType, GenericImageView, ImageBuffer, Rgba};
use imageproc::{utils::load_image_or_panic};

use exoquant::{optimizer, ditherer, convert_to_indexed, Color};

use crate::util::ChangeImageOptions;

pub fn modify_preview_image(options: ChangeImageOptions) -> DynamicImage {
    let mut img = load_image_or_panic(options.path);

    if options.downscale.enabled && 
        options.downscale.width > 0 && options.downscale.height > 0 && 
        options.downscale.width < img.width() && options.downscale.height < img.height() {
        img = img.resize(options.downscale.width, options.downscale.height, FilterType::CatmullRom);
    }

    if options.simplify.enabled {
        let pixels: Vec<Color> = img.pixels().clone().map(|p| Color::new(p.2[0], p.2[1], p.2[2], p.2[3])).collect();
        let (palette, indexed_data) = convert_to_indexed(&pixels,
            img.width() as usize, 
            options.simplify.colors as usize, 
            &optimizer::KMeans, 
            &ditherer::FloydSteinberg::new());

        let buf = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
            let i = y * img.width() + x;
            let p = palette[indexed_data[i as usize] as usize];

            Rgba([p.r, p.g, p.b, p.a])
        });
        
        img = buf.into();
    }

    return img;
}