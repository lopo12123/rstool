use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use flate2::{Compression, GzBuilder};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;
use crate::archive::utils::ArchiveEntry;

/// 将 gz 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (仅包含一项 `ArchiveEntry`)
pub fn unpack_gz(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut archive = GzDecoder::new(Cursor::new(binary));

    let disk_root: PathBuf = disk_root.into();
    let pack_dir = archive.header().map_or(
        "UnnamedFile".to_string(),
        |header| header.filename().map_or(
            "UnnamedFile".to_string(),
            |name| String::from_utf8_lossy(name).to_string(),
        ),
    );

    vec![
        ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file: true,
            raw: {
                let mut raw: Vec<u8> = vec![];
                archive.read_to_end(&mut raw).unwrap();
                Some(raw)
            },
        }
    ]
}

/// 将 `ArchiveEntry` 列表打包为 gz, 返回二进制数据 (仅压缩第一项文件, 若列表都是文件夹则返回空)
pub fn pack_gz(entries: Vec<ArchiveEntry>) -> Vec<u8> {
    // 找到第一个文件, 作为压缩对象
    match entries.into_iter().find(|entry| entry.is_file) {
        // 若列表中有文件则压缩第一个文件
        Some(file) => {
            let mut bundle = GzBuilder::new().filename(file.pack_dir);
            let mut bundle = bundle.write(Cursor::new(vec![]), Compression::default());
            bundle.write_all(file.raw.unwrap().as_slice()).unwrap();
            bundle.finish().unwrap().into_inner()
        }
        // 若列表都是文件夹则返回空
        None => {
            vec![]
        }
    }
}

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

#[cfg(test)]
mod unit_test {
    use std::fs;
    use super::*;
    use crate::archive::utils::ArchiveBuilder;

    /// unpack 函数测试
    #[test]
    fn unpack_gz_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\pack_gz.gz").unwrap();
        let entries = unpack_gz(binary, r"C:\Users\20366\Desktop\misc\unpack_gz".into());

        fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_gz").unwrap();

        for entry in entries {
            // println!("disk_dir: = {}\n pack_dir: = {}\n is_file: = {}\n\n", entry.disk_dir.to_str().unwrap(), entry.pack_dir, entry.is_file);

            if entry.is_file {
                fs::write(entry.disk_dir, entry.raw.unwrap()).unwrap();
            } else {
                fs::create_dir_all(entry.disk_dir).unwrap();
            }
        }
    }

    /// pack 函数测试
    #[test]
    fn pack_gz_test() {
        let base = r"C:\Users\20366\Desktop\misc";
        let items = vec!["你好.txt".into()];

        let entries = ArchiveBuilder::build(base.into(), items).get_entries();
        let package = pack_gz(entries);

        fs::write(r"C:\Users\20366\Desktop\misc\pack_gz.gz", package).unwrap();
        println!("done!");
    }
}