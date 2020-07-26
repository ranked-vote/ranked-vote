use crate::model::election::{Candidate, ElectionInfo};
use crate::tabulator::TabulatorRound;
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
pub struct ContestReport {
    pub info: ElectionInfo,
    pub ballot_count: u32,
    pub candidates: Vec<Candidate>,
    pub rounds: Vec<TabulatorRound>,
}

impl ContestReport {
    pub fn winner(&self) -> &Candidate {
        let final_round_winner = self
            .rounds
            .last()
            .unwrap()
            .allocations
            .first()
            .unwrap()
            .allocatee
            .unwrap_candidate_id();
        &self.candidates[final_round_winner.0 as usize]
    }

    pub fn num_candidates(&self) -> u32 {
        self.candidates.iter().filter(|d| !d.write_in).count() as u32
    }
}
