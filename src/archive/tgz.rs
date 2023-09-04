use std::cmp::Ordering;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use flate2::{Compression, GzBuilder};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use crate::archive::utils::ArchiveEntry;

/// 将 gz 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (仅包含一项 `ArchiveEntry`)
pub fn unpack_gz(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut archive = GzDecoder::new(Cursor::new(binary));

    let disk_root: PathBuf = disk_root.into();
    let pack_dir = archive.header().map_or(
        "UNKNOWN_FILE".to_string(),
        |header| header.filename().map_or(
            "UNKNOWN_FILE".to_string(),
            |name| String::from_utf8_lossy(name).to_string(),
        ),
    );

    vec![
        ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file: true,
            is_dir: false,
            raw: {
                let mut raw: Vec<u8> = vec![];
                archive.read_to_end(&mut raw).unwrap();
                Some(raw)
            },
        }
    ]
}

/// 将 `ArchiveEntry` 列表打包为 gz, 返回二进制数据 (仅压缩第一项文件, 若列表都是文件夹则返回空)
pub fn pack_gz(entries: Vec<ArchiveEntry>, _filename: String) -> Vec<u8> {
    // 找到第一个文件, 作为压缩对象
    match entries.into_iter().find(|entry| entry.is_file) {
        // 若列表中有文件则压缩第一个文件
        Some(file) => {
            let filename = PathBuf::from(file.pack_dir).file_name().map_or("UNKNOWN_FILE".to_string(), |name| name.to_str().unwrap().to_string());
            println!("WARNING: 'pack_gz' only compress the first file, filename = '{}'", filename);
            let mut bundle = GzBuilder::new().filename(filename).write(Cursor::new(vec![]), Compression::default());
            bundle.write_all(file.raw.unwrap().as_slice()).unwrap();
            bundle.finish().unwrap().into_inner()
        }
        // 若列表都是文件夹则返回空
        None => {
            println!("Error: 'pack_gz' failed, all entries are folders");
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
        let is_dir = item.header().entry_type().is_dir();

        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file,
            is_dir,
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

/// 将 `ArchiveEntry` 列表打包为 tar, 返回二进制数据
pub fn pack_tar(entries: Vec<ArchiveEntry>, filename: String) -> Vec<u8> {
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


/// 将 tgz 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (按照文件夹优先, 文件次之的顺序排序)
pub fn unpack_tgz(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut entries = vec![];
    let disk_root: PathBuf = disk_root.into();

    let mut archive = tar::Archive::new(GzDecoder::new(Cursor::new(binary)));

    for item in archive.entries().unwrap() {
        let mut item = item.unwrap();
        let pack_dir = item.path().unwrap().to_str().unwrap().to_string();
        let is_file = item.header().entry_type().is_file();
        let is_dir = item.header().entry_type().is_dir();

        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file,
            is_dir,
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


/// 将 `ArchiveEntry` 列表打包为 tgz, 返回二进制数据
pub fn pack_tgz(entries: Vec<ArchiveEntry>) -> Vec<u8> {
    let mut tar_bundle = tar::Builder::new(Cursor::new(vec![]));

    for entry in entries {
        if entry.is_file {
            let bytes = entry.raw.unwrap();
            let mut header = tar::Header::new_gnu();
            header.set_path(&entry.pack_dir).unwrap();
            header.set_size(bytes.len() as u64);
            header.set_cksum();
            tar_bundle.append_data(&mut header, entry.pack_dir, &mut Cursor::new(bytes)).unwrap();
        } else {
            tar_bundle.append_dir(entry.pack_dir, entry.disk_dir).unwrap();
        }
    }

    // let gz_bundle = GzBuilder::new().filename("middle.tar").write(Cursor::new(vec![]), Compression::default());

    let tar_raw = tar_bundle.into_inner().unwrap().into_inner();

    // pack_gz(vec![
    //     ArchiveEntry {
    //         disk_dir: PathBuf::from("middle.tar"),
    //         pack_dir: "middle.tar".into(),
    //         is_file: true,
    //         is_dir: false,
    //         raw: Some(tar_raw),
    //     }
    // ])
    todo!()

    // tar_bundle.into_inner().unwrap().finish().unwrap().into_inner()

    // let mut gz_bundle = GzBuilder::new().filename("compress.tar").write(Cursor::new(vec![]), Compression::default());
    // gz_bundle.write_all(&bundle.into_inner().unwrap().into_inner()).unwrap();
    // gz_bundle.finish().unwrap().into_inner()
    // gz_bundle.write(&bundle.into_inner().unwrap().into_inner()).unwrap();
    // gz_bundle.finish().unwrap().into_inner()
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
        let package = pack_gz(entries, "".to_string());

        fs::write(r"C:\Users\20366\Desktop\misc\pack_gz.gz", package).unwrap();
        println!("done!");
    }

    /// unpack 函数测试
    #[test]
    fn unpack_tar_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\tar-rs-0.4.40.tar").unwrap();
        let entries = unpack_tar(binary, r"C:\Users\20366\Desktop\misc\unpack_tar".into());

        fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_tar").unwrap();

        for entry in entries {
            println!("disk_dir: = {}\npack_dir: = {}\nis_file: = {}\nis_dir: = {}\n", entry.disk_dir.to_str().unwrap(), entry.pack_dir, entry.is_file, entry.is_dir);

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
        let package = pack_tar(entries, "".to_string());

        fs::write(r"C:\Users\20366\Desktop\misc\pack_tar.tar", package).unwrap();
        println!("done!");
    }

    /// unpack 函数测试
    #[test]
    fn unpack_tgz_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\pack_tgz.tgz").unwrap();
        let entries = unpack_tar(binary, r"C:\Users\20366\Desktop\misc\unpack_tgz".into());

        fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_tgz").unwrap();

        for entry in entries {
            println!("disk_dir: = {}\n pack_dir: = {}\n is_file: = {}\n\n", entry.disk_dir.to_str().unwrap(), entry.pack_dir, entry.is_file);

            if entry.is_file {
                fs::write(entry.disk_dir, entry.raw.unwrap()).unwrap();
            } else {
                fs::create_dir_all(entry.disk_dir).unwrap();
            }
        }
    }

    /// pack 函数测试
    #[test]
    fn pack_tgz_test() {
        let base = r"C:\Users\20366\Desktop\misc";
        let items = vec!["folder".into()];

        let entries = ArchiveBuilder::build(base.into(), items).get_entries();
        let package = pack_tar(entries, "".to_string());

        fs::write(r"C:\Users\20366\Desktop\misc\pack_tgz.tgz", package).unwrap();
        println!("done!");
    }
}