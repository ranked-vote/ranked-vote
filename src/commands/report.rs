use crate::model::election::ElectionPreprocessed;
use crate::read_metadata::read_meta;
use crate::report::{generate_report, preprocess_election};
use crate::util::io::{read_serialized, write_serialized};
use colored::*;
use std::fs::create_dir_all;
use std::path::Path;

pub fn report(
    meta_dir: &str,
    raw_dir: &str,
    report_dir: &str,
    force_preprocess: bool,
    force_report: bool,
) {
    let raw_path = Path::new(raw_dir);
    for (_, ec) in read_meta(meta_dir) {
        let raw_base = raw_path.join(ec.path.clone());
        let report_path = Path::new(report_dir);

        for (election_path, election) in &ec.elections {
            eprintln!("Election: {}", election_path.red());
            for contest in &election.contests {
                let office = ec.offices.get(&contest.office).expect(&format!(
                    "Expected office {} to be in offices.",
                    &contest.office
                ));
                eprintln!("Office: {}", office.name.red());

                let output_base = report_path
                    .join(&ec.path)
                    .join(&election_path)
                    .join(&contest.office);

                let report_path = output_base.join("report.json");
                if report_path.exists() && !force_report && !force_preprocess {
                    eprintln!(
                        "Skipping because {} exists.",
                        report_path.to_str().unwrap().bright_cyan()
                    );
                    continue;
                }

                create_dir_all(&output_base).unwrap();
                let preprocessed_path = output_base.join("normalized.json.gz");

                let preprocessed: ElectionPreprocessed =
                    if preprocessed_path.exists() && !force_preprocess {
                        eprintln!(
                            "Loading preprocessed {}.",
                            preprocessed_path.to_str().unwrap().bright_cyan()
                        );
                        read_serialized(&preprocessed_path)
                    } else {
                        eprintln!(
                            "Generating preprocessed {}.",
                            preprocessed_path.to_str().unwrap().bright_cyan()
                        );
                        let preprocessed =
                            preprocess_election(&raw_base, election, election_path, &ec, contest);
                        write_serialized(&preprocessed_path, &preprocessed);
                        eprintln!("Processed {} ballots", preprocessed.ballots.ballots.len());
                        preprocessed
                    };

                let contest_report = generate_report(&preprocessed);

                write_serialized(&report_path, &contest_report);
            }
        }
    }
}
