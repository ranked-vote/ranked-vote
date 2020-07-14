use crate::model::election::ElectionMetadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContestReport {
    pub meta: ElectionMetadata,
    pub ballot_count: u32,
}
