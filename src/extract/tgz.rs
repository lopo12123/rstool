use std::io::Cursor;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn extract_tgz(tgz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match Archive::new(
        GzDecoder::new(Cursor::new(tgz_buffer))
    ).unpack(dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}