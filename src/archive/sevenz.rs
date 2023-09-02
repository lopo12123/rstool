use std::cmp::Ordering;
use std::io::{Cursor};
use std::path::{Path, PathBuf};
use sevenz_rust::{Password, SevenZArchiveEntry};
use crate::archive::utils::ArchiveEntry;

/// 将 7z 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (按照文件夹优先, 文件次之的顺序排序)
pub fn unpack(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut entries = vec![];
    let disk_root: PathBuf = disk_root.into();

    let len: u64 = binary.len() as u64;
    let mut archive = sevenz_rust::SevenZReader::new(
        Cursor::new(binary),
        len,
        Password::empty(),
    ).unwrap();

    archive.for_each_entries(|entry, reader| {
        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(entry.name()),
            pack_dir: entry.name.clone(),
            is_file: !entry.is_directory,
            raw: if entry.is_directory { None } else {
                let mut bytes: Vec<u8> = vec![];
                reader.read_to_end(&mut bytes).unwrap();
                Some(bytes)
            },
        });

        Ok(true)
    }).unwrap();


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

/// 将 `ArchiveEntry` 列表打包为 7z, 返回二进制数据
pub fn pack(entries: Vec<ArchiveEntry>) -> Vec<u8> {
    let mut bundle = sevenz_rust::SevenZWriter::new(Cursor::new(vec![])).unwrap();

    for entry in entries {
        let mut item = SevenZArchiveEntry::default();
        item.name = entry.pack_dir.clone();
        item.is_directory = !entry.is_file;

        bundle.push_archive_entry(item, entry.raw.map(|bytes| Cursor::new(bytes))).unwrap();
    }

    bundle.finish().unwrap().into_inner()
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::fs;
    use crate::archive::utils::ArchiveBuilder;

    /// unpack 函数测试
    #[test]
    fn unpack_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\folder.7z").unwrap();
        let entries = unpack(binary, r"C:\Users\20366\Desktop\misc\unpack_7z".into());

        // fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_7z").unwrap();

        for entry in entries {
            println!("disk_dir: = {}\npack_dir: = {}\nis_file: = {}\n\n", entry.disk_dir.to_str().unwrap(), entry.pack_dir, entry.is_file);

            // if entry.is_file {
            //     fs::write(entry.disk_dir, entry.raw.unwrap()).unwrap();
            // } else {
            //     fs::create_dir_all(entry.disk_dir).unwrap();
            // }
        }
    }

    /// pack 函数测试
    #[test]
    fn pack_test() {
        // let base = r"C:\Users\20366\Desktop\misc_test\zip";
        let base = r"C:\Users\20366\Desktop\misc";
        let items = vec!["folder".into()];

        let entries = ArchiveBuilder::build(base.into(), items).get_entries();
        let package = pack(entries);

        fs::write(r"C:\Users\20366\Desktop\misc\folder.7z", package).unwrap();
        println!("done!");
    }
}