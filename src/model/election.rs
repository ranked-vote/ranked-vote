use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Choice {
    Vote(u32),
    Undervote,
    Overvote,
    WriteIn,
}

struct ChoiceVisitor;

impl<'de> Visitor<'de> for ChoiceVisitor {
    type Value = Choice;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned integer or 'U', 'O', 'V'")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Choice::Vote(v as u32))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "U" => Ok(Choice::Undervote),
            "O" => Ok(Choice::Overvote),
            "W" => Ok(Choice::WriteIn),
            _ => Err(de::Error::custom("Expected U, O, or W if char.")),
        }
    }
}

impl Serialize for Choice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Choice::Vote(v) => serializer.serialize_u32(*v),
            Choice::Undervote => serializer.serialize_char('U'),
            Choice::Overvote => serializer.serialize_char('O'),
            Choice::WriteIn => serializer.serialize_char('W'),
        }
    }
}

impl<'de> Deserialize<'de> for Choice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ChoiceVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Ballot {
    pub id: String,
    pub choices: Vec<Choice>,
}

impl Ballot {
    pub fn new(id: String, choices: Vec<Choice>) -> Ballot {
        Ballot { id, choices }
    }
}

#[derive(Serialize, Deserialize)]
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
    pub ballots: Election,
}
