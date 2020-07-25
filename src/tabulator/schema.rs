use crate::model::election::{CandidateId, Choice};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorRound {
    pub allocations: Vec<TabulatorAllocation>,
    pub undervote: u32,
    pub overvote: u32,
    pub continuing_ballots: u32,
    pub transfers: Vec<Transfer>,
    //eliminated: Vec<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorAllocation {
    pub allocatee: Allocatee,
    pub votes: u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Allocatee {
    Candidate(CandidateId),
    Exhausted,
}

impl Allocatee {
    pub fn from_choice(c: Choice) -> Allocatee {
        match c {
            Choice::Vote(v) => Allocatee::Candidate(v),
            _ => Allocatee::Exhausted
        }
    }
}

impl Serialize for Allocatee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Allocatee::Candidate(CandidateId(c)) => serializer.serialize_u32(*c),
            Allocatee::Exhausted => serializer.serialize_str("X"),
        }
    }
}

#[derive(Serialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: CandidateId,
    pub to: Allocatee,
    pub count: u32,
}
