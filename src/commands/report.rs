use crate::model::election::ElectionPreprocessed;
use crate::model::report::{ContestIndexEntry, ElectionIndexEntry, ReportIndex};
use crate::read_metadata::read_meta;
use crate::report::{generate_report, preprocess_election};
use crate::util::{read_serialized, write_serialized};
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
    let report_path = Path::new(report_dir);
    let mut election_index_entries: Vec<ElectionIndexEntry> = Vec::new();

    for (_, jurisdiction) in read_meta(meta_dir) {
        let raw_base = raw_path.join(jurisdiction.path.clone());

        for (election_path, election) in &jurisdiction.elections {
            let mut contest_index_entries: Vec<ContestIndexEntry> = Vec::new();
            eprintln!("Election: {}", election_path.red());
            for contest in &election.contests {
                let office = jurisdiction.offices.get(&contest.office).expect(&format!(
                    "Expected office {} to be in offices.",
                    &contest.office
                ));
                eprintln!("Office: {}", office.name.red());

                let output_base = report_path
                    .join(&jurisdiction.path)
                    .join(&election_path)
                    .join(&contest.office);

                let report_path = output_base.join("report.json");
                let report = if report_path.exists() && !force_report && !force_preprocess {
                    eprintln!(
                        "Skipping because {} exists.",
                        report_path.to_str().unwrap().bright_cyan()
                    );
                    read_serialized(&report_path)
                } else {
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
                            let preprocessed = preprocess_election(
                                &raw_base,
                                election,
                                election_path,
                                &jurisdiction,
                                contest,
                            );
                            write_serialized(&preprocessed_path, &preprocessed);
                            eprintln!("Processed {} ballots", preprocessed.ballots.ballots.len());
                            preprocessed
                        };

                    let contest_report = generate_report(&preprocessed);

                    write_serialized(&report_path, &contest_report);
                    contest_report
                };

                contest_index_entries.push(ContestIndexEntry {
                    office: report.info.office.clone(),
                    office_name: report.info.office_name.clone(),
                    name: report.info.name.clone(),
                    winner: report.winner().name.clone(),
                    num_candidates: report.num_candidates(),
                    num_rounds: report.rounds.len() as u32,
                })
            }

            election_index_entries.push(ElectionIndexEntry {
                path: election_path.clone(),
                jurisdiction_name: jurisdiction.name.clone(),
                election_name: election.name.clone(),
                date: election.date.clone(),
                contests: contest_index_entries,
            })
        }
    }

    election_index_entries.sort_by(|a, b| (&b.date, &b.path).cmp(&(&a.date, &a.path)));
    let report_index = ReportIndex {
        elections: election_index_entries,
    };

    write_serialized(&report_path.join("index.json"), &report_index);
}
