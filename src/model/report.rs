use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContestReport {
    name: String,
    commission_path: String,
    election_path: String,
    office: String,
    candidates: BTreeMap<String, Candidate>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Candidate {
    name: String,
}
