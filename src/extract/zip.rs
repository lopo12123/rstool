use std::io::Cursor;
use std::path::{Path};

pub fn extract_zip(zip_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match zip_extract::extract(
        Cursor::new(zip_buffer),
        dest,
        false,
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err))
    }
}