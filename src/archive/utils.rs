use std::{fs, io};
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
    pub fn dir(base: impl Into<String>, path_to_root: impl Into<String>) -> ArchiveEntry {
        ArchiveEntry {
            base: base.into(),
            path_to_root: path_to_root.into(),
            is_dir: true,
            binary: None,
        }
    }
    pub fn file(base: impl AsRef<Path> + Into<String>, path_to_root: impl Into<String>) -> io::Result<ArchiveEntry> {
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

    pub fn from_path_list(base: impl AsRef<Path>, path_list: Vec<String>) -> Vec<ArchiveEntry> {
        let mut entries = vec![];


        entries
    }
}