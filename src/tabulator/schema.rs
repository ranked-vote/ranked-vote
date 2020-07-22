use crate::model::election::CandidateId;
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

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Allocatee {
    Candidate(CandidateId),
    Exhausted,
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

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: CandidateId,
    pub to: Allocatee,
    pub count: u32,
}
