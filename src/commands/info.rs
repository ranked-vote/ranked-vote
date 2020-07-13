use crate::model::metadata::ElectionCommission;
use crate::util::get_files_from_path;
use colored::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn info(meta_dir: &str) {
    let files = get_files_from_path(Path::new(meta_dir));

    for file in files.unwrap() {
        eprintln!("File: {}", file.to_string_lossy().blue());
        let file = File::open(file).unwrap();

        let reader = BufReader::new(file);
        let ec: ElectionCommission = serde_json::from_reader(reader).unwrap();

        eprintln!("Name: {}", ec.name.blue());
        eprintln!("Path: {}", ec.path.blue());
        eprintln!("Kind: {}", ec.kind.blue());

        for (key, election) in &ec.elections {
            eprintln!("Election: {}", key.blue());
            eprintln!("  Name: {}", election.name.blue());
            eprintln!("  Date: {}", election.date.blue());

            for (file, _) in &election.files {
                eprintln!("    File: {}", file.blue());
            }
        }
    }
}
