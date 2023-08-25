use std::{fs};
use std::path::{PathBuf};
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

pub struct ArchiveEntry {
    /// 相对于 `disk_root` 的路径
    pub pack_dir: String,
    /// 是否为文件
    pub is_file: bool,
    /// 二进制数据
    pub raw: Option<Vec<u8>>,
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
}

impl ArchiveBuilder {
    /// 解析 `items`, 分为 `as_file`, `as_folder`, `ignored` 三类
    fn parse_records(&mut self, items: Vec<&str>) {
        for item in items {
            // 拼接出条目的完整路径
            let mut record_disk_dir = self.disk_root.clone();
            record_disk_dir.push(item);

            match PathType::parse(&record_disk_dir) {
                // 若为文件则添加文件
                PathType::File => {
                    self.as_file.push(item.to_string());
                }
                // 若为文件夹则添加文件夹中的所有文件
                PathType::Folder => {
                    for item in WalkDir::new(record_disk_dir) {
                        let item = item.unwrap();

                        // 路径层级统一成使用 '/' 分隔
                        match item.path().strip_prefix(self.disk_root.clone()) {
                            Ok(v) => {
                                let v = v.to_str().unwrap().to_string().replace(r"\", "/");

                                // 忽略空路径 (WalkDir 的根路径)
                                if v == "" {
                                    continue;
                                }

                                if item.file_type().is_dir() {
                                    self.as_folder.push(v);
                                } else if item.file_type().is_file() {
                                    self.as_file.push(v);
                                }
                            }
                            Err(err) => {
                                println!("StripPrefixError: {}", err);
                            }
                        }
                    }
                }
                // 不存在则忽略
                PathType::NotExist => {
                    self.ignored.push(item.to_string());
                }
            }
        }
    }

    /// read the binary of item, based on `disk_root`
    fn read_raw(&self, item: &str) -> Option<Vec<u8>> {
        let mut p = self.disk_root.clone();
        p.push(item);

        fs::read(p).ok()
    }

    /// parse items into `file` or `folder` or `ignored`, based on `disk_root`
    pub fn build(disk_root: impl Into<PathBuf>, items: Vec<&str>) -> ArchiveBuilder {
        let mut prefab = ArchiveBuilder {
            disk_root: disk_root.into(),
            as_file: vec![],
            as_folder: vec![],
            ignored: vec![],
        };

        prefab.parse_records(items);

        prefab
    }

    /// get entries
    pub fn get_entries(&self) -> Vec<ArchiveEntry> {
        let mut entries = vec![];

        for file in &self.as_file {
            let raw = self.read_raw(&file);
            entries.push(ArchiveEntry {
                pack_dir: file.to_string(),
                is_file: true,
                raw,
            });
        }

        for folder in &self.as_folder {
            entries.push(ArchiveEntry {
                pack_dir: folder.to_string(),
                is_file: false,
                raw: None,
            });
        }

        entries
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    /// ArchiveBuilder build 测试
    #[test]
    fn build_test() {
        let base = r"C:\Users\20366\Desktop\misc_test\zip";
        let items = vec!["."];

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
}