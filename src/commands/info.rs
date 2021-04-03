use crate::read_metadata::read_meta;
use colored::*;

pub fn info(meta_dir: &str) {
    for (_, ec) in read_meta(meta_dir) {
        eprintln!("Name: {}", ec.name.blue());
        eprintln!("Path: {}", ec.path.blue());
        eprintln!("Kind: {}", ec.kind.blue());

        for (key, election) in &ec.elections {
            eprintln!("Election: {}", key.blue());
            eprintln!("  Name: {}", election.name.blue());
            eprintln!("  Date: {}", election.date.blue());

            for file in election.files.keys() {
                eprintln!("    File: {}", file.blue());
            }
        }
    }
}
