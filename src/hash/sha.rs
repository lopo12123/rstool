use sha2::{Sha256, Digest, Sha512};
use crate::hash::utils::hash_bytes;

pub fn sha256_bytes(bytes: &[u8]) -> String {
    hash_bytes::<Sha256>(bytes)
}

pub fn sha256_str(source: String) -> String {
    sha256_bytes(source.as_bytes())
}

pub fn sha512_bytes(bytes: &[u8]) -> String {
    hash_bytes::<Sha512>(bytes)
}

pub fn sha512_str(source: String) -> String {
    sha512_bytes(source.as_bytes())
}

#[cfg(test)]
mod unit_test {
    use std::str::Chars;
    use super::*;

    #[test]
    fn tt() {
        // let r = sha256_str("hello world".to_string());
        let r = hash_bytes::<Sha256>(b"hello world");
        // let r = sha512_str("hello world".to_string());
        println!("r: {}", r);
    }
}