use crate::model::election::{Ballot, Candidate, Choice, Election};
use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const CANDIDATE: &str = "Candidate";
const WRITE_IN: &str = "WRITE-IN";
const WRITE_IN_PREFIX: &str = "WRITE-IN - ";

#[derive(Debug)]
struct CandidateData {
    /// Mapping from external candidate numbers to our candidate numbers.
    id_to_candidate: HashMap<u32, Candidate>,
    id_to_index: HashMap<u32, u32>,
    candidates: Vec<Candidate>,
    write_in: Option<u32>,
}

impl CandidateData {
    pub fn new() -> CandidateData {
        CandidateData {
            id_to_candidate: HashMap::new(),
            id_to_index: HashMap::new(),
            candidates: Vec::new(),
            write_in: None,
        }
    }

    pub fn add(&mut self, candidate_id: u32, candidate: Candidate) {
        self.id_to_candidate.insert(candidate_id, candidate);
    }

    pub fn id_to_choice(&mut self, candidate_id: u32) -> Choice {
        if Some(candidate_id) == self.write_in {
            Choice::WriteIn
        } else {
            let index = if let Some(c) = self.id_to_index.get(&candidate_id) {
                *c
            } else {
                let candidate = self
                    .id_to_candidate
                    .get(&candidate_id)
                    .expect("Saw a candidate on the ballot who was not in the original data.");
                let index = self.candidates.len() as u32;
                self.candidates.push(candidate.clone());
                self.id_to_index.insert(candidate_id, index);
                index
            };

            Choice::Vote(index)
        }
    }

    pub fn to_vec(self) -> Vec<Candidate> {
        self.candidates
    }
}

#[derive(Debug)]
struct MasterRecord {
    record_type: String,
    record_id: u32,
    description: String,
    _list_order: u32,
    contest_id: u32,
    is_writein: bool,
    _is_provisional: bool,
}

impl MasterRecord {
    fn parse(input: &str) -> MasterRecord {
        MasterRecord {
            record_type: input[0..10].trim().clone().into(),
            record_id: input[10..17].parse().unwrap(),
            description: input[17..67].trim().clone().into(),
            _list_order: input[67..74].parse().unwrap(),
            contest_id: input[74..81].parse().unwrap(),
            is_writein: &input[81..82] == "1",
            _is_provisional: &input[82..83] == "1",
        }
    }
}

#[derive(Debug)]
struct BallotRecord {
    contest_id: u32,
    pref_voter_id: u32,
    _serial_number: u32,
    _tally_type_id: u32,
    _precinct_id: u32,
    vote_rank: u32,
    candidate_id: u32,
    over_vote: bool,
    under_vote: bool,
}

impl BallotRecord {
    fn parse(input: &str) -> BallotRecord {
        BallotRecord {
            contest_id: input[0..7].parse().unwrap(),
            pref_voter_id: input[7..16].parse().unwrap(),
            _serial_number: input[16..23].parse().unwrap(),
            _tally_type_id: input[23..26].parse().unwrap(),
            _precinct_id: input[26..33].parse().unwrap(),
            vote_rank: input[33..36].parse().unwrap(),
            candidate_id: input[36..43].parse().unwrap(),
            over_vote: &input[43..44] == "1",
            under_vote: &input[44..45] == "1",
        }
    }
}

fn read_candidates(reader: &mut dyn BufRead) -> CandidateData {
    let mut candidates = CandidateData::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let record = MasterRecord::parse(&line);

        if record.record_type == CANDIDATE {
            let name = record.description;

            if name == WRITE_IN {
                candidates.write_in = Some(record.record_id);
                continue;
            }

            let candidate = if name.starts_with(WRITE_IN_PREFIX) {
                let name = name[(WRITE_IN_PREFIX.len())..].to_string();
                Candidate::new(name, true)
            } else {
                Candidate::new(name, record.is_writein)
            };
            candidates.add(record.record_id, candidate);
        }
    }
    candidates
}

fn read_ballots<'a>(
    reader: &mut dyn BufRead,
    candidates: &mut CandidateData,
    contest: u32,
) -> Vec<Ballot> {
    let mut ballots = Vec::new();

    for (id, votes) in reader
        .lines()
        .into_iter()
        .map(|v| BallotRecord::parse(&v.unwrap()))
        .filter(|v| v.contest_id == contest)
        .group_by(|v| v.pref_voter_id)
        .into_iter()
    {
        let mut choices = Vec::new();

        for (i, ballot_record) in votes.enumerate() {
            if ballot_record.vote_rank != (i + 1) as u32 {
                panic!("Got record out of order.")
            }
            if ballot_record.over_vote {
                choices.push(Choice::Overvote)
            } else if ballot_record.under_vote {
                choices.push(Choice::Undervote)
            } else {
                choices.push(candidates.id_to_choice(ballot_record.candidate_id))
            }
        }

        ballots.push(Ballot::new(id.to_string(), choices))
    }
    ballots
}

pub fn sfo_ballot_reader<'a>(path: &Path, params: BTreeMap<String, String>) -> Election {
    let contest: u32 = params
        .get("contest")
        .expect("SFO elections should have a contest param.")
        .parse()
        .expect("contest param should be a number.");
    let master_file = params
        .get("masterLookup")
        .expect("SFO elections should have masterLookup parameter.");
    let ballot_file = params
        .get("ballotImage")
        .expect("SFO elections should have ballotImage parameter.");

    let mut master_reader = BufReader::new(File::open(path.join(master_file)).unwrap());
    let mut candidates = read_candidates(&mut master_reader);

    let mut ballot_reader = BufReader::new(File::open(path.join(ballot_file)).unwrap());
    let ballots = read_ballots(&mut ballot_reader, &mut candidates, contest);

    Election::new(candidates.to_vec(), ballots)
}
