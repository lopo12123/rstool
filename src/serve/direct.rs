use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::get;

pub struct StateDirect {
    pub root: PathBuf,
    pub entry: PathBuf,
}

#[get("/<path..>")]
pub async fn index_direct(state: &rocket::State<StateDirect>, path: PathBuf) -> Option<NamedFile> {
    if path == PathBuf::new() {
        NamedFile::open(state.entry.clone()).await.ok()
    } else {
        NamedFile::open(state.root.clone().join(path)).await.ok()
    }
}