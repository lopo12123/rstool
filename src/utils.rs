use std::fs;

pub struct Utils {}

impl Utils {
    /// 检查目标是否存在且是文件
    pub fn check_file(filepath: &str) -> bool {
        match fs::metadata(&filepath) {
            Ok(meta) => meta.is_file(),
            Err(_) => false
        }
    }

    /// 检查目标是否存在且是目录
    pub fn check_dir(filepath: &str) -> bool {
        match fs::metadata(&filepath) {
            Ok(meta) => meta.is_dir(),
            Err(_) => false
        }
    }
}