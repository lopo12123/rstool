use std::fs::File;
use std::io::Read;
use std::path::{Path};
use crate::archive::utils::ensured_path;

mod sevenz;
mod tgz;
mod zip;
mod utils;

// ==================== Pack ====================

type PackWorker = fn(buffer: Vec<u8>, dest: &Path) -> Result<(), String>;

pub struct PackImpl {}

impl PackImpl {
    pub fn pack() {}

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

#[cfg(test)]
mod unit_test {
    use std::path::PathBuf;
    use walkdir::WalkDir;
    use super::*;

    #[test]
    fn tttt() {
        let a = Some(1);

        println!("hello!");
    }

    #[test]
    fn walk() {
        // let source = r"D:\rstool\test\folder";
        //
        // let root = PathBuf::from(source);
        //
        // let walker = WalkDir::new(source);
        // for entry in walker {
        //     let entry = entry.unwrap();
        //     let path = entry.path();
        //     // entry.depth();
        //     let path_str = path.to_str().unwrap();
        //     let path_str = path_str.replace(source, "");
        //     println!("path: {:?}\nto_str: {:?}", &path, &path.to_str().unwrap());
        //
        //
        //     // let mut p = PathBuf::from(r"D:\rstool");
        //     // p.push(&path_str);
        //     // println!("after push: {}", p.to_str().unwrap());
        // }

        let source = ".";

        let walker = WalkDir::new(source);
        let last = walker.into_iter().last().unwrap().unwrap();
        println!("{:?}", last.path());
    }
}