mod direct;
mod single;
mod mixed;

use std::env::current_dir;
use std::net::{IpAddr, Ipv4Addr};
use std::path::{PathBuf};
use ipconfig::get_adapters;
use rocket::{Config, catch, routes, Request, catchers};
use rocket::http::ext::Normalize;
use rocket::tokio::runtime::Runtime;
use crate::serve::direct::{index_direct, StateDirect};
use crate::serve::mixed::{index_mixed, StateMixed};
use crate::serve::single::{index_single, StateSingle};

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
            Err(format!("Invalid root (root does not exist)"))
        } else if !path_root.is_dir() {
            Err(format!("Invalid root (root is not a directory)"))
        } else {
            if path_root.is_absolute() {
                Ok(path_root.canonicalize().unwrap())
            } else {
                match current_dir() {
                    Ok(curr) => Ok(curr.join(path_root).canonicalize().unwrap()),
                    Err(_) => Err(format!("Unable to resolve the absolute path of the current location"))
                }
            }
        }
    }

    /// entry: 入口文件
    fn get_entry(root: PathBuf, entry: &str) -> Result<PathBuf, String> {
        let path_entry = root.join(entry);

        if !path_entry.exists() {
            Err(format!("Invalid entry (entry does not exist)"))
        } else if !path_entry.is_file() {
            Err(format!("Invalid entry (entry is not a file)"))
        } else {
            if path_entry.is_absolute() {
                Ok(path_entry.canonicalize().unwrap())
            } else {
                match current_dir() {
                    Ok(curr) => Ok(curr.join(path_entry).canonicalize().unwrap()),
                    Err(_) => Err(format!("Unable to resolve the absolute path of the entry file"))
                }
            }
        }
    }

    /// mode: 'single' | 'mixed' | 'direct'
    fn get_mode(mode: &str) -> Result<ServerMode, String> {
        match mode {
            "single" => Ok(ServerMode::Single),
            "mixed" => Ok(ServerMode::Mixed),
            "direct" => Ok(ServerMode::Direct),
            _ => Err(format!("Invalid mode (required: 'single' / 'mixed' / 'direct', received: {mode})"))
        }
    }
    // endregion

    /// 解析并校验参数, 构建服务器实例
    pub fn try_build(root: String, entry: String, port: u16, mode: String) -> Result<Self, String> {
        let _root = ServerBuilder::get_root(&root)?;
        println!("[try_build] root: {:?}", _root.normalized_str());
        let _entry = ServerBuilder::get_entry(_root.clone(), &entry)?;
        println!("[try_build] entry: {:?}", _entry.normalized_str());
        // let _port = ServerBuilder::get_port(port)?;
        println!("[try_build] port: {port}");
        let _mode = ServerBuilder::get_mode(&mode)?;
        println!("[try_build] mode: {_mode:?}");

        Ok(ServerBuilder {
            root: _root,
            entry: _entry,
            port,
            mode: _mode,
        })
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
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
    async fn single(config: Config, entry: PathBuf) {
        rocket::build()
            .manage(StateSingle { entry })
            .configure(config)
            .attach(rocket::fairing::AdHoc::on_response("CORS", |_, res| Box::pin(async move {
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type"));
                // remove this header to allow iframe
                res.remove_header("X-Frame-Options")
            })))
            .mount("/", routes![index_single])
            .register("/", catchers![not_found, internal_error])
            .launch().await.ok();
    }
    /// 混合模式
    async fn mixed(config: Config, root: PathBuf, entry: PathBuf) {
        rocket::build()
            .manage(StateMixed { root, entry })
            .configure(config)
            .attach(rocket::fairing::AdHoc::on_response("CORS", |_, res| Box::pin(async move {
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type"));
                // remove this header to allow iframe
                res.remove_header("X-Frame-Options")
            })))
            .mount("/", routes![index_mixed])
            .register("/", catchers![not_found, internal_error])
            .launch().await.ok();
    }
    /// 直接模式
    async fn direct(config: Config, root: PathBuf, entry: PathBuf) {
        rocket::build()
            .manage(StateDirect { root, entry })
            .configure(config)
            .attach(rocket::fairing::AdHoc::on_response("CORS", |_, res| Box::pin(async move {
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "*"));
                res.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "Content-Type"));
                // remove this header to allow iframe
                res.remove_header("X-Frame-Options")
            })))
            .mount("/", routes![index_direct])
            .register("/", catchers![not_found, internal_error])
            .launch().await.ok();
    }
    // endregion

    /// 处理 Command::Serve 子命令
    pub fn handle(root: String, entry: String, port: u16, mode: String) {
        println!("[Commands::Serve] root = '{root}', entry = '{entry}', port = '{port}', mode = '{mode}'");

        match ServerBuilder::try_build(root, entry, port, mode) {
            Ok(server) => {
                for ip in ServeImpl::get_ips() {
                    println!("[local_ip] http://{ip}:{port}/");
                }

                let config = Config {
                    port: server.port,
                    address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                    ..Config::debug_default()
                    // ..Config::release_default()
                };

                match server.mode {
                    ServerMode::Single => {
                        Runtime::new()
                            .unwrap()
                            .block_on(ServeImpl::single(config, server.entry));
                    }
                    ServerMode::Mixed => {
                        Runtime::new()
                            .unwrap()
                            .block_on(ServeImpl::mixed(config, server.root, server.entry));
                    }
                    ServerMode::Direct => {
                        Runtime::new()
                            .unwrap()
                            .block_on(ServeImpl::direct(config, server.root, server.entry));
                    }
                }
            }
            Err(err) => println!("Error: {err}"),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use ipconfig::{get_adapters};

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
        // let p = "D:\\rstool\\target\\debug\\../../examples";
        // println!("{:?}", Path::new(p).canonicalize().unwrap());

        let p = PathBuf::from("C:\\Windows\\System32");
        println!("p: {p:?}; {}", p.exists());
    }
}