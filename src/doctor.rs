use std::env::{consts, split_paths, var};

const PASS: &str = "[√]";
const FAIL: &str = "[×]";

pub struct DoctorImpl {}

impl DoctorImpl {
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
    }
}

#[cfg(test)]
mod unit_test {
    use std::env;
    use super::*;
    use std::env::{consts, split_paths};

    #[test]
    fn check() {
        // let paths = env::var("PATH").unwrap_or_else(|_| String::new());
        let paths = env!("PATH");
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
        println!("{PASS}");
    }
}