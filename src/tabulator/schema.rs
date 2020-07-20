use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorRound {
    allocations: Vec<TabulatorAllocation>,
    undervote: u32,
    overvote: u32,
    continuing_ballots: u32,
    eliminated: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorAllocation {
    allocatee: Allocatee,
    votes: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Allocatee {
    Candidate(u32),
    Exhausted,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    from: u32,
    to: u32,
    count: u32
}
