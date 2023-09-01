use std::cmp::Ordering;
use std::io::{Cursor};
use std::path::{Path, PathBuf};
use sevenz_rust::Password;
use crate::archive::utils::ArchiveEntry;

pub fn extract_sevenz(sevenz_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match sevenz_rust::decompress(Cursor::new(sevenz_buffer), dest) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{err}")),
    }
}


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

pub fn pack(binary: Vec<u8>, disk_root: impl Into<PathBuf>) -> Vec<ArchiveEntry> {
    // let mut entries = vec![];
    // // let disk_root = disk_root.into();
    //
    // // TODO: 7z pack
    //
    // entries

    todo!()
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::fs;

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
}