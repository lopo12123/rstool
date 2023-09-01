use std::fs;
use std::io::Read;
use md5::Md5;
use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};

/// 通用hash函数
fn hash_bytes<Hasher: Digest>(bytes: &[u8]) -> String {
    let hex_bytes = Hasher::digest(bytes);
    let mut result = String::new();
    for byte in hex_bytes {
        result += &format!("{:x}", byte);
    }
    result
}

fn algorithm_to_hasher(algorithm: &str) -> Option<Box<fn(&[u8]) -> String>> {
    match algorithm.to_lowercase().as_str() {
        "md5" => Some(Box::new(hash_bytes::<Md5>)),
        "ripemd128" => Some(Box::new(hash_bytes::<Ripemd128>)),
        "ripemd160" => Some(Box::new(hash_bytes::<Ripemd160>)),
        "ripemd256" => Some(Box::new(hash_bytes::<Ripemd256>)),
        "ripemd320" => Some(Box::new(hash_bytes::<Ripemd320>)),
        "sha1" => Some(Box::new(hash_bytes::<Sha1>)),
        "sha224" => Some(Box::new(hash_bytes::<Sha224>)),
        "sha256" => Some(Box::new(hash_bytes::<Sha256>)),
        "sha384" => Some(Box::new(hash_bytes::<Sha384>)),
        "sha512" => Some(Box::new(hash_bytes::<Sha512>)),
        "sha512_224" => Some(Box::new(hash_bytes::<Sha512_224>)),
        "sha512_256" => Some(Box::new(hash_bytes::<Sha512_256>)),
        "sha3_224" => Some(Box::new(hash_bytes::<Sha3_224>)),
        "sha3_256" => Some(Box::new(hash_bytes::<Sha3_256>)),
        "sha3_384" => Some(Box::new(hash_bytes::<Sha3_384>)),
        "sha3_512" => Some(Box::new(hash_bytes::<Sha3_512>)),
        _ => None
    }
}

/// Supported algorithms (case insensitive):
/// - md5 (MD5) -- default
/// - ripemd (Ripemd128; Ripemd160; Ripemd256; Ripemd320)
/// - sha1 (SHA1)
/// - sha2 (SHA224; SHA256; SHA384; SHA512; SHA512_224; SHA512_256)
/// - sha3 (SHA3_224; SHA3_256; SHA3_384; SHA3_512)
pub struct HashImpl {}

impl HashImpl {
    /// 使用指定算法计算目标的摘要
    fn hash(algorithm: String, bytes: &[u8]) -> Result<String, String> {
        match algorithm_to_hasher(&algorithm) {
            Some(hasher) => Ok(hasher(bytes)),
            None => Err(format!("Invalid algorithm '{algorithm}'")),
        }
    }

    /// 处理 Command::Hash 子命令
    pub fn handle(source: String, filemode: bool, algorithm: String) {
        println!("[Commands::Hash] source = '{source}', filemode = '{filemode}', algorithm = '{algorithm}'");

        let source_bytes: Result<Vec<u8>, String> = if !filemode { Ok(source.into_bytes()) } else {
            fs::read(source).map_err(|err| format!("{err}"))
        };

        match source_bytes {
            Ok(bytes) => match HashImpl::hash(algorithm, &bytes) {
                Ok(result) => println!("Ok: {result}"),
                Err(err) => println!("Error: {err}"),
            },
            Err(err) => println!("Error: {err}"),
        }
    }
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn hash_test() {
        // HashImpl::handle("hello world".to_string(), false, "md5".to_string());
        // HashImpl::handle("hello world".to_string(), false, "sha256".to_string());
        // HashImpl::handle("hello world".to_string(), false, "sha512".to_string());
        // HashImpl::handle("hello world".to_string(), false, "sha".to_string());

        // HashImpl::handle("./aho_corasick-6ed754f677c9af28.d".to_string(), true, "sha256".to_string());
    }
}