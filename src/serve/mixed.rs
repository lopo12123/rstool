use std::path::PathBuf;
use rocket::fs::NamedFile;
use rocket::get;

pub struct StateMixed {
    pub root: PathBuf,
    pub entry: PathBuf,
}

#[get("/<path..>")]
pub async fn index_mixed(state: &rocket::State<StateMixed>, path: PathBuf) -> Option<NamedFile> {
    let target = state.root.clone().join(path);
    let target_exist = match target.try_exists() {
        Ok(r) => r,
        Err(_) => false
    };

    if target_exist && target.is_file() {
        NamedFile::open(target).await.ok()
    } else {
        NamedFile::open(state.entry.clone()).await.ok()
    }
}