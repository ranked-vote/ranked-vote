use crate::model::election::{Candidate, CandidateId, Choice};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug)]
pub struct CandidateMap<ExternalCandidateId: Eq + Hash + Clone> {
    /// Mapping from external candidate numbers to our candidate numbers.
    id_to_index: HashMap<ExternalCandidateId, CandidateId>,
    candidates: Vec<Candidate>,
}

impl<ExternalCandidateId: Eq + Hash + Clone + Debug> CandidateMap<ExternalCandidateId> {
    pub fn new() -> CandidateMap<ExternalCandidateId> {
        CandidateMap {
            id_to_index: HashMap::new(),
            candidates: Vec::new(),
        }
    }

    pub fn add(&mut self, external_candidate_id: ExternalCandidateId, candidate: Candidate) {
        self.id_to_index.insert(
            external_candidate_id,
            CandidateId(self.candidates.len() as u32),
        );
        self.candidates.push(candidate);
    }

    pub fn add_id_to_choice(
        &mut self,
        external_candidate_id: ExternalCandidateId,
        candidate: Candidate,
    ) -> Choice {
        if !self.id_to_index.contains_key(&external_candidate_id) {
            eprintln!("New candidate: {:?}", external_candidate_id);
            self.add(external_candidate_id.clone(), candidate);
        }

        self.id_to_choice(external_candidate_id)
    }

    pub fn id_to_choice(&self, external_candidate_id: ExternalCandidateId) -> Choice {
        let index = self
            .id_to_index
            .get(&external_candidate_id)
            .expect("Candidate on ballot but not in master lookup.");

        Choice::Vote(*index)
    }

    pub fn into_vec(self) -> Vec<Candidate> {
        self.candidates
    }
}
