use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use flate2::read::GzDecoder;
use tar::Archive;

/// tgz(tar.gz) 即 tar + gzip
pub fn extract_tgz(tgz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match Archive::new(
        GzDecoder::new(Cursor::new(tgz_buffer))
    ).unpack(dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}

/// gz 仅单文件压缩
pub fn extract_gz(gz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match File::create(PathBuf::from(dest).join("UNPACKED_GZFILE")) {
        Ok(mut unpacked_gz) => {
            let mut gz = GzDecoder::new(Cursor::new(gz_buffer));
            match std::io::copy(&mut gz, &mut unpacked_gz) {
                Ok(_) => Ok(()),
                Err(write_err) => Err(format!("{}", write_err)),
            }
        }
        Err(create_err) => Err(format!("{}", create_err)),
    }
}

/// tar 仅归档无压缩
pub fn extract_tar(tar_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match Archive::new(Cursor::new(tar_buffer)).unpack(dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }
}