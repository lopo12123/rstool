use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod sevenz;
mod tgz;
mod zip;

// ==================== Utils ====================
fn ensured_path(target: String) -> Result<PathBuf, String> {
    let path = PathBuf::from(target);
    if !path.exists() {
        match fs::create_dir_all(&path) {
            Ok(_) => Ok(path),
            Err(err) => Err(format!("{err}"))
        }
    } else {
        Ok(path)
    }
}

// ==================== Pack ====================

type PackWorker = fn(buffer: Vec<u8>, dest: &Path) -> Result<(), String>;

pub struct PackImpl {}

impl PackImpl {
    pub fn handle(destination: String, source: Vec<String>) {
        println!("[Commands::Pack] destination = '{destination}', source = '{source:?}'");
    }
}

// ==================== UnPack ====================

type UnpackWorker = fn(buffer: Vec<u8>, dest: &Path) -> Result<(), String>;

pub struct UnpackImpl {}

impl UnpackImpl {
    fn unpack(suffix: &str, buffer: Vec<u8>, destination: String) -> Result<(), String> {
        let try_worker: Option<UnpackWorker> = match suffix {
            "7z" => Some(sevenz::extract_sevenz),
            "gz" => Some(tgz::extract_gz),
            "tar" => Some(tgz::extract_tar),
            "tgz" | "tar.gz" => Some(tgz::extract_tgz),
            "zip" => Some(zip::extract_zip),
            _ => None,
        };

        match try_worker {
            Some(worker) => match ensured_path(destination) {
                Ok(dest) => worker(buffer, &dest),
                Err(err) => Err(err),
            }
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(source: String, destination: String) {
        let suffix = source.split(".").last().unwrap_or("").to_string();
        println!("[Commands::Unpack] source = '{source}', destination = '{destination}', suffix = '{suffix}'");

        match File::open(source) {
            Ok(mut file) => {
                let mut bytes = vec![];
                match file.read_to_end(&mut bytes) {
                    Ok(_) => match UnpackImpl::unpack(&suffix, bytes, destination) {
                        Ok(_) => println!("Ok"),
                        Err(unpack_err) => println!("Error: {unpack_err}"),
                    }
                    Err(read_err) => println!("Error: {read_err}"),
                }
            }
            Err(open_err) => println!("Error: {open_err}"),
        }
    }
}

// ==================== Deprecated ====================

type Extractor = fn(buffer: Vec<u8>, dest: &Path) -> Result<(), String>;

pub struct ExtractImpl {}

impl ExtractImpl {
    fn ensured_path(target: String) -> Result<PathBuf, String> {
        let path = PathBuf::from(target);
        if !path.exists() {
            match fs::create_dir_all(&path) {
                Ok(_) => Ok(path),
                Err(err) => Err(format!("{err}"))
            }
        } else {
            Ok(path)
        }
    }

    fn extract(format: &str, buffer: Vec<u8>, destination: String) -> Result<(), String> {
        let extractor: Option<Extractor> = match format {
            "7z" => Some(sevenz::extract_sevenz),
            "gz" => Some(tgz::extract_gz),
            // "rar" => Some(rar::extract_rar),
            "tar" => Some(tgz::extract_tar),
            "tgz" | "tar.gz" => Some(tgz::extract_tgz),
            "zip" => Some(zip::extract_zip),
            _ => None,
        };

        match extractor {
            Some(worker) => match ExtractImpl::ensured_path(destination) {
                Ok(dest) => worker(buffer, &dest),
                Err(err) => Err(err),
            }
            None => Err(format!("Invalid format"))
        }
    }

    pub fn handle(source: String, target: String, format: Option<String>) {
        let format = format.unwrap_or(source.split(".").last().unwrap_or("").to_string());
        println!("[Commands::Serve] source = '{source}', target = '{target}', format = '{format}'");

        match File::open(source) {
            Ok(mut file) => {
                let mut bytes = vec![];
                match file.read_to_end(&mut bytes) {
                    Ok(_) => match ExtractImpl::extract(&format, bytes, target) {
                        Ok(_) => println!("Ok"),
                        Err(extract_err) => println!("Error: {extract_err}"),
                    }
                    Err(read_err) => println!("Error: {read_err}"),
                };
            }
            Err(open_err) => println!("Error: {open_err}"),
        }
    }
}