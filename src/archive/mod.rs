use std::fs;
use std::path::{PathBuf};
use crate::archive::utils::{ArchiveBuilder, ArchiveEntry};

mod sevenz;
mod tgz;
mod zip;
mod utils;

// ==================== Pack ====================

type PackWorker = fn(entries: Vec<ArchiveEntry>, filename: String) -> Vec<u8>;

pub struct PackImpl {}

impl PackImpl {
    pub fn pack(target: &PathBuf, entries: Vec<ArchiveEntry>) -> Result<Vec<u8>, String> {
        let suffix = target.extension().map_or("", |ext| ext.to_str().unwrap_or(""));
        if suffix == "" {
            return Err(format!("Fail to parse suffix"));
        }

        let filename = target.file_name().map_or("", |name| name.to_str().unwrap_or(""));
        if filename == "" {
            return Err(format!("Fail to parse filename"));
        }

        let pack_worker: Option<PackWorker> = match suffix {
            "zip" => Some(zip::pack),
            "7z" => Some(sevenz::pack),
            "gz" => Some(tgz::pack_gz),
            "tar" => Some(tgz::pack_tar),
            // TODO: 其他格式
            _ => None,
        };

        match pack_worker {
            Some(worker) => Ok(worker(entries, filename.to_string())),
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(root: PathBuf, destination: String, source: Vec<String>) {
        println!("[Commands::Pack] destination = '{destination}', source = '{source:?}'");

        let target = PathBuf::from(destination);
        match PackImpl::pack(&target, ArchiveBuilder::build(root, source).get_entries()) {
            Ok(buffer) => {
                match fs::write(target, buffer) {
                    Ok(_) => println!("Ok"),
                    Err(err) => println!("Error: {err}"),
                }
            }
            Err(pack_err) => println!("Error: {pack_err}"),
        }
    }
}

// ==================== UnPack ====================
type UnpackWorker = fn(binary: Vec<u8>, source_stem: String, disk_root: String) -> Vec<ArchiveEntry>;

pub struct UnpackImpl {}

impl UnpackImpl {
    fn write_to_disk(items: Vec<ArchiveEntry>, destination: String) {
        // ensure destination exists
        fs::create_dir_all(&destination).unwrap();

        for item in items {
            if item.is_file {
                fs::write(item.disk_dir, item.raw.unwrap()).unwrap();
            } else if item.is_dir {
                fs::create_dir_all(item.disk_dir).unwrap();
            }
        }
    }

    fn unpack(buffer: Vec<u8>, source_path: String, destination: String) -> Result<Vec<ArchiveEntry>, String> {
        let parsed_path = PathBuf::from(source_path);
        let stem = parsed_path.file_stem().unwrap().to_str().unwrap();
        let suffix = parsed_path.extension().unwrap().to_str().unwrap();

        let unpack_worker: Option<UnpackWorker> = match suffix {
            "zip" => Some(zip::unpack),
            "7z" => Some(sevenz::unpack),
            "gz" => Some(tgz::unpack_gz),
            "tar" => Some(tgz::unpack_tar),
            // TODO: 其他格式
            _ => None,
        };

        match unpack_worker {
            Some(worker) => Ok(worker(buffer, stem.to_string(), destination)),
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(source: String, destination: String) {
        let suffix = source.split(".").last().unwrap_or("").to_string();
        println!("[Commands::Unpack] source = '{source}', destination = '{destination}', suffix = '{suffix}'");

        match fs::read(&source) {
            Ok(buffer) => match UnpackImpl::unpack(buffer, source, destination.clone()) {
                Ok(items) => {
                    UnpackImpl::write_to_disk(items, destination);
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
    use std::path::PathBuf;
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

    #[test]
    fn filename_test() {
        let source_name = "a/b/c/filename.tar.gz".to_string();

        let parse = PathBuf::from(source_name);

        println!("filename: {:?}", parse.file_name().unwrap());
        println!("extension: {:?}", parse.extension().unwrap());
        println!("extension: {:?}", parse.file_stem().unwrap());
    }
}