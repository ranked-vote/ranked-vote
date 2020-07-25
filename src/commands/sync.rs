use crate::read_metadata::read_meta;
use crate::util::{hash_file, write_serialized};
use colored::*;
use std::collections::HashSet;
use std::fs;
use std::fs::create_dir_all;
use std::path::Path;

pub fn sync(meta_dir: &str, raw_dir: &str) {
    let raw_path = Path::new(raw_dir);
    for (path, mut ec) in read_meta(meta_dir) {
        let ec_path = raw_path.join(ec.path.clone());
        if !ec_path.is_dir() {
            eprintln!(
                "Creating missing directory: {}",
                ec_path.to_string_lossy().red()
            );
            create_dir_all(ec_path.clone()).unwrap();
        }

        for (election_key, election) in ec.elections.iter_mut() {
            let election_path = ec_path.join(election_key);
            if !election_path.is_dir() {
                eprintln!(
                    "Creating missing directory: {}",
                    election_path.to_string_lossy().red()
                );
                create_dir_all(election_path.clone()).unwrap();
            }

            let mut expected_files: HashSet<String> =
                election.files.keys().map(|x| x.clone()).collect();

            for entry in fs::read_dir(election_path).unwrap() {
                let entry = entry.unwrap();
                let filename = String::from(entry.file_name().to_str().unwrap());
                if filename.starts_with(".") {
                    continue;
                };
                if !expected_files.remove(&filename) {
                    eprintln!(
                        "Found data file: {}",
                        entry.file_name().to_string_lossy().red()
                    );

                    let hash_str = hash_file(entry.path());
                    eprintln!("Hash: {}", hash_str.green());

                    election.files.insert(filename.into(), hash_str);
                }
            }

            for missing_file in expected_files {
                eprintln!("{}: missing file {}", "Warning".red(), missing_file.blue());
            }
        }

        write_serialized(&path, &ec);
    }
}
