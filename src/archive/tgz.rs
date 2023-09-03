use std::cmp::Ordering;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use flate2::{Compression, GzBuilder};
use flate2::read::GzDecoder;
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

/// 将 tar 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (按照文件夹优先, 文件次之的顺序排序)
pub fn unpack_tar(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut entries = vec![];
    let disk_root: PathBuf = disk_root.into();

    let mut archive = tar::Archive::new(Cursor::new(binary));

    for item in archive.entries().unwrap() {
        let mut item = item.unwrap();
        let pack_dir = item.path().unwrap().to_str().unwrap().to_string();
        let is_file = item.header().entry_type().is_file();

        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file,
            raw: if !is_file { None } else {
                let mut raw: Vec<u8> = vec![];
                item.read_to_end(&mut raw).unwrap();
                Some(raw)
            },
        });
    }

    // 按照文件夹优先, 文件次之的顺序排序
    entries.sort_by(|a, b| {
        if (a.is_file == true) == (b.is_file == true) {
            Ordering::Equal
        } else if a.is_file {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    entries
}

/// 将 `ArchiveEntry` 列表打包为 zip, 返回二进制数据
pub fn pack_tar(entries: Vec<ArchiveEntry>) -> Vec<u8> {
    let mut bundle = tar::Builder::new(Cursor::new(vec![]));

    for entry in entries {
        if entry.is_file {
            let bytes = entry.raw.unwrap();
            let mut header = tar::Header::new_gnu();
            header.set_path(&entry.pack_dir).unwrap();
            header.set_size(bytes.len() as u64);
            header.set_cksum();
            bundle.append_data(&mut header, entry.pack_dir, &mut Cursor::new(bytes)).unwrap();
        } else {
            bundle.append_dir(entry.pack_dir, entry.disk_dir).unwrap();
        }
    }

    bundle.into_inner().unwrap().into_inner()
}


/// tgz(tar.gz) 即 tar + gzip
pub fn extract_tgz(tgz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match tar::Archive::new(
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
    match tar::Archive::new(Cursor::new(tar_buffer)).unpack(dest) {
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

    /// unpack 函数测试
    #[test]
    fn unpack_tar_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\pack_tar.tar").unwrap();
        let entries = unpack_tar(binary, r"C:\Users\20366\Desktop\misc\unpack_tar".into());

        fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_tar").unwrap();

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
    fn pack_tar_test() {
        let base = r"C:\Users\20366\Desktop\misc";
        let items = vec!["folder".into()];

        let entries = ArchiveBuilder::build(base.into(), items).get_entries();
        let package = pack_tar(entries);

        fs::write(r"C:\Users\20366\Desktop\misc\pack_tar.tar", package).unwrap();
        println!("done!");
    }
}