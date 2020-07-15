use crate::formats::read_election;
use crate::model::election::{ElectionInfo, ElectionPreprocessed};
use crate::normalizers::normalize_election;
use crate::read_metadata::read_meta;
use crate::report::{generate_report, preprocess_election};
use crate::util::io::write_serialized;
use colored::*;
use std::fs::create_dir_all;
use std::path::Path;

pub fn report(meta_dir: &str, raw_dir: &str, report_dir: &str) {
    let raw_path = Path::new(raw_dir);
    for (_, ec) in read_meta(meta_dir) {
        let raw_base = raw_path.join(ec.path.clone());
        let report_path = Path::new(report_dir);

        for (election_path, election) in &ec.elections {
            eprintln!("Election: {}", election_path.red());
            for contest in &election.contests {
                let office = ec.offices.get(&contest.office).unwrap();
                eprintln!("Office: {}", office.name.red());
                // Figure out if we need to generate report at all.

                // Figure out if we need to preprocess.
                let output_base = report_path
                    .join(&ec.path)
                    .join(&election_path)
                    .join(&contest.office);
                create_dir_all(&output_base).unwrap();
                let preprocessed_path = output_base.join("normalized.json.gz");

                let preprocessed =
                    preprocess_election(&raw_base, election, election_path, &ec, contest);

                let report_path = output_base.join("report.json");

                write_serialized(&preprocessed_path, &preprocessed);

                let contest_report = generate_report(&preprocessed);

                write_serialized(&report_path, &contest_report);
            }
        }
    }
}
