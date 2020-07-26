pub mod model;

use crate::formats::common::{normalize_name, CandidateMap};
use crate::formats::nist_sp_1500::model::{CandidateManifest, CandidateType, CvrExport};
use crate::model::election::{Ballot, Candidate, Choice, Election};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

struct ReaderOptions {
    cvr: String,
    contest: u32,
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

        ReaderOptions { contest, cvr }
    }
}

fn get_candidates(manifest: &CandidateManifest, contest_id: u32) -> CandidateMap<u32> {
    let mut map = CandidateMap::new();

    for candidate in &manifest.list {
        if candidate.contest_id == contest_id {
            let write_in = match candidate.candidate_type {
                CandidateType::WriteIn => true,
                CandidateType::QualifiedWriteIn => true,
                _ => false,
            };
            map.add(
                candidate.id,
                Candidate::new(normalize_name(&candidate.description), write_in),
            );
        }
    }

    map
}

fn get_ballots(cvr: &CvrExport, contest_id: u32, map: &CandidateMap<u32>) -> Vec<Ballot> {
    let mut ballots: Vec<Ballot> = Vec::new();

    for session in &cvr.sessions {
        for contest in &session.contests() {
            if contest.id == contest_id {
                let mut choices: Vec<Choice> = Vec::new();
                for mark in &contest.marks {
                    let choice = if mark.is_ambiguous {
                        Choice::Overvote
                    } else {
                        map.id_to_choice(mark.candidate_id)
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

    let candidates = get_candidates(&candidate_manifest, options.contest);

    let cvr: CvrExport = {
        let file = archive.by_name("CvrExport.json").unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let ballots = get_ballots(&cvr, options.contest, &candidates);

    Election::new(candidates.to_vec(), ballots)
}
