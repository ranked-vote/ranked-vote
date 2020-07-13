use crate::model::election::ElectionMetadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContestReport {
    meta: ElectionMetadata,
}
