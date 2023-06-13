use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::get;

pub struct StateSingle {
    pub entry: PathBuf,
}

#[get("/<_path..>")]
pub async fn index_single(state: &rocket::State<StateSingle>, _path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(state.entry.clone()).await.ok()
}