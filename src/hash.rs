use std::fs;
use std::process::Command;
use regex::Regex;
use crate::utils::Utils;

pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl Algorithm {
    /// 在目标字符串中找寻可能是当前算法结果的串
    pub fn find<'a>(&self, text: &'a str) -> &'a str {
        let len = match self {
            Algorithm::Md5 => "32",
            Algorithm::Sha1 => "40",
            Algorithm::Sha256 => "64",
            Algorithm::Sha512 => "128",
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
    pub fn to_name(&self) -> &str {
        match self {
            Algorithm::Md5 => "md5",
            Algorithm::Sha1 => "sha1",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
        }
    }

    /// 从算法名生成
    pub fn from_name(name: &str) -> Option<Algorithm> {
        match name.to_lowercase().as_str() {
            "md5" => Some(Algorithm::Md5),
            "sha1" => Some(Algorithm::Sha1),
            "sha256" => Some(Algorithm::Sha256),
            "sha512" => Some(Algorithm::Sha512),
            _ => None,
        }
    }
}

pub struct HashImpl {}

impl HashImpl {
    /// 获取目标文件的哈希
    ///
    /// 如果目标不存在或非文件则抛出错误
    fn hash(filepath: &str, algorithm_name: &str) -> Result<String, String> {
        if !Utils::check_file(filepath) {
            return Err(format!("file does not exist, or is not a file"));
        }
        match Algorithm::from_name(algorithm_name) {
            Some(algorithm) => match Command::new("certutil")
                .args(&["-hashfile", filepath, algorithm.to_name()])
                .output() {
                Ok(out) => {
                    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&out.stderr).to_string();

                    if out.status.success() {
                        let digest = algorithm.find(&stdout);
                        if digest == "" {
                            Err(format!("Unable to find hash value from the result, the original output is as follows:\n{stdout}"))
                        } else {
                            Ok(digest.to_string())
                        }
                    } else {
                        Err(
                            if stdout.is_empty() && stderr.is_empty() {
                                format!("Failed to obtain file hash. No standard output or standard error output")
                            } else {
                                format!("Failed to obtain file hash. The standard output and standard error output are as follows:\n<stdout>: {stdout}\n<stderr>: {stderr}")
                            }
                        )
                    }
                }
                Err(_) => Err(format!("Failed to execute process")),
            }
            None => Err(format!("Invalid algorithm"))
        }
    }

    /// 处理 Command::Hash 子命令
    pub fn handle(file: String, algorithm: String) {
        println!("[Commands::Hash] file: '{file}', algorithm: '{algorithm}'");

        match HashImpl::hash(&file, &algorithm) {
            Ok(res) => println!("Ok: {res}"),
            Err(err) => println!("Error: {err}"),
        };
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

    #[test]
    fn ttt() {
        let r = r"CertUtil: -hashfile 失败: 0x800703ee (WIN32: 1006 ERROR_FILE_INVALID)
CertUtil: 文件所在的卷已被外部更改，因此打开的文件不再有效。
";

        println!("{}", if Algorithm::Md5.find(r) == "" { "empty" } else { "ok" });
    }
}