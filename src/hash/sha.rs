use sha2::{Sha256, Digest};
use sha2::digest::{FixedOutput, Output};

pub fn sha256_str(source: String) {
    let mut hasher = Sha256::default();
    hasher.update(source.as_bytes());
    let result = hasher.finalize_fixed();
    println!("{:?}", result);
}

#[cfg(test)]
mod unit_test {
    use std::str::Chars;
    use super::*;

    #[test]
    fn tt() {
        sha256_str("hello world".to_string());
    }

    #[test]
    fn trans() {
        let bytes: Vec<u8> = vec![185, 77, 39, 185, 147, 77, 62, 8, 165, 46, 82, 215, 218, 125, 171, 250, 196, 132, 239, 227, 122, 83, 128, 238, 144, 136, 247, 172, 226, 239, 205, 233];

    }
}