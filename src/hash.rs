use std::fmt::format;
use std::fs;
use std::process::Command;
use regex::Regex;

pub enum HashType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl HashType {
    /// 在目标字符串中找寻可能是当前算法结果的串
    pub fn find<'a>(&self, text: &'a str) -> &'a str {
        let len = match self {
            HashType::Md5 => "32",
            HashType::Sha1 => "40",
            HashType::Sha256 => "64",
            HashType::Sha512 => "128",
        };

        Regex::new(&format!("\\b[0-9a-zA-Z]{{{}}}\\b", len))
            .map_or(
                "",
                |re| re.captures(text)
                    .map_or(
                        "",
                        |cap| cap.get(0)
                            .map_or(
                                "",
                                |s| s.as_str(),
                            ),
                    ),
            )
    }

    /// 获取算法名
    pub fn to_str(&self) -> &str {
        match self {
            HashType::Md5 => "md5",
            HashType::Sha1 => "sha1",
            HashType::Sha256 => "sha256",
            HashType::Sha512 => "sha512",
        }
    }
}

pub struct HashImpl {}

impl HashImpl {
    /// 检查目标是否存在且是文件
    pub(crate) fn check(filepath: &str) -> bool {
        match fs::metadata(&filepath) {
            Ok(meta) => meta.is_file(),
            Err(_) => false
        }
    }

    /// 获取目标文件的哈希
    ///
    /// 如果目标不存在或非文件则抛出错误
    pub fn hash(filepath: &str, algorithm: HashType) -> Result<String, String> {
        if !Self::check(filepath) {
            return Err(format!("target does not exist or is not a file"));
        }

        match Command::new("certutil")
            .args(&["-hashfile", filepath, algorithm.to_str()])
            .output() {
            Ok(r) => if r.status.success() {
                Ok(String::from_utf8_lossy(&r.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&r.stderr).to_string())
            },
            Err(_) => Err(format!("failed to execute process")),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn check() {
        let r = HashImpl::check("rstool.exe");
        println!("r: {}", r);
    }

    #[test]
    fn ttt() {
        let r = r"SHA256 的 rstool.exe 哈希:
686539a58446f82405c8215f30a81c3fad5afd44edfbee2725ebf685fb1d5209
CertUtil: -hashfile 命令成功完成。";

        println!("{}", HashType::Sha256.find(r));
    }
}