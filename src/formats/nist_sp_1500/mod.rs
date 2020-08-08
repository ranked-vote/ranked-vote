pub mod model;

use crate::formats::common::{normalize_name, CandidateMap};
use crate::formats::nist_sp_1500::model::{CandidateManifest, CandidateType, CvrExport, Mark};
use crate::model::election::{self, Ballot, Candidate, Choice, Election};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;

use std::path::Path;

struct ReaderOptions {
    cvr: String,
    contest: u32,
    drop_unqualified_write_in: bool,
}

impl ReaderOptions {
    pub fn from_params(params: BTreeMap<String, String>) -> ReaderOptions {
        let cvr = params
            .get("cvr")
            .expect("nist_sp_1500 elections should have cvr parameter.")
            .clone();
        let contest = params
            .get("contest")
            .expect("nist_sp_1500 elections should have contest parameter.")
            .parse()
            .expect("contest param should be a number.");
        let drop_unqualified_write_in: bool = params
            .get("dropUnqualifiedWriteIn")
            .map(|d| d.parse().unwrap())
            .unwrap_or(false);

        ReaderOptions {
            contest,
            cvr,
            drop_unqualified_write_in,
        }
    }
}

fn get_candidates(
    manifest: &CandidateManifest,
    contest_id: u32,
    drop_unqualified_write_in: bool,
) -> (CandidateMap<u32>, Option<u32>) {
    let mut map = CandidateMap::new();
    let mut write_in_external_id = None;

    for candidate in &manifest.list {
        if candidate.contest_id == contest_id {
            let candidate_type = match candidate.candidate_type {
                CandidateType::WriteIn => election::CandidateType::WriteIn,
                CandidateType::QualifiedWriteIn => election::CandidateType::QualifiedWriteIn,
                CandidateType::Regular => election::CandidateType::Regular,
            };

            if drop_unqualified_write_in && candidate_type == election::CandidateType::WriteIn {
                write_in_external_id = Some(candidate.id);
                continue;
            }

            map.add(
                candidate.id,
                Candidate::new(
                    normalize_name(&candidate.description, false),
                    candidate_type,
                ),
            );
        }
    }

    (map, write_in_external_id)
}

fn get_ballots(
    cvr: &CvrExport,
    contest_id: u32,
    map: &CandidateMap<u32>,
    dropped_write_in: Option<u32>,
) -> Vec<Ballot> {
    let mut ballots: Vec<Ballot> = Vec::new();

    for session in &cvr.sessions {
        for contest in &session.contests() {
            if contest.id == contest_id {
                let mut choices: Vec<Choice> = Vec::new();
                for (_, marks) in &contest.marks.iter().group_by(|x| x.rank) {
                    let marks: Vec<&Mark> = marks.filter(|d| !d.is_ambiguous).collect();

                    let choice = match marks.as_slice() {
                        [v] if Some(v.candidate_id) == dropped_write_in => {
                            // The standard way of handling write-ins with CVR files seems to
                            // be that write-in candidates who reach a certain threshold are
                            // promoted to "QualifiedWriteIn" type. For tabulation, unqualified
                            // write-in candidates are dropped by treating them as undervotes.
                            Choice::Undervote
                        }
                        [v] => map.id_to_choice(v.candidate_id),
                        [] => Choice::Undervote,
                        _ => Choice::Overvote,
                    };

                    choices.push(choice);
                }

                ballots.push(Ballot::new(session.record_id.to_string(), choices));
            }
        }
    }

    ballots
}

pub fn nist_ballot_reader(path: &Path, params: BTreeMap<String, String>) -> Election {
    let options = ReaderOptions::from_params(params);

    let file = File::open(path.join(&options.cvr)).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    let candidate_manifest: CandidateManifest = {
        let file = archive.by_name("CandidateManifest.json").unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let (candidates, dropped_write_in) = get_candidates(
        &candidate_manifest,
        options.contest,
        options.drop_unqualified_write_in,
    );

    let cvr: CvrExport = {
        let file = archive.by_name("CvrExport.json").unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let ballots = get_ballots(&cvr, options.contest, &candidates, dropped_write_in);

    Election::new(candidates.to_vec(), ballots)
}
