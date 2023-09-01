use std::fs;
use std::path::{PathBuf};
use crate::archive::utils::{ArchiveBuilder, ArchiveEntry};

mod sevenz;
mod tgz;
mod zip;
mod utils;

// ==================== Pack ====================

type PackWorker = fn(entries: Vec<ArchiveEntry>) -> Vec<u8>;

pub struct PackImpl {}

impl PackImpl {
    pub fn pack(target: &str, entries: Vec<ArchiveEntry>) -> Result<Vec<u8>, String> {
        let pack_worker: Option<PackWorker> = match target {
            "zip" => Some(zip::pack),
            // TODO: 其他格式
            _ => None,
        };

        match pack_worker {
            Some(worker) => {
                Ok(worker(entries))
            }
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(root: PathBuf, destination: String, source: Vec<String>) {
        let target = destination.split(".").last().unwrap_or("").to_string();
        println!("[Commands::Pack] destination = '{destination}', source = '{source:?}'");

        match PackImpl::pack(&target, ArchiveBuilder::build(root, source).get_entries()) {
            Ok(buffer) => {
                match fs::write(destination, buffer) {
                    Ok(_) => println!("Ok"),
                    Err(err) => println!("Error: {err}"),
                }
            }
            Err(pack_err) => println!("Error: {pack_err}"),
        }
    }
}

// ==================== UnPack ====================
type UnpackWorker = fn(binary: Vec<u8>, disk_root: String) -> Vec<ArchiveEntry>;

pub struct UnpackImpl {}

impl UnpackImpl {
    fn write_to_disk(items: Vec<ArchiveEntry>) {
        for item in items {
            if item.is_file {
                fs::write(item.disk_dir, item.raw.unwrap()).unwrap();
            } else {
                fs::create_dir_all(item.disk_dir).unwrap();
            }
        }
    }

    fn unpack(suffix: &str, buffer: Vec<u8>, destination: String) -> Result<Vec<ArchiveEntry>, String> {
        let unpack_worker: Option<UnpackWorker> = match suffix {
            "zip" => Some(zip::unpack),
            // TODO: 其他格式
            _ => None,
        };

        match unpack_worker {
            Some(worker) => {
                Ok(worker(buffer, destination))
            }
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(source: String, destination: String) {
        let suffix = source.split(".").last().unwrap_or("").to_string();
        println!("[Commands::Unpack] source = '{source}', destination = '{destination}', suffix = '{suffix}'");

        match fs::read(source) {
            Ok(buffer) => match UnpackImpl::unpack(&suffix, buffer, destination) {
                Ok(items) => {
                    UnpackImpl::write_to_disk(items);
                    println!("Ok");
                }
                Err(unpack_err) => println!("Error: {unpack_err}"),
            }
            Err(read_err) => println!("Error: {read_err}"),
        }
    }
}

#[cfg(test)]
mod unit_test {
    use walkdir::WalkDir;

    #[test]
    fn walk() {
        let source = r"C:\Users\20366\Desktop\misc_test\zip";
        let walker = WalkDir::new(source);
        for entry in walker {
            let entry = entry.unwrap();
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            let path_str = path_str.replace(source, "");

            println!("path: {:?}\nto_str: {:?}\n", &path, path_str);
        }
    }
}