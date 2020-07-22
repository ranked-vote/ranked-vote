use crate::formats::util::CandidateMap;
use crate::model::election::{Ballot, Candidate, CandidateId, Choice, Election};
use crate::util::string::UnicodeString;
use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct ReaderOptions {
    ballots: String,
    archive: String,
}

impl ReaderOptions {
    pub fn from_params(params: BTreeMap<String, String>) -> Self {
        let ballots = params
            .get("ballots")
            .expect("BTV elections should have ballots parameter.")
            .clone();
        let archive = params
            .get("archive")
            .expect("BTV elections should have archive parameter.")
            .clone();

        ReaderOptions { ballots, archive }
    }
}

pub fn parse_ballot(source: &str) -> Vec<Choice> {
    if source == "" {
        return vec![];
    }

    let ranks = source.split(',');
    let mut choices = Vec::new();

    for rank in ranks {
        let choice = if rank.contains("=") {
            Choice::Overvote
        } else if let Some(candidate_id) = rank.strip_prefix("C") {
            let candidate_id: u32 = candidate_id.parse().unwrap();
            Choice::Vote(CandidateId(candidate_id - 1))
        } else {
            panic!("Bad candidate list ({}).", rank)
        };
        choices.push(choice);
    }

    choices
}

pub fn btv_ballot_reader(path: &Path, params: BTreeMap<String, String>) -> Election {
    let options = ReaderOptions::from_params(params);

    let mut archive = {
        let file = File::open(path.join(&options.archive)).unwrap();
        zip::ZipArchive::new(file).unwrap()
    };

    let lines = {
        let file = archive.by_name(&options.ballots).unwrap();
        BufReader::new(file).lines()
    };

    let candidate_rx = Regex::new(r#".CANDIDATE C(\d+), "(.+)""#).unwrap();
    let ballot_rx = Regex::new(r#"([^,]+), \d\) (.+)"#).unwrap();

    let mut candidates: Vec<Candidate> = Vec::new();
    let mut ballots: Vec<Ballot> = Vec::new();

    for line in lines {
        let line = line.unwrap();

        if let Some(caps) = candidate_rx.captures(&line) {
            let id: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let name: String = caps.get(2).unwrap().as_str().into();
            assert_eq!(id - 1, candidates.len() as u32);

            candidates.push(Candidate::new(name, false));
        } else if let Some(caps) = ballot_rx.captures(&line) {
            let id: &str = caps.get(1).unwrap().as_str();
            let votes: &str = caps.get(2).unwrap().as_str();

            let choices = parse_ballot(votes);
            let ballot = Ballot::new(id.into(), choices);
            ballots.push(ballot);
        }
    }

    Election {
        candidates,
        ballots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ballot() {
        assert_eq!(Vec::new() as Vec<Choice>, parse_ballot(""));

        assert_eq!(vec![Choice::Vote(CandidateId(3))], parse_ballot("C04"));

        assert_eq!(
            vec![Choice::Vote(CandidateId(3)), Choice::Vote(CandidateId(2))],
            parse_ballot("C04,C03")
        );

        assert_eq!(
            vec![Choice::Overvote, Choice::Vote(CandidateId(2))],
            parse_ballot("C04=C06,C03")
        );
    }
}
