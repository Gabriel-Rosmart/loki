use std::{
    ffi::OsStr,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

pub struct Fetcher;

impl Fetcher {
    const FORBIDDEN_EXTENSIONS: &'static [&'static str] = &[
        "png", "jpg", "jpeg", "gif", "bmp", "gz", "doc", "docx", "pdf", "odt", "xls", "xlsx",
        "ods", "ppt", "pptx", "7z",
    ];

    pub fn fetch_directory(path: &str) -> Vec<PathBuf> {
        WalkDir::new(path)
            .into_iter()
            .map(|file| file.unwrap())
            .filter(|file| Self::is_fetchable(file))
            .map(|file| file.into_path())
            .collect()
    }

    fn is_fetchable(entry: &DirEntry) -> bool {
        entry.file_type().is_file()
            && !(entry.metadata().unwrap().permissions().mode() & 0o111 != 0)
            && !entry.path_is_symlink()
            && !entry.path().has_extension(Self::FORBIDDEN_EXTENSIONS)
    }
}

pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(ref extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}
