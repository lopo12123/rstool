use std::{fs};
use std::path::{PathBuf};
use walkdir::WalkDir;

// --------------------- PathType --------------------------
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

// --------------------- ArchiveEntry ---------------------
pub struct ArchiveEntry {
    /// 条目在磁盘上的路径
    pub disk_dir: PathBuf,
    /// 条目在归档中的路径, 相对于 `disk_root`
    pub pack_dir: String,
    /// 是否为文件
    pub is_file: bool,
    /// 是否为文件夹
    pub is_dir: bool,
    /// 二进制数据
    pub raw: Option<Vec<u8>>,
}

// --------------------- ArchiveBuilder ---------------------
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
    fn parse_items(&mut self, items: Vec<String>) {
        for item in items {
            // 拼接出条目的完整路径
            let mut disk_dir = self.disk_root.clone();
            disk_dir.push(item.clone());

            match PathType::parse(&disk_dir) {
                // 若为文件则添加文件
                PathType::File => {
                    self.as_file.push(item);
                }
                // 若为文件夹则添加文件夹中的所有文件
                PathType::Folder => {
                    for item in WalkDir::new(disk_dir) {
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
                    self.ignored.push(item);
                }
            }
        }
    }

    /// parse items into `file` or `folder` or `ignored`, based on `disk_root`
    pub fn build(disk_root: PathBuf, items: Vec<String>) -> ArchiveBuilder {
        let mut prefab = ArchiveBuilder {
            disk_root,
            as_file: vec![],
            as_folder: vec![],
            ignored: vec![],
        };

        prefab.parse_items(items);

        prefab
    }

    /// get entries
    pub fn get_entries(&self) -> Vec<ArchiveEntry> {
        let mut entries = vec![];

        for file in &self.as_file {
            let mut disk_dir = self.disk_root.clone();
            disk_dir.push(&file);

            let raw = fs::read(&disk_dir).ok();

            entries.push(ArchiveEntry {
                disk_dir,
                pack_dir: file.to_string(),
                is_file: true,
                is_dir: false,
                raw,
            });
        }

        for folder in &self.as_folder {
            let mut disk_dir = self.disk_root.clone();
            disk_dir.push(&folder);

            entries.push(ArchiveEntry {
                disk_dir,
                pack_dir: folder.to_string(),
                is_file: false,
                is_dir: true,
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
        // let base = r"C:\Users\20366\Desktop\misc_test\zip";
        let base = r"C:\Users\20366\Desktop\misc";
        let items = vec![".".into()];

        let b = ArchiveBuilder::build(base.into(), items);

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