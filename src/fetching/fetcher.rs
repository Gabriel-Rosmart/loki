use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Fetcher;

impl Fetcher {
    pub fn fetch_directory(path: &str) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .map(|file| file.unwrap())
            .filter(|file| file.file_type().is_file())
            .map(|file| file.into_path())
            .collect()
    }
}
