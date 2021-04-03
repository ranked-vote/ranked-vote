use crate::model::metadata::Jurisdiction;
use crate::util::{get_files_from_path, read_serialized};
use colored::*;
use std::path::{Path, PathBuf};

/// Read all metadata files under the given directory (recursively) and return
/// an iterator over the results.
pub fn read_meta(path: &str) -> impl Iterator<Item = (PathBuf, Jurisdiction)> {
    let files = get_files_from_path(Path::new(path)).unwrap();

    files.into_iter().map(|file| {
        eprintln!("File: {}", file.to_string_lossy().blue());
        let ec = read_serialized(&file);
        (file, ec)
    })
}
