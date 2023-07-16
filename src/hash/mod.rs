mod md5;
mod sha;

use std::fs;
use crate::utils::Utils;

pub struct HashImpl {}

impl HashImpl {
    /// 处理 Command::Hash 子命令
    pub fn handle(source: String, filemode: bool, algorithm: String) {
        println!("[Commands::Hash] source: '{source}', filemode: '{filemode}', algorithm: '{algorithm}'");

        // TODO
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn check() {
        let a = "";
        let b = "";
        println!("{}", a == b);
    }
}