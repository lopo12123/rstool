use std::io::Cursor;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageOutputFormat};
use image::imageops::FilterType;
use crate::image::utils::{ImageMeta, ParsedImage};

/// Parse the raw data into the meta data and the image data
pub fn parse_png(png_buffer: Vec<u8>) -> Result<ParsedImage, String> {
    match image::load_from_memory(&png_buffer) {
        Ok(png) => {
            Ok(ParsedImage {
                meta: ImageMeta::new(
                    png.width(),
                    png.height(),
                    "png".to_string(),
                    png.color(),
                    png.color().bits_per_pixel(),
                ),
                dyn_image: png,
            })
        }
        Err(err) => Err(format!("{err}"))
    }
}

/// Write the dynamic image to a buffer in the format of 'png'
pub fn to_png(dyn_image: DynamicImage, wh: Option<(u32, u32)>) -> Result<Vec<u8>, String> {
    let final_image = match wh {
        Some((w, h)) => dyn_image.resize(w, h, FilterType::Nearest),
        None => dyn_image
    };

    let mut png_buffer = vec![];
    match final_image.write_to(&mut Cursor::new(&mut png_buffer), ImageOutputFormat::Png) {
        Ok(_) => Ok(png_buffer),
        Err(err) => Err(format!("{err}"))
    }
}

#[cfg(test)]
mod unit_tests {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use super::*;

    #[test]
    fn test_parse_png() {
        let mut png_buffer = vec![];
        let mut png_path = File::open("D:\\rstool\\examples\\image\\img.png").unwrap();
        png_path.read_to_end(&mut png_buffer).unwrap();

        let png_meta = parse_png(png_buffer);
        // println!("png meta: {:?}", png_meta.unwrap().meta.to_string());

        std::fs::write("D:\\rstool\\examples\\image\\img2.png", to_png(png_meta.unwrap().dyn_image, None).unwrap()).unwrap();
    }
}