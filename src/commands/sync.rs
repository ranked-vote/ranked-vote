use crate::model::metadata::ElectionCommission;
use crate::util::get_files_from_path;
use colored::*;
use sha1::{Digest, Sha1};
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::fs::{create_dir_all, File};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

pub fn sync(meta_dir: &str, raw_dir: &str) {
    let raw_path = Path::new(raw_dir);
    let files = get_files_from_path(Path::new(meta_dir));

    for path in files.unwrap() {
        let mut ec: ElectionCommission = {
            let file = File::open(path.clone()).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

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
                if !expected_files.remove(&filename) {
                    eprintln!(
                        "Found data file: {}",
                        entry.file_name().to_string_lossy().red()
                    );

                    let mut file = File::open(&entry.path()).unwrap();
                    let mut hasher = Sha1::new();
                    io::copy(&mut file, &mut hasher).unwrap();
                    let hash = hasher.finalize();
                    let hash_str = format!("{:x}", hash);
                    eprintln!("Hash: {}", hash_str.green());

                    election.files.insert(filename.into(), hash_str);
                }
            }

            for missing_file in expected_files {
                eprintln!("{}: missing file {}", "Warning".red(), missing_file.blue());
            }
        }

        {
            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap();
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &ec).unwrap();
        }
    }
}
