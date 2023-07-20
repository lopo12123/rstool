use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

mod rar;
mod sevenz;
mod tgz;
mod zip;

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
        println!("[Commands::Serve] source: '{source}' target: '{target}', format: '{format:?}'");

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