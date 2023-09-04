use std::cmp::Ordering;
use std::io::{Cursor, Read, Write};
use std::path::{PathBuf};
use crate::archive::utils::ArchiveEntry;

/// 将 zip 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (按照文件夹优先, 文件次之的顺序排序)
pub fn unpack(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry> {
    let mut entries = vec![];
    let disk_root: PathBuf = disk_root.into();

    let mut archive = zip::ZipArchive::new(Cursor::new(binary)).unwrap();

    for idx in 0..archive.len() {
        let mut file = archive.by_index(idx).unwrap();
        let pack_dir = match file.enclosed_name() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            None => { continue; }
        };
        // let is_file = !file.name().ends_with('/');

        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file: file.is_file(),
            is_dir: file.is_dir(),
            raw: if file.name().ends_with('/') { None } else {
                let mut raw: Vec<u8> = vec![];
                file.read_to_end(&mut raw).unwrap();
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
pub fn pack(entries: Vec<ArchiveEntry>) -> Vec<u8> {
    let mut bundle = zip::ZipWriter::new(Cursor::new(vec![]));

    // 路径层级只能用 '/' 分隔
    for entry in entries {
        if entry.is_file {
            bundle.start_file(entry.pack_dir, Default::default()).unwrap();
            bundle.write_all(entry.raw.unwrap().as_slice()).unwrap();
        } else {
            bundle.add_directory(entry.pack_dir, Default::default()).unwrap();
        }
    }

    bundle.finish().unwrap().into_inner()
}

#[cfg(test)]
mod unit_test {
    use std::fs;
    use super::*;
    use crate::archive::utils::ArchiveBuilder;

    /// unpack 函数测试
    #[test]
    fn unpack_test() {
        let binary = fs::read(r"C:\Users\20366\Desktop\misc\folder.zip").unwrap();
        let entries = unpack(binary, r"C:\Users\20366\Desktop\misc\unpack_zip".into());

        // fs::create_dir_all(r"C:\Users\20366\Desktop\misc\unpack_test").unwrap();

        for entry in entries {
            println!("disk_dir: = {}\n pack_dir: = {}\n is_file: = {}\n\n", entry.disk_dir.to_str().unwrap(), entry.pack_dir, entry.is_file);

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
        let items = vec![".".into()];

        let entries = ArchiveBuilder::build(base.into(), items).get_entries();
        let package = pack(entries);

        fs::write(r"C:\Users\20366\Desktop\misc.zip", package).unwrap();
        println!("done!");
    }
}