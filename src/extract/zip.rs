use std::io::Cursor;
use std::path::{Path};

pub fn extract_zip(zip_buffer: Vec<u8>, dest: &Path) -> Result<(), String> {
    match zip_extract::extract(
        Cursor::new(zip_buffer),
        dest,
        false,
    ) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("{}", err))
    }
}

#[cfg(test)]
mod unit_test {
    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;
    use crate::extract::zip::extract_zip;

    #[test]
    fn unpack_zip() {
        let mut buffer = vec![];
        let mut file = File::open("C:\\Users\\20366\\Downloads\\阿比盖尔.zip").unwrap();
        file.read_to_end(&mut buffer).unwrap();
        match extract_zip(buffer, &PathBuf::from("D:\\rstool\\test")) {
            Ok(_) => {
                println!("done");
            }
            Err(err) => {
                println!("fail! {}", err);
            }
        };
    }
}