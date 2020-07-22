use crate::model::election::{Candidate, CandidateId, Choice};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct CandidateMap<ExternalCandidateId: Eq + Hash> {
    /// Mapping from external candidate numbers to our candidate numbers.
    id_to_index: HashMap<ExternalCandidateId, CandidateId>,
    candidates: Vec<Candidate>,
    //write_in: Option<ExternalCandidateId>,
}

impl<ExternalCandidateId: Eq + Hash> CandidateMap<ExternalCandidateId> {
    pub fn new() -> CandidateMap<ExternalCandidateId> {
        CandidateMap {
            id_to_index: HashMap::new(),
            candidates: Vec::new(),
            //write_in: None,
        }
    }

    /*
    pub fn set_write_in(&mut self, external_candidate_id: ExternalCandidateId) {
        self.write_in = Some(external_candidate_id)
    }
    */

    pub fn add(&mut self, external_candidate_id: ExternalCandidateId, candidate: Candidate) {
        self.id_to_index.insert(
            external_candidate_id,
            CandidateId(self.candidates.len() as u32),
        );
        self.candidates.push(candidate);
    }

    pub fn id_to_choice(&self, external_candidate_id: ExternalCandidateId) -> Choice {
        /*
        if Some(&external_candidate_id) == self.write_in.as_ref() {
            Choice::WriteIn
        } else {
            let index = self
                .id_to_index
                .get(&external_candidate_id)
                .expect("Candidate on ballot but not in master lookup.");

            Choice::Vote(*index)
        }*/
        let index = self
            .id_to_index
            .get(&external_candidate_id)
            .expect("Candidate on ballot but not in master lookup.");

        Choice::Vote(*index)
    }

    pub fn to_vec(self) -> Vec<Candidate> {
        self.candidates
    }
}
