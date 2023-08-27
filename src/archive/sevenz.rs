use std::io::Cursor;
use std::path::{Path, PathBuf};
use crate::archive::utils::ArchiveEntry;

pub fn extract_sevenz(sevenz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match sevenz_rust::decompress(Cursor::new(sevenz_buffer), dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{err}")),
    }
}

pub fn pack(binary: Vec<u8>, disk_root: impl Into<PathBuf>) -> Vec<ArchiveEntry>{
    let mut entries = vec![];
    let disk_root = disk_root.into();

    // TODO: 7z pack

    entries
}

#[cfg(test)]
mod unit_test {}