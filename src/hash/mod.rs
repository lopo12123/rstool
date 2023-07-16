use sha2::Digest;

mod md5;
mod sha;

/// 通用hash函数
fn hash_bytes<Hasher: Digest>(bytes: &[u8]) -> String {
    let hex_bytes = Hasher::digest(bytes);
    let mut result = String::new();
    for byte in hex_bytes {
        result += &format!("{:x}", byte);
    }
    result
}

/// Supported digest algorithms:
/// - md5
/// - ripemd (Ripemd128; Ripemd160; Ripemd256; Ripemd320)
/// - sha1
/// - sha2 (SHA224; SHA256; SHA384; SHA512; SHA512_224; SHA512_256)
/// - sha3 (SHA3_224; SHA3_256; SHA3_384; SHA3_512)
pub struct HashImpl {}

impl HashImpl {
    /// 处理 Command::Hash 子命令
    pub fn handle(source: String, filemode: bool, algorithm: String) {
        println!("[Commands::Hash] source: '{source}', filemode: '{filemode}', algorithm: '{algorithm}'");

        // TODO
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn check() {
        let a = "";
        let b = "";
        println!("{}", a == b);
    }
}