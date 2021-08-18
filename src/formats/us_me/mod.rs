use crate::formats::common::{normalize_name, CandidateMap};
use crate::model::election::{Ballot, Candidate, CandidateType, Choice, Election};
use calamine::{open_workbook_auto, DataType, Reader};
use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;

struct ReaderOptions {
    files: Vec<String>,
}

impl ReaderOptions {
    pub fn from_params(params: BTreeMap<String, String>) -> ReaderOptions {
        let files: Vec<String> = params
            .get("files")
            .unwrap()
            .split(';')
            .map(|x| x.to_string())
            .collect();

        ReaderOptions { files }
    }
}

pub fn parse_choice(candidate: &str, candidate_map: &mut CandidateMap<String>) -> Choice {
    if candidate == "overvote" {
        Choice::Overvote
    } else if candidate == "undervote" {
        Choice::Undervote
    } else {
        lazy_static! {
            static ref CANDIDATE_RX: Regex =
                Regex::new(r#"(?:DEM |REP )?([^\(]*[^ \()])(?: +\(\d+\))?"#).unwrap();
        }
        let candidate = if let Some(c) = CANDIDATE_RX.captures(candidate) {
            c.get(1).unwrap().as_str()
        } else {
            eprintln!("not matched: {}", candidate);
            candidate
        };

        candidate_map.add_id_to_choice(
            candidate.to_string(),
            Candidate::new(normalize_name(candidate, true), CandidateType::Regular),
        )
    }
}

pub fn read_ballot(row: &[DataType], candidate_map: &mut CandidateMap<String>) -> Ballot {
    let id = row.get(0).unwrap().get_float().unwrap() as u32;

    let mut choices = Vec::new();
    for vote in &row[3..] {
        let cand = vote.get_string().unwrap();
        let choice = parse_choice(cand, candidate_map);
        choices.push(choice);
    }

    Ballot::new(id.to_string(), choices)
}

pub fn maine_ballot_reader(path: &Path, params: BTreeMap<String, String>) -> Election {
    let options = ReaderOptions::from_params(params);
    let mut ballots: Vec<Ballot> = Vec::new();
    let mut candidate_map: CandidateMap<String> = CandidateMap::new();

    for file in options.files {
        eprintln!("Reading: {}", file);
        let mut workbook = open_workbook_auto(path.join(file)).unwrap();
        let first_sheet = workbook.sheet_names().first().unwrap().clone();
        let sheet = workbook.worksheet_range(&first_sheet).unwrap().unwrap();

        let mut rows = sheet.rows();
        rows.next();
        for row in rows {
            let ballot = read_ballot(row, &mut candidate_map);
            ballots.push(ballot);
        }
    }

    Election::new(candidate_map.into_vec(), ballots)
}
