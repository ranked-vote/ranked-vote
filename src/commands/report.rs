use crate::formats::read_election;
use crate::model::election::{ElectionMetadata, ElectionPreprocessed};
use crate::model::metadata::ElectionCommission;
use crate::util::get_files_from_path;
use flate2::{write::GzEncoder, Compression};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::normalizers::normalize_election;
use crate::model::report::ContestReport;

pub fn report(meta_dir: &str, raw_dir: &str, report_dir: &str) {
    let raw_path = Path::new(raw_dir);
    let files = get_files_from_path(Path::new(meta_dir));

    for path in files.unwrap() {
        eprintln!("Processing: {}", path.to_string_lossy());
        let ec: ElectionCommission = {
            let file = File::open(path.clone()).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        };

        let raw_base = raw_path.join(ec.path.clone());
        let report_path = Path::new(report_dir);

        for (election_path, election) in &ec.elections {
            for contest in &election.contests {
                let office = ec.offices.get(&contest.office).unwrap();
                // Figure out if we need to generate report at all.

                // Figure out if we need to preprocess.
                let output_base = report_path
                    .join(&ec.path)
                    .join(&election_path)
                    .join(&contest.office);
                create_dir_all(&output_base).unwrap();
                let preprocessed_path = output_base.join("normalized.json.gz");
                let report_path = output_base.join("report.json");

                let mut ballots = read_election(
                    &election.data_format,
                    &raw_base.join(&election_path),
                    contest.loader_params.clone().unwrap_or_default(),
                );

                ballots = normalize_election(&election.normalization, ballots);

                let preprocessed = ElectionPreprocessed {
                    meta: ElectionMetadata {
                        name: office.name.clone(),
                        office: contest.office.clone(),
                        date: election.date.clone(),
                        data_format: election.data_format.clone(),
                        tabulation: election.tabulation.clone(),
                        loader_params: contest.loader_params.clone(),
                    },
                    ballots,
                };

                {
                    let file = File::create(preprocessed_path).unwrap();
                    let gzfile = GzEncoder::new(file, Compression::best());
                    let writer = BufWriter::new(gzfile);
                    serde_json::to_writer(writer, &preprocessed).unwrap();
                }

                let contest_report = ContestReport {
                    meta: preprocessed.meta.clone(),
                    ballot_count: preprocessed.ballots.ballots.len() as u32
                };

                {
                    let file = File::create(report_path).unwrap();
                    let writer = BufWriter::new(file);
                    serde_json::to_writer_pretty(writer, &contest_report).unwrap();
                }
            }
        }
    }
}
