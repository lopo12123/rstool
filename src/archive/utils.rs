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
enum PathType { File, Folder, NotExist }

impl PathType {
    /// 获取路径的类型
    pub fn parse(p: &PathBuf) -> PathType {
        if !p.exists() {
            PathType::NotExist
        } else {
            if p.is_dir() {
                PathType::Folder
            } else {
                PathType::File
            }
        }
    }
}

/// 压缩包中的一个条目
pub struct ArchiveEntry {
    /// 条目在压缩包下的路径 (压缩包作为根目录)
    pub pack_dir: String,
    /// 是否为目录
    pub is_dir: bool,
    /// 若为文件，则包含文件内容, 否则为 None
    pub binary: Option<Vec<u8>>,
}

impl ArchiveEntry {
    pub fn dir(pack_dir: impl Into<String>) -> ArchiveEntry {
        ArchiveEntry {
            pack_dir: pack_dir.into(),
            is_dir: true,
            binary: None,
        }
    }

    pub fn file(pack_dir: impl Into<String>, binary: Vec<u8>) -> ArchiveEntry {
        ArchiveEntry {
            pack_dir: pack_dir.into(),
            is_dir: false,
            binary: Some(binary),
        }
    }
}

pub struct ArchiveBuilder {
    /// 本地的根目录
    pub disk_root: PathBuf,
    /// 解析后判定为文件 (相对于 `disk_root`)
    pub as_file: Vec<String>,
    /// 解析后判定为目录 (相对于 `disk_root`)
    pub as_folder: Vec<String>,
    /// 解析后判定为忽略 (不存在或无权限) (相对于 `disk_root`)
    pub ignored: Vec<String>,
    /// 压缩包内容项
    pub entries: Vec<ArchiveEntry>,
}

impl ArchiveBuilder {
    pub fn build(disk_root: impl Into<PathBuf>, records: Vec<&str>) -> ArchiveBuilder {
        let mut prefab = ArchiveBuilder {
            disk_root: disk_root.into(),
            as_file: vec![],
            as_folder: vec![],
            ignored: vec![],
            entries: vec![],
        };

        prefab.parse_records(records);

        // for entry in entries {
        //     let mut p = PathBuf::from(disk_dir_base);
        //     p.push(entry);
        //
        //     match PathType::parse(&p) {
        //         // 若为文件则添加文件
        //         PathType::File => {
        //             match ArchiveEntry::file(p, entry) {
        //                 Ok(archive_entry) => archive_entry_list.push(archive_entry),
        //                 Err(err) => println!("Error: {} ({})", err, entry),
        //             }
        //         }
        //         // 若为文件夹则添加文件夹中的所有文件
        //         PathType::Folder => {
        //             let _entries = ArchiveEntry::from_dir(base, entry);
        //             for entry in _entries {
        //                 archive_entry_list.push(entry);
        //             }
        //         }
        //         // 不存在则静默忽略
        //         _ => {}
        //     }
        // }

        // ArchiveBuilder {
        //     entries: vec![]
        // }

        prefab
    }

    /// 解析 `records`, 分为 `as_file`, `as_folder`, `ignored` 三类
    fn parse_records(&mut self, records: Vec<&str>) {
        for record in records {
            // 拼接出条目的完整路径
            let mut record_disk_dir = self.disk_root.clone();
            record_disk_dir.push(record);

            match PathType::parse(&record_disk_dir) {
                // 若为文件则添加文件
                PathType::File => {
                    self.as_file.push(record.to_string());
                }
                // 若为文件夹则添加文件夹中的所有文件
                PathType::Folder => {
                    for item in WalkDir::new(record_disk_dir) {
                        let item = item.unwrap();
                        if item.file_type().is_dir() {
                            match item.path().strip_prefix(self.disk_root.clone()) {
                                Ok(v) => {
                                    println!("Ok: {}", v.to_str().unwrap());
                                }
                                Err(err) => {
                                    println!("Error: {}", err);
                                }
                            }
                        } else if item.file_type().is_file() {
                            // TODO: 直接添加
                        }
                    }
                }
                // 不存在则忽略
                PathType::NotExist => {
                    self.ignored.push(record.to_string());
                }
            }
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn build_test() {
        let base = r"D:\rstool\test";
        let items = vec![
            "folder",
            r"\folder",
            "/aa.html",
            "img@64x64.ico",
            "rstool2.exe",
            "with blank.txt",
            "not_exist.txt",
        ];

        let b = ArchiveBuilder::build(base, items);

        for folder in b.as_folder {
            println!("folder: {}", folder);
        }
        for file in b.as_file {
            println!("file: {}", file);
        }
        for ignore in b.ignored {
            println!("ignore: {}", ignore);
        }
    }

    #[test]
    fn walk() {
        let base = r"D:\rstool\test";
        let items = vec![
            r"\folder",
            "/aa.html",
            "img@64x64.ico",
            "rstool2.exe",
            "with blank.txt",
            "not_exist.txt",
        ];

        let mut p = PathBuf::from(base);
        p.push("img@64x64.ico");

        println!("exists: {}", p.exists());

        match fs::read(&p) {
            Ok(_) => {
                println!("ok");
            }
            Err(err) => {
                println!("error: {}", err);
            }
        };

        // let base = r"D:\pool\pack_test";
        // let items = vec![
        //     r"\folder",
        //     "中文",
        //     "a.txt",
        //     "b.txt",
        //     "with blank.txt",
        //     "not_exist.txt",
        // ];

        // let list = ArchiveEntry::from_path_list(base, items);
        // println!("list: {}", list.len());
    }
}