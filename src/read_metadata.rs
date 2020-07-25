use crate::model::metadata::Jurisdiction;
use crate::util::io::read_serialized;
use crate::util::path::get_files_from_path;
use colored::*;
use std::path::{Path, PathBuf};

pub fn read_meta<'a>(path: &str) -> impl Iterator<Item = (PathBuf, Jurisdiction)> {
    let files = get_files_from_path(Path::new(path)).unwrap();

    files.into_iter().map(|file| {
        eprintln!("File: {}", file.to_string_lossy().blue());
        let ec = read_serialized(&file);
        (file, ec)
    })
}
