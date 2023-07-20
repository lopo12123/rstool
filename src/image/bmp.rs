pub struct BmpMeta {
    pub width: u32,
    pub height: u32,
    pub bit_count: u16,
    pub compression: u32,
    pub image_size: u32,
    pub x_pixels_per_meter: u32,
    pub y_pixels_per_meter: u32,
    pub color_used: u32,
    pub color_important: u32,
}

/// Parse the raw data into the meta data and the image data
pub fn parse_bmp(bmp_buffer: Vec<u8>) {}

pub fn write_bmp() {}

mod unit_tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn test_parse_bmp() {
        let bmp_path = Path::new("test.bmp");
    }
}