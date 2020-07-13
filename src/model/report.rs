use serde::{Deserialize, Serialize};
use crate::model::election::ElectionMetadata;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContestReport {
    meta: ElectionMetadata,

    
}
