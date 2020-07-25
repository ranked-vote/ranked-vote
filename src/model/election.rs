use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct CandidateId(pub u32);

struct CandidateIdVisitor;

impl<'de> Visitor<'de> for CandidateIdVisitor {
    type Value = CandidateId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned integer")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CandidateId(v as u32))
    }
}

impl Serialize for CandidateId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

impl<'de> Deserialize<'de> for CandidateId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(CandidateIdVisitor)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct Candidate {
    name: String,
    write_in: bool,
}

impl Candidate {
    pub fn new(name: String, write_in: bool) -> Candidate {
        Candidate { name, write_in }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub enum Choice {
    Vote(CandidateId),
    Undervote,
    Overvote,
}

#[derive(Debug, PartialEq)]
pub struct Ballot {
    pub id: String,
    pub choices: Vec<Choice>,
}

impl Ballot {
    pub fn new(id: String, choices: Vec<Choice>) -> Ballot {
        Ballot { id, choices }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NormalizedBallot {
    pub id: String,
    choices: VecDeque<CandidateId>,
    pub overvoted: bool,
}

impl NormalizedBallot {
    pub fn new(id: String, choices: Vec<CandidateId>, overvoted: bool) -> NormalizedBallot {
        NormalizedBallot {
            id,
            choices: choices.into(),
            overvoted,
        }
    }

    #[allow(unused)]
    pub fn choices(&self) -> Vec<CandidateId> {
        self.choices.clone().into()
    }

    pub fn top_vote(&self) -> Choice {
        match self.choices.front() {
            Some(v) => Choice::Vote(*v),
            None => {
                if self.overvoted {
                    Choice::Overvote
                } else {
                    Choice::Undervote
                }
            }
        }
    }

    pub fn pop_top_vote(mut self) -> Self {
        self.choices.pop_front();
        self
    }
}

pub struct Election {
    pub candidates: Vec<Candidate>,
    pub ballots: Vec<Ballot>,
}

impl Election {
    pub fn new(candidates: Vec<Candidate>, ballots: Vec<Ballot>) -> Election {
        Election {
            candidates,
            ballots,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NormalizedElection {
    pub candidates: Vec<Candidate>,
    pub ballots: Vec<NormalizedBallot>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ElectionInfo {
    /// Name of election.
    pub name: String,

    /// Date of election:
    pub date: String,

    pub data_format: String,

    pub tabulation: String,

    pub office: String,

    pub loader_params: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectionPreprocessed {
    pub info: ElectionInfo,
    pub ballots: NormalizedElection,
}
