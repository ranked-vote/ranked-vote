use crate::formats::common::CandidateMap;
use crate::model::election::{Ballot, Candidate, CandidateType, Choice, Election};
use crate::util::read_serialized;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawBallot {
    pub id: String,
    pub votes: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawBallots {
    pub ballots: Vec<RawBallot>,
}

struct ReaderOptions {
    file: String,
}

impl ReaderOptions {
    pub fn from_params(params: BTreeMap<String, String>) -> ReaderOptions {
        let file: String = params.get("file").unwrap().clone();

        ReaderOptions { file }
    }
}

pub fn parse_choice(candidate: &str, candidate_map: &mut CandidateMap<String>) -> Choice {
    if candidate == "over" {
        Choice::Overvote
    } else if candidate == "under" {
        Choice::Undervote
    } else {
        candidate_map.add_id_to_choice(
            candidate.to_string(),
            Candidate::new(candidate.to_string(), CandidateType::Regular),
        )
    }
}

pub fn json_reader(path: &Path, params: BTreeMap<String, String>) -> Election {
    let options = ReaderOptions::from_params(params);

    let raw_ballots: RawBallots = read_serialized(&path.join(options.file));
    let mut candidate_map = CandidateMap::new();

    let ballots: Vec<Ballot> = raw_ballots
        .ballots
        .iter()
        .map(|d| {
            Ballot::new(
                d.id.clone(),
                d.votes
                    .iter()
                    .map(|e| parse_choice(e, &mut candidate_map))
                    .collect(),
            )
        })
        .collect();

    Election::new(candidate_map.into_vec(), ballots)
}
