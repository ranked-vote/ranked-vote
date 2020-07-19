use crate::model::election::{Candidate, Choice};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CandidateMap {
    /// Mapping from external candidate numbers to our candidate numbers.
    id_to_index: HashMap<u32, u32>,
    candidates: Vec<Candidate>,
    write_in: Option<u32>,
}

impl CandidateMap {
    pub fn new() -> CandidateMap {
        CandidateMap {
            id_to_index: HashMap::new(),
            candidates: Vec::new(),
            write_in: None,
        }
    }

    pub fn set_write_in(&mut self, value: u32) {
        self.write_in = Some(value)
    }

    pub fn add(&mut self, candidate_id: u32, candidate: Candidate) {
        self.id_to_index
            .insert(candidate_id, self.candidates.len() as u32);
        self.candidates.push(candidate);
    }

    pub fn id_to_choice(&self, candidate_id: u32) -> Choice {
        if Some(candidate_id) == self.write_in {
            Choice::WriteIn
        } else {
            let index = self
                .id_to_index
                .get(&candidate_id)
                .expect("Candidate on ballot but not in master lookup.");

            Choice::Vote(*index)
        }
    }

    pub fn to_vec(self) -> Vec<Candidate> {
        self.candidates
    }
}
