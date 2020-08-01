use crate::model::election::{CandidateId, Choice};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorRound {
    pub allocations: Vec<TabulatorAllocation>,
    pub undervote: u32,
    pub overvote: u32,
    pub continuing_ballots: u32,
    pub transfers: Vec<Transfer>,
    //eliminated: Vec<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TabulatorAllocation {
    pub allocatee: Allocatee,
    pub votes: u32,
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Debug)]
pub enum Allocatee {
    Candidate(CandidateId),
    Exhausted,
}

impl Allocatee {
    pub fn from_choice(c: Choice) -> Allocatee {
        match c {
            Choice::Vote(v) => Allocatee::Candidate(v),
            _ => Allocatee::Exhausted,
        }
    }

    pub fn candidate_id(&self) -> Option<CandidateId> {
        match self {
            Allocatee::Candidate(c) => Some(*c),
            _ => None,
        }
    }
}

impl Serialize for Allocatee {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Allocatee::Candidate(CandidateId(c)) => serializer.serialize_u32(*c),
            Allocatee::Exhausted => serializer.serialize_str("X"),
        }
    }
}

struct AllocateeVisitor;

impl<'de> Visitor<'de> for AllocateeVisitor {
    type Value = Allocatee;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned integer or \"X\"")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Allocatee::Candidate(CandidateId(v as u32)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "X" => Ok(Allocatee::Exhausted),
            _ => Err(de::Error::custom("Expected \"X\".")),
        }
    }
}

impl<'de> Deserialize<'de> for Allocatee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AllocateeVisitor)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub from: CandidateId,
    pub to: Allocatee,
    pub count: u32,
}
