use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Candidate {
    name: String,
    write_in: bool,
}

impl Candidate {
    pub fn new(name: String, write_in: bool) -> Candidate {
        Candidate { name, write_in }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    Vote(u32),
    Undervote,
    Overvote,
    WriteIn,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ballot {
    pub id: String,
    pub choices: Vec<Choice>,
}

impl Ballot {
    pub fn new(id: String, choices: Vec<Choice>) -> Ballot {
        Ballot { id, choices }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Election {
    candidates: Vec<Candidate>,
    ballots: Vec<Ballot>,
}

impl Election {
    pub fn new(candidates: Vec<Candidate>, ballots: Vec<Ballot>) -> Election {
        Election {
            candidates,
            ballots,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionMetadata {
    /// Name of election.
    pub name: String,

    /// Date of election:
    pub date: String,

    pub data_format: String,

    pub tabulation: String,

    pub office: String,

    pub loader_params: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionPreprocessed {
    pub meta: ElectionMetadata,
    pub ballots: Election,
}
