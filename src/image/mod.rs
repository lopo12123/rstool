use std::path::{Path, PathBuf};

mod bmp;
mod png;
mod utils;

pub struct ImageImpl {}

impl ImageImpl {
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

    /// Read the image from the source file, get the meta data and the raw data
    fn read_image(source: &Path, format: Option<String>) -> Result<(), String> {
        // Try to parse the format from the file extension if it is not provided
        match format.or(ImageImpl::try_parse_format(source)) {
            Some(image_format) => {
                // Read the raw data from the source file
                let file_raw = match std::fs::read(source) {
                    Ok(raw_file) => raw_file,
                    Err(err) => return Err(format!("{err}"))
                };

                // Parse the raw data into the meta data and the image data
                // match image_format.as_str() {
                //     "bmp" => match bmp::parse_bmp(file_raw) {
                //         Ok(_) => {
                //             Ok(())
                //         }
                //         Err(parse_err) => Err(format!("{parse_err}"))
                //     }
                //     _ => Err(format!("Invalid format. (Expect one of: bmp, gif, jpg/jpeg, png, tiff, Got: {image_format:?})"))
                // }

                Ok(())
            }
            None => Err(format!("Invalid format. (Could not parse format from file extension)"))
        }
    }


    pub fn handle(source: String, format: Option<String>, width: Option<u32>, height: Option<u32>) {
        println!("[Commands::Image] source: '{source}' format: '{format:?}' width: '{width:?}' height: '{height:?}'");

        let source_path = Path::new(&source);

        if !source_path.exists() {
            println!("Error: Source file does not exist");
        } else if source_path.is_dir() {
            println!("Error: Source file is not a file");
        } else {}
    }
}