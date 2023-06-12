use std::env::current_dir;
use std::path::{Path, PathBuf};
use ipconfig::get_adapters;
use rocket::{Build, Config, get, launch, Rocket, routes};
use rocket::fs::NamedFile;
use rocket::http::ext::Normalize;
use rocket::tokio::runtime::Runtime;

#[derive(Debug)]
enum ServerMode { Single, Mixed, Direct }

#[derive(Debug)]
struct ServerBuilder {
    root: PathBuf,
    entry: PathBuf,
    port: u16,
    mode: ServerMode,
}

impl ServerBuilder {
    // region 参数解析
    /// root: 服务根目录
    fn get_root(root: &str) -> Result<PathBuf, String> {
        let path_root = PathBuf::from(root);

        if !path_root.exists() {
            Err(format!("invalid root (root does not exist)"))
        } else if !path_root.is_dir() {
            Err(format!("invalid root (root is not a directory)"))
        } else {
            if path_root.is_absolute() {
                Ok(path_root.canonicalize().unwrap())
            } else {
                match current_dir() {
                    Ok(curr) => Ok(curr.join(path_root).canonicalize().unwrap()),
                    Err(_) => Err(format!("unable to resolve the absolute path of the current location"))
                }
            }
        }
    }

    /// entry: 入口文件
    fn get_entry(root: PathBuf, entry: &str) -> Result<PathBuf, String> {
        let path_entry = root.join(entry);

        if !path_entry.exists() {
            Err(format!("invalid entry (entry does not exist)"))
        } else if !path_entry.is_file() {
            Err(format!("invalid entry (entry is not a file)"))
        } else {
            if path_entry.is_absolute() {
                Ok(path_entry.canonicalize().unwrap())
            } else {
                match current_dir() {
                    Ok(curr) => Ok(curr.join(path_entry).canonicalize().unwrap()),
                    Err(_) => Err(format!("unable to resolve the absolute path of the entry file"))
                }
            }
        }
    }

    /// port: 1024 ~ 65535
    fn get_port(port: u16) -> Result<u16, String> {
        if port < 1024 {
            Err(format!("invalid port (required: 1024 ~ 65535, received: {port})"))
        } else {
            Ok(port)
        }
    }

    /// mode: 'single' | 'mixed' | 'direct'
    fn get_mode(mode: &str) -> Result<ServerMode, String> {
        match mode {
            "single" => Ok(ServerMode::Single),
            "mixed" => Ok(ServerMode::Mixed),
            "direct" => Ok(ServerMode::Direct),
            _ => Err(format!("invalid mode (required: 'single' / 'mixed' / 'direct', received: {mode})"))
        }
    }
    // endregion

    /// 解析并校验参数, 构建服务器实例
    pub fn try_build(root: String, entry: String, port: u16, mode: String) -> Result<Self, String> {
        let _root = ServerBuilder::get_root(&root)?;
        println!("[try_build] root: {:?}", _root.normalized_str());
        let _entry = ServerBuilder::get_entry(_root.clone(), &entry)?;
        println!("[try_build] entry: {:?}", _entry.normalized_str());
        let _port = ServerBuilder::get_port(port)?;
        println!("[try_build] port: {port}");
        let _mode = ServerBuilder::get_mode(&mode)?;
        println!("[try_build] mode: {_mode:?}");

        Ok(ServerBuilder {
            root: _root,
            entry: _entry,
            port: _port,
            mode: _mode,
        })
    }
}

#[get("/")]
fn index_single(my_str: &rocket::State<String>) -> String {
    my_str.to_string()
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

    // region 三种启动模式
    /// 单一模式
    // #[launch]
    async fn single() {
        let my_str = "自定义参数";
        rocket::build()
            .manage(my_str.to_string())
            .mount("/", routes![index_single])
            .launch().await.ok();
    }
    /// 混合模式
    fn mixed() {}
    /// 直接模式
    fn direct() {}
    // endregion

    /// 处理 Command::Serve 子命令
    pub fn handle(root: String, entry: String, port: u16, mode: String) {
        println!("[Commands::Serve] root: '{root}' entry: '{entry}', port: '{port}', mode: '{mode}'");

        match ServerBuilder::try_build(root, entry, port, mode) {
            Ok(server) => {
                for ip in ServeImpl::get_ips() {
                    println!("[serve_at] http://{ip}:{port}/");
                }

                match server.mode {
                    ServerMode::Single => {
                        Runtime::new()
                            .unwrap()
                            .block_on(ServeImpl::single());
                    }
                    ServerMode::Mixed => ServeImpl::mixed(),
                    ServerMode::Direct => ServeImpl::direct(),
                }
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
    use rocket::http::ext::Normalize;

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
        let p = "D:\\rstool\\target\\debug\\../../examples";
        println!("{:?}", Path::new(p).canonicalize().unwrap());
    }
}