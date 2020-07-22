use crate::model::election::{Candidate, ElectionInfo};
use crate::tabulator::schema::TabulatorRound;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestReport {
    pub info: ElectionInfo,
    pub ballot_count: u32,
    pub candidates: Vec<Candidate>,
    pub rounds: Vec<TabulatorRound>,
}
