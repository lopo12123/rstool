use std::{fs, io};
use std::path::{Path, PathBuf};

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
    /// 条目路径 (相对于压缩包根目录)
    pub path_to_root: String,
    /// 是否为目录
    pub is_dir: bool,
    /// 若为文件，则包含文件内容
    pub binary: Option<Vec<u8>>,
}

impl ArchiveEntry {
    pub fn dir(path_to_root: impl Into<String>) -> ArchiveEntry {
        ArchiveEntry { path_to_root: path_to_root.into(), is_dir: true, binary: None }
    }
    pub fn file(path_to_root: impl Into<String>, disk_path: impl AsRef<Path>) -> io::Result<ArchiveEntry> {
        match fs::read(disk_path) {
            Ok(binary) => Ok(ArchiveEntry {
                path_to_root: path_to_root.into(),
                is_dir: false,
                binary: Some(binary),
            }),
            Err(err) => Err(err),
        }
    }
}