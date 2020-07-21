mod schema;

use crate::model::election::{Ballot, CandidateId, Choice};
use crate::tabulator::schema::{Allocatee, TabulatorAllocation, TabulatorRound};
use std::collections::HashMap;

struct TabulatorState {
    pub allocations: HashMap<Choice, Vec<Vec<Choice>>>,
}

impl TabulatorState {
    pub fn new(allocations: HashMap<Choice, Vec<Vec<Choice>>>) -> TabulatorState {
        TabulatorState { allocations }
    }

    pub fn do_elimination(self) -> (Self, TabulatorRound) {
        unimplemented!()
    }
}

pub fn tabulate(ballots: &Vec<Ballot>, eager: bool) -> Vec<TabulatorRound> {
    let mut assignments: HashMap<Choice, Vec<Vec<Choice>>> = HashMap::new();
    for ballot in ballots {
        if let Some(c) = ballot.choices.get(0) {
            assignments
                .entry(*c)
                .or_insert_with(|| Vec::new())
                .push(ballot.choices.clone())
        } else {
            assignments
                .entry(Choice::Undervote)
                .or_insert_with(|| Vec::new())
                .push(Vec::new())
        }
    }

    unimplemented!()
}
