use serde::{Deserialize, Serialize};

// CvrExport.json file.

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CvrExport {
    version: String,
    election_id: String,
    pub sessions: Vec<Session>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Session {
    tabulator_id: u32,
    batch_id: u32,
    pub record_id: u32,
    counting_group_id: u32,
    image_mask: String,
    original: SessionBallot,
    modified: Option<SessionBallot>,
}

impl Session {
    pub fn ballot(&self) -> &SessionBallot {
        if let Some(ballot) = &self.modified {
            ballot
        } else {
            &self.original
        }
    }

    pub fn contests(&self) -> Vec<ContestMarks> {
        match &self.original.contests {
            Some(c) => (*c).clone(),
            None => self
                .ballot()
                .cards
                .as_ref()
                .unwrap()
                .iter()
                .flat_map(|card| card.contests.clone())
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SessionBallot {
    precinct_portion_id: u32,
    ballot_type_id: u32,
    is_current: bool,
    contests: Option<Vec<ContestMarks>>,
    cards: Option<Vec<Card>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Card {
    id: u32,
    paper_index: u32,
    contests: Vec<ContestMarks>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContestMarks {
    pub id: u32,
    pub marks: Vec<Mark>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mark {
    pub candidate_id: u32,
    party_id: Option<u32>,
    pub rank: u32,
    mark_density: u32,
    pub is_ambiguous: bool,
    is_vote: bool,
}

// CandidateManifest.json

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CandidateManifest {
    version: String,
    pub list: Vec<Candidate>,
}

#[derive(Serialize, Deserialize)]
pub enum CandidateType {
    WriteIn,
    Regular,
    QualifiedWriteIn,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Candidate {
    pub description: String,
    pub id: u32,
    external_id: Option<String>,
    pub contest_id: u32,

    #[serde(rename = "Type")]
    pub candidate_type: CandidateType,
}

// ContestManifest.json

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContestManifest {
    version: String,
    list: Vec<Contest>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contest {
    description: String,
    id: Option<u32>,
    external_id: Option<String>,
    vote_for: u32,
    num_of_ranks: u32,
}
