use image::{ColorType, DynamicImage};

#[derive(Debug)]
pub struct ImageMeta {
    pub w: u32,
    pub h: u32,
    pub format: String,
    pub color_type: ColorType,
    pub bit_depth: u16,
}

impl ImageMeta {
    pub fn new(w: u32, h: u32, format: String, color_type: ColorType, bit_depth: u16) -> Self {
        Self { w, h, format, color_type, bit_depth }
    }
}

impl ToString for ImageMeta {
    fn to_string(&self) -> String {
        format!("dimension = {w}x{h}, format = {format}, color_type = {color_type:?}, bit_depth = {bit_depth}", w = self.w, h = self.h, format = self.format, color_type = self.color_type, bit_depth = self.bit_depth)
    }
}

pub struct ParsedImage {
    /// The meta data of the image
    pub meta: ImageMeta,
    /// DynamicImage is a wrapper around ImageBuffer that provides dynamic behavior, use `dyn_image.write_to(&mut buf, format)` to write the image to a buffer in the specified format
    pub dyn_image: DynamicImage,
}