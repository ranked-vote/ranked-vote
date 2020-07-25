use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

/// Crawl a directory tree, appending non-hidden files encountered to
/// a passed mutable `result` vector.
fn walk_path(path: &Path, result: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
        // Don't recurse into private directories.
        return Ok(());
    }
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            walk_path(&entry.path(), result)?;
        }
    } else {
        result.push(PathBuf::from(path))
    }
    Ok(())
}

/// Crawl a directory tree and return a flat vector of the non-hidden
/// files in it (wrapped in an IO result).
pub fn get_files_from_path(path: &Path) -> io::Result<Vec<PathBuf>> {
    if path.exists() {
        let mut v = Vec::new();
        walk_path(path, &mut v)?;
        Ok(v)
    } else {
        panic!(format!("Path {} does not exist.", path.to_string_lossy()))
    }
}
