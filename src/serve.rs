use std::env::current_dir;
use std::fmt::format;
use std::fs;
use std::path::{Path, PathBuf};
use ipconfig::get_adapters;
use rocket::Config;

enum ServerMode { Single, Mixed, Direct }

struct ServerBuilder {
    root: PathBuf,
    entry: PathBuf,
    port: u16,
    mode: ServerMode,
}

impl ServerBuilder {
    /// root: 存在的目录
    fn check_root(root: &str) -> bool {
        match fs::metadata(root) {
            Ok(meta) => meta.is_dir(),
            Err(_) => false
        }
    }
    /// root: 存在的目录
    fn get_root(root: &str) -> Result<PathBuf, String> {
        let path_root = PathBuf::from(root);

        if !path_root.exists() {
            Err(format!("invalid root (root does not exist)"))
        } else if !path_root.is_dir() {
            Err(format!("invalid root (root is not a directory)"))
        } else {
            if path_root.is_absolute() {
                Ok(path_root)
            } else {
                match current_dir() {
                    Ok(curr) => Ok(curr.join(path_root)),
                    Err(_) => Err(format!("unable to resolve the absolute path of the current location"))
                }
            }
        }
    }
    /// entry: 存在的文件
    fn check_entry(entry: &str) -> bool {
        match fs::metadata(entry) {
            Ok(meta) => meta.is_file(),
            Err(_) => false
        }
    }
    /// port: 1024 ~ 65535
    fn check_port(port: u16) -> bool {
        port >= 1024
    }

    /// mode: 'single' | 'mixed' | 'direct'
    fn get_mode(mode: &str) -> Option<ServerMode> {
        match mode {
            "single" => Some(ServerMode::Single),
            "mixed" => Some(ServerMode::Mixed),
            "direct" => Some(ServerMode::Direct),
            _ => None
        }
    }


    /// 解析并校验参数, 构建服务器实例
    pub fn try_build(root: String, entry: String, port: u16, mode: String) -> Result<Self, String> {
        if !ServerBuilder::check_root(&root) {
            Err(format!("invalid root (root does not exist or is not a directory)"))
        } else if !ServerBuilder::check_entry(&entry) {
            Err(format!("invalid entry (entry does not exist or is not a file)"))
        } else if !ServerBuilder::check_port(port) {
            Err(format!("invalid port (required: 1024 ~ 65535, received: {port})"))
        } else {
            match ServerBuilder::get_mode(&mode.to_lowercase()) {
                Some(m) => {
                    // todo
                    Err(format!("invalid port (required: 1024 ~ 65535, received: {port})"))
                }
                None => Err(format!("invalid mode (required: 'single' / 'mixed' / 'direct', received: {port})"))
            }
        }
    }

    pub fn start(&self) {
        // todo
    }
}

pub struct ServeImpl {}

impl ServeImpl {
    /// 获取本机的所有局域网ip地址
    fn get_ips() -> Vec<String> {
        let mut ips = vec![format!("localhost"), format!("127.0.0.1")];
        for adapter in get_adapters().unwrap() {
            for ip_address in adapter.ip_addresses() {
                if ip_address.is_ipv4() {
                    let ip_str = ip_address.to_string();
                    if ip_str.starts_with("192.") || ip_str.starts_with("172.") || ip_str.starts_with("10.") {
                        ips.push(ip_str);
                    }
                }
            }
        }
        ips
    }

    /// 处理 Command::Serve 子命令
    pub fn handle(root: String, entry: String, port: u16, mode: String) {
        println!("[Commands::Serve] root: '{root}' entry: '{entry}', port: '{port}', mode: '{mode}'");

        match ServerBuilder::try_build(root, entry, port, mode) {
            Ok(server) => {
                for ip in ServeImpl::get_ips() {
                    println!("http://{ip}:{port}");
                }

                server.start();
            }
            Err(err) => println!("Error: {err}"),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use std::path::Path;
    use super::*;
    use ipconfig::{get_adapters, Adapter};

    #[test]
    fn ips() {
        let mut ips = vec![format!("localhost"), format!("127.0.0.1")];
        for adapter in get_adapters().unwrap() {
            for ip_address in adapter.ip_addresses() {
                if ip_address.is_ipv4() {
                    let ip_str = ip_address.to_string();
                    if ip_str.starts_with("192.") || ip_str.starts_with("172.") || ip_str.starts_with("10.") {
                        ips.push(ip_str);
                    }
                }
            }
        }
        println!("ips: {:?}", ips);
    }

    #[test]
    fn ppp() {
        let p = ".";
        println!("{}, {}", Path::new("D:/").is_absolute(), Path::new("H:/").is_absolute());
    }
}