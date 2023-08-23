use std::{fs, io};
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 确保目标目录一定存在 (若不存在则自动创建)
pub fn ensured_path(target: String) -> Result<PathBuf, String> {
    let path = PathBuf::from(target);
    if !path.exists() {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(path),
            Err(err) => Err(format!("{err}"))
        }
    } else {
        Ok(path)
    }
}

/// 路径类型
#[derive(Debug)]
enum PathType { File, Dir, NotExist }

/// 获取路径的类型
fn get_path_type(p: &PathBuf) -> PathType {
    if !p.exists() {
        PathType::NotExist
    } else {
        if p.is_dir() {
            PathType::Dir
        } else {
            PathType::File
        }
    }
}

/// 压缩包中的一个条目
pub struct ArchiveEntry {
    /// 压缩包在本地的路径
    pub base: String,
    /// 条目在压缩包下的路径 (压缩包作为根目录)
    pub path_to_root: String,
    /// 是否为目录
    pub is_dir: bool,
    /// 若为文件，则包含文件内容
    pub binary: Option<Vec<u8>>,
}

impl ArchiveEntry {
    pub fn dir(base: &str, path_to_root: &str) -> ArchiveEntry {
        ArchiveEntry {
            base: base.into(),
            path_to_root: path_to_root.into(),
            is_dir: true,
            binary: None,
        }
    }

    pub fn file(base: &str, path_to_root: &str) -> io::Result<ArchiveEntry> {
        match fs::read(&base) {
            Ok(binary) => Ok(ArchiveEntry {
                base: base.into(),
                path_to_root: path_to_root.into(),
                is_dir: false,
                binary: Some(binary),
            }),
            Err(err) => Err(err),
        }
    }


    pub fn from_dir(base: &str, path_to_root: &str) -> Vec<ArchiveEntry> {
        let mut entries = vec![];

        // TODO: 遍历并添加文件夹中的所有文件, 若不存在子文件则添加空文件夹

        entries
    }

    pub fn from_path_list(base: &str, entry_list: Vec<&str>) -> Vec<ArchiveEntry> {
        let mut entries = vec![];

        for entry in entry_list {
            let mut p = PathBuf::from(base);
            p.push(entry);

            match get_path_type(&p) {
                // 若为文件则添加文件
                PathType::File => {
                    match ArchiveEntry::file(base, entry) {
                        Ok(entry) => entries.push(entry),
                        Err(err) => println!("Error: {} ({})", err, entry),
                    }
                }
                // 若为文件夹则添加文件夹中的所有文件
                PathType::Dir => {
                    let _entries = ArchiveEntry::from_dir(base, entry);
                    for entry in _entries {
                        entries.push(entry);
                    }
                }
                // 不存在则静默忽略
                _ => {}
            }
        }

        entries
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn walk() {
        // let base = r"D:\rstool\test";
        // let items = vec![
        //     r"\folder",
        //     "/aa.html",
        //     "img@64x64.ico",
        //     "rstool2.exe",
        //     "with blank.txt",
        //     "not_exist.txt",
        // ];

        let base = r"D:\pool\pack_test";
        let items = vec![
            r"\folder",
            "中文",
            "a.txt",
            "b.txt",
            "with blank.txt",
            "not_exist.txt",
        ];

        let list = ArchiveEntry::from_path_list(base, items);
        println!("list: {}", list.len());

        // let base = PathBuf::from(base);
        // for item in items {
        //     let mut p = base.clone();
        //     // FIXME: concat absolute paths
        //     p.push(item);
        //     println!("path type is: {:?}", p);
        // }
    }
}