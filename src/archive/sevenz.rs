use std::io::Cursor;
use std::path::Path;

pub fn extract_sevenz(sevenz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match sevenz_rust::decompress(Cursor::new(sevenz_buffer), dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{err}")),
    }
}