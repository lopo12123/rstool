use serde::Deserialize;
use std::env::{consts, split_paths, var};

const OWNER: &str = "lopo12123";
const REPO: &str = "rstool";

const PASS: &str = "[√]";
const WARN: &str = "[!]";
const FAIL: &str = "[×]";

#[derive(Debug, Deserialize)]
struct ReleaseInfo {
    tag_name: String,
}

pub struct DoctorImpl {}

impl DoctorImpl {
    /// 获取最新版本
    fn get_latest_version() -> Result<String, String> {
        match reqwest::blocking::Client::builder().user_agent("reqwest").build() {
            Ok(client) => match client.get(format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest")).send() {
                Ok(response) => match response.json::<ReleaseInfo>() {
                    Ok(release_info) => {
                        Ok(release_info.tag_name.replace("v", ""))
                    }
                    Err(_) => Err(format!("failed to parse response"))
                }
                Err(_) => Err(format!("failed to send request"))
            }
            Err(_) => Err(format!("failed to build request client"))
        }
    }

    /// 检查操作系统
    fn check_os(verbose: bool) {
        if verbose {
            println!("{} OS (required 'windows', current: '{}')", if consts::OS == "windows" { PASS } else { FAIL }, consts::OS);
        } else {
            println!("{} OS", if consts::OS == "windows" { PASS } else { FAIL });
        }
    }

    /// 检查cmd
    fn check_cmd(verbose: bool) {
        let paths = var("PATH").unwrap_or_else(|_| String::new());
        let directories = split_paths(&paths);

        for directory in directories {
            let cmd_path = directory.join("cmd.exe");
            if cmd_path.exists() {
                if verbose {
                    println!("{PASS} cmd.exe (found at {cmd_path:?})");
                } else {
                    println!("{PASS} cmd.exe");
                }
                return;
            }
        }

        if verbose {
            println!("{FAIL} cmd.exe (cmd.exe not found in environment variable 'PATH')");
        } else {
            println!("{FAIL} cmd.exe");
        }
    }

    /// 检查版本
    fn check_version(verbose: bool) {
        let version = env!("CARGO_PKG_VERSION");

        match DoctorImpl::get_latest_version() {
            Ok(latest) => {
                let symbol = if latest == version { PASS } else { WARN };
                if verbose {
                    println!("{symbol} version (latest: '{latest}', current: '{version}')");
                } else {
                    println!("{symbol} version");
                }
            }
            Err(err) => {
                if verbose {
                    println!("{FAIL} version ({err})");
                } else {
                    println!("{FAIL} version");
                }
            }
        }
    }

    /// 处理 Command::Doctor 子命令
    pub fn handle(verbose: bool) {
        println!("[Commands::Doctor] verbose: {verbose}");

        if !verbose {
            println!("Doctor summary (to see all details, run with -v/--verbose):");
        } else {
            println!("Doctor summary with all details:");
        }

        DoctorImpl::check_os(verbose);
        DoctorImpl::check_cmd(verbose);
        DoctorImpl::check_version(verbose);
    }
}

#[cfg(test)]
mod unit_test {
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::env;
    use super::*;
    use std::env::{consts, split_paths};
    use reqwest::blocking::get;

    #[test]
    fn check() {
        let paths = env::var("PATH").unwrap_or_else(|_| String::new());
        let directories = split_paths(&paths);

        for directory in directories {
            let cmd_path = directory.join("cmd.exe");
            if cmd_path.exists() {
                println!("Found cmd.exe at {:?}", cmd_path);
                break;
            }
        }
    }


    #[test]
    fn tt() {
        let client = reqwest::blocking::Client::builder()
            .user_agent("reqwest").build().unwrap();

        match client.get("https://api.github.com/repos/lopo12123/flow-chart/releases/latest").send() {
            Ok(res) => {
                let json: ReleaseInfo = res.json().expect("fail to deserialize");
                println!("res: {:?}", json);
            }
            Err(err) => {
                println!("err: {}", err);
            }
        }
    }
}