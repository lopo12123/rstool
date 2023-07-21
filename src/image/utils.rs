use std::io::Cursor;
use image::{ColorType, DynamicImage, ImageOutputFormat};
use image::imageops::FilterType;

#[derive(Debug)]
pub struct ImageMeta {
    pub w: u32,
    pub h: u32,
    pub color_type: ColorType,
    pub bit_depth: u16,
}

impl ImageMeta {
    pub fn new(w: u32, h: u32, color_type: ColorType, bit_depth: u16) -> Self {
        Self { w, h, color_type, bit_depth }
    }
}

impl ToString for ImageMeta {
    fn to_string(&self) -> String {
        format!("dimension = {w}x{h}, color_type = {color_type:?}, bit_depth = {bit_depth}", w = self.w, h = self.h, color_type = self.color_type, bit_depth = self.bit_depth)
    }
}

pub struct ParsedImage {
    /// The meta data of the image
    pub meta: ImageMeta,
    /// DynamicImage is a wrapper around ImageBuffer that provides dynamic behavior, use `dyn_image.write_to(&mut buf, format)` to write the image to a buffer in the specified format
    pub dyn_image: DynamicImage,
}

pub fn parse_image(img_buffer: Vec<u8>) -> Result<ParsedImage, String> {
    match image::load_from_memory(&img_buffer) {
        Ok(img) => {
            Ok(ParsedImage {
                meta: ImageMeta::new(
                    img.width(),
                    img.height(),
                    img.color(),
                    img.color().bits_per_pixel(),
                ),
                dyn_image: img,
            })
        }
        Err(err) => Err(format!("{err}"))
    }
}

fn guess_out_format(format: &str) -> Option<ImageOutputFormat> {
    match format {
        "bmp" => Some(ImageOutputFormat::Bmp),
        "gif" => Some(ImageOutputFormat::Gif),
        "ico" => Some(ImageOutputFormat::Ico),
        "jpg" | "jpeg" => Some(ImageOutputFormat::Jpeg(100)),
        "png" => Some(ImageOutputFormat::Png),
        "tiff" => Some(ImageOutputFormat::Tiff),
        _ => None
    }
}

pub fn to_image(dyn_image: DynamicImage, wh: Option<(u32, u32)>, format: &str) -> Result<Vec<u8>, String> {
    match guess_out_format(format) {
        Some(out_format) => {
            let final_image = match wh {
                Some((w, h)) => dyn_image.resize(w, h, FilterType::Nearest),
                None => dyn_image
            };

            let mut png_buffer = vec![];
            match final_image.write_to(&mut Cursor::new(&mut png_buffer), out_format) {
                Ok(_) => Ok(png_buffer),
                Err(err) => Err(format!("{err}"))
            }
        }
        None => Err(format!("Invalid format. (Expect one of: bmp, gif, ico, jpg/jpeg, png, tiff, Got: {format})"))
    }
}