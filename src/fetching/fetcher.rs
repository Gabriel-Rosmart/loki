use std::{fs, path::PathBuf};

pub struct Fetcher;

impl Fetcher {
    pub fn fetch_directory(path: &str) -> Vec<PathBuf> {
        fs::read_dir(path)
            .expect("Unable to read directory")
            .map(|file| file.unwrap().path())
            .collect()
    }
}
