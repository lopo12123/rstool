use std::{fs, io};
use std::cmp::Ordering;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use crate::archive::utils::ArchiveEntry;

pub fn extract_zip(zip_buffer: Vec<u8>, desc: &Path) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(zip_buffer)).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => desc.join(path),
            None => continue,
        };

        // if the entry is a directory, create the directory
        if file.name().ends_with('/') {
            fs::create_dir_all(outpath).unwrap();
        }
        // if the entry is a file, extract it
        else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}

/// 将 zip 解包为 `ArchiveEntry` 列表, 返回 `ArchiveEntry` 列表 (按照文件夹优先, 文件次之的顺序排序)
pub fn unpack(binary: Vec<u8>, disk_root: impl Into<PathBuf>) -> Vec<ArchiveEntry> {
    let mut entries = vec![];
    let disk_root = disk_root.into();

    let mut archive = zip::ZipArchive::new(Cursor::new(binary)).unwrap();

    for idx in 0..archive.len() {
        let mut file = archive.by_index(idx).unwrap();
        let pack_dir = match file.enclosed_name() {
            Some(dir) => dir.to_str().unwrap().to_string(),
            None => { continue; }
        };
        let is_file = !file.name().ends_with('/');

        entries.push(ArchiveEntry {
            disk_dir: disk_root.join(&pack_dir),
            pack_dir,
            is_file,
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
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use crate::archive::utils::ArchiveBuilder;

    /// unpack 函数测试
    #[test]
    fn unpack_test() {
        let archive = r"C:\Users\20366\Desktop\misc_test\zip_package.zip";
        let binary = fs::read(archive).unwrap();
        let entries = unpack(binary, r"C:\Users\20366\Desktop\misc_test\unpack_test");

        fs::create_dir_all(r"C:\Users\20366\Desktop\misc_test\unpack_test").unwrap();

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
    fn pack_test() {
        let base = r"C:\Users\20366\Desktop\misc_test\zip";
        let items = vec!["."];

        let entries = ArchiveBuilder::build(base, items).get_entries();
        let package = pack(entries);
        fs::write(r"C:\Users\20366\Desktop\misc_test\zip_package.zip", package).unwrap();
        println!("done!");
    }

    #[test]
    /// zip 打包测试
    fn zip_test() {
        let path = Path::new(r"D:\rstool\test\pack.zip");
        let path = Path::new(r"C:\Users\20366\Desktop\misc_test\pack.zip");
        let file = File::create(path).unwrap();

        let mut zip = zip::ZipWriter::new(file);

        // zip.add_directory("根目录/", Default::default());
        zip.add_directory("根目录/子目录1", Default::default());
        zip.add_directory("根目录\\子目录1", Default::default());
        zip.add_directory("根目录/子目录2", Default::default());
        zip.add_directory("根目录/sub1/sub2", Default::default());
        zip.add_directory("根目录/sub1/sub2/sub3", Default::default());
        zip.start_file("根目录/子目录3/文件1.txt", Default::default()).unwrap();
        zip.write_all(b"Hello, World!").unwrap();

        zip.finish();
    }
}