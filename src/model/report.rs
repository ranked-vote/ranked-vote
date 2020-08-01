use crate::model::election::{Candidate, ElectionInfo, CandidateId};
use crate::tabulator::{TabulatorRound, Allocatee};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportIndex {
    pub elections: Vec<ElectionIndexEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionIndexEntry {
    pub path: String,
    pub jurisdiction_name: String,
    pub election_name: String,
    pub date: String,
    pub contests: Vec<ContestIndexEntry>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestIndexEntry {
    pub office: String,
    pub office_name: String,
    pub name: String,
    pub winner: String,
    pub num_candidates: u32,
    pub num_rounds: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidateVotes {
    pub candidate: CandidateId,
    pub first_round_votes: u32,
    pub transfer_votes: u32,
    pub round_eliminated: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidatePairEntry {
    pub frac: f32,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidatePairTable {
    pub rows: Vec<Allocatee>,
    pub cols: Vec<Allocatee>,
    pub entries: Vec<Vec<Option<CandidatePairEntry>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestReport {
    pub info: ElectionInfo,
    pub ballot_count: u32,
    pub candidates: Vec<Candidate>,
    pub rounds: Vec<TabulatorRound>,
    pub winner: CandidateId,
    pub num_candidates: u32,
    pub total_votes: Vec<CandidateVotes>,
    pub pairwise_preferences: CandidatePairTable,
    pub first_alternate: CandidatePairTable,
    pub first_final: CandidatePairTable,
}

impl ContestReport {
    pub fn winner(&self) -> &Candidate {
        &self.candidates[self.winner.0 as usize]
    }
}
