use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};

fn walk_path(path: &Path, result: &mut Vec<PathBuf>) -> io::Result<()> {
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

pub fn get_files_from_path(path: &Path) -> io::Result<Vec<PathBuf>> {
    if path.exists() {
        let mut v = Vec::new();
        walk_path(path, &mut v)?;
        Ok(v)
    } else {
        panic!(format!("Path {} does not exist.", path.to_string_lossy()))
    }
}
