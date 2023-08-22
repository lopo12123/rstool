use std::fs;
use std::io::Cursor;
use std::path::Path;

pub fn extract_zip(zip_buffer: Vec<u8>, desc: &Path) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(Cursor::new(zip_buffer)).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => desc.join(path),
            None => continue,
        };

        // if the entry is a directory, create the directory
        if file.name().ends_with('/') {
            fs::create_dir_all(outpath).unwrap();
        }
        // if the entry is a file, extract it
        else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    Ok(())
}

pub fn archive_zip() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod unit_test {
    use std::fs::File;
    use super::*;

    #[test]
    fn pack() {
        let path = Path::new(r"D:\rstool\test\pack.zip");
        let file = File::create(path).unwrap();

        let mut zip = zip::ZipWriter::new(file);

        // zip.add_directory("根目录/", Default::default());
        zip.add_directory("根目录/子目录1", Default::default());
        zip.add_directory("根目录/子目录2", Default::default());

        zip.finish();
    }
}