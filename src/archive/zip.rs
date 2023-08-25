use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;
use crate::archive::utils::ArchiveEntry;

pub fn unpack(zip_buffer: Vec<u8>, desc: &Path) -> Result<(), String> {
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