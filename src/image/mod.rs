use std::path::{Path, PathBuf};
use image::DynamicImage;
use crate::image::png::{parse_png, to_png};
use crate::image::utils::ParsedImage;

mod bmp;
mod png;
mod utils;

/// Try to parse the format from the file extension
fn try_parse_format(source: &Path) -> Option<String> {
    match source.extension() {
        Some(ext) => match ext.to_str() {
            Some("bmp") => Some("bmp".to_string()),
            Some("gif") => Some("gif".to_string()),
            Some("jpg" | "jpeg") => Some("jpg".to_string()),
            Some("png") => Some("png".to_string()),
            Some("tiff") => Some("tiff".to_string()),
            _ => None,
        }
        None => None,
    }
}

/// Try to parse the size from the string
fn try_parse_size(size: Option<String>) -> (Option<u32>, Option<u32>) {
    match size {
        Some(size_str) => {
            let size_vec: Vec<&str> = size_str.split('x').collect();
            if size_vec.len() == 2 {
                (
                    size_vec[0].parse::<u32>().ok(),
                    size_vec[1].parse::<u32>().ok()
                )
            } else {
                (None, None)
            }
        }
        None => (None, None),
    }
}

fn calc_final_size(size: (Option<u32>, Option<u32>), w: u32, h: u32) -> Option<(u32, u32)> {
    let (nw, nh) = size;
    if nw.is_some() && nh.is_some() {
        Some((nw.unwrap(), nh.unwrap()))
    } else if nw.is_none() && nh.is_none() {
        // (w, h)
        None
    } else if nw.is_some() {
        Some((nw.unwrap(), (nw.unwrap() as f32 / w as f32 * h as f32) as u32))
    } else {
        Some(((nh.unwrap() as f32 / h as f32 * w as f32) as u32, nh.unwrap()))
    }
}

fn normalized_size(size: Option<(u32, u32)>) -> String {
    match size {
        Some((w, h)) => format!("{w}x{h}"),
        None => "".to_string()
    }
}

pub struct ImageImpl {}

impl ImageImpl {
    /// Read the image from the buffer (with format specified), get the meta data and the raw data
    fn read_image_buffer(source_buffer: Vec<u8>, format: String) -> Result<ParsedImage, String> {
        match format.as_str() {
            "png" => parse_png(source_buffer),
            _ => Err(format!("Invalid source format. (Expect one of: bmp, gif, jpg/jpeg, png, tiff, Got: {format})"))
        }
    }

    /// Read the image from the path, get the meta data and the raw data
    fn read_image(source: &Path) -> Result<ParsedImage, String> {
        // Try to parse the format from the file extension if it is not provided
        match try_parse_format(source) {
            Some(source_format) => {
                // Read the raw data from the source file
                let file_raw = match std::fs::read(source) {
                    Ok(raw_file) => raw_file,
                    Err(err) => return Err(format!("{err}"))
                };

                ImageImpl::read_image_buffer(file_raw, source_format)
            }
            None => Err(format!("Invalid format. (Could not parse format from file extension)"))
        }
    }

    /// Generate the dynamic image to the buffer (with format specified)
    fn generate_image(dyn_image: DynamicImage, format: &str, size: Option<(u32, u32)>) -> Result<Vec<u8>, String> {
        match format {
            "png" => to_png(dyn_image, size),
            _ => Err(format!("Invalid target format. (Expect one of: bmp, gif, jpg/jpeg, png, tiff, Got: {format})"))
        }
    }

    pub fn handle(source: String, format: Option<String>, size: Option<String>) {
        println!("[Commands::Image] source = '{source}', format = '{format:?}', size = '{size:?}'");

        let source_path = Path::new(&source);

        if !source_path.exists() {
            println!("Error: Source file does not exist");
        } else if source_path.is_dir() {
            println!("Error: Source file is not a file");
        } else {
            match ImageImpl::read_image(source_path) {
                Ok(parsed_image) => {
                    println!("Image meta: {}", parsed_image.meta.to_string());

                    if format.is_some() || size.is_some() {
                        let target_format = format.unwrap_or(parsed_image.meta.format);
                        let target_size = calc_final_size(try_parse_size(size), parsed_image.meta.w, parsed_image.meta.h);

                        match ImageImpl::generate_image(parsed_image.dyn_image, &target_format, target_size) {
                            Ok(image_buffer) => {
                                let target_stem = source_path.file_stem().map_or("", |stem| stem.to_str().unwrap_or("unknown")).to_string();
                                // write to file
                                let mut target_path = PathBuf::from(source_path.parent().unwrap_or(Path::new("")));
                                target_path.push(format!("{}@{}.{}", target_stem, normalized_size(target_size), target_format));
                                match std::fs::write(&target_path, image_buffer) {
                                    Ok(_) => println!("Ok. (Image generated successfully at '{:?}')", target_path),
                                    Err(write_err) => println!("Error: {write_err}")
                                }
                            }
                            Err(generate_err) => println!("Error: {generate_err}")
                        }
                    }
                }
                Err(parse_err) => println!("Error: {parse_err}"),
            }
        }
    }
}