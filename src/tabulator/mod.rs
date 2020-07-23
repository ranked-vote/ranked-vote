pub mod schema;

use crate::model::election::{Ballot, CandidateId, Choice};
use crate::tabulator::schema::{Allocatee, TabulatorAllocation, TabulatorRound, Transfer};
use std::collections::{HashMap, HashSet};

struct Allocations {
    exhausted: u32,
    votes: Vec<(CandidateId, u32)>,
}

impl Allocations {
    pub fn is_final(&self) -> bool {
        match self.votes.as_slice() {
            // Three or more candidates. The allocation is final if
            // the first-place candidate beats the second-place candidate
            // by a margin of more than the number of continuing ballots.
            [(_, first_votes), (_, second_votes), ..] => {
                let rest = &self.votes[2..];
                let rest_votes: u32 = rest.iter().map(|(_, v)| v).sum();
                first_votes - second_votes > rest_votes
            }
            // If two or fewer candidates remain, we know it is final.
            _ => true,
        }
    }

    pub fn to_vec(self) -> Vec<TabulatorAllocation> {
        let mut v = Vec::with_capacity(self.votes.len() + 1);
        for (id, votes) in self.votes {
            v.push(TabulatorAllocation {
                allocatee: Allocatee::Candidate(id),
                votes,
            });
        }
        v.push(TabulatorAllocation {
            allocatee: Allocatee::Exhausted,
            votes: self.exhausted,
        });
        v
    }

    pub fn continuing(&self) -> u32 {
        self.votes.iter().map(|(_, v)| v).sum()
    }
}

struct TabulatorState {
    pub allocations: HashMap<Choice, Vec<Vec<Choice>>>,
    pub transfers: Vec<Transfer>,
    eliminated: HashSet<CandidateId>,
}

impl TabulatorState {
    pub fn as_round(&self) -> TabulatorRound {
        let allocations = self.allocations();
        let undervote = self
            .allocations
            .get(&Choice::Undervote)
            .map(|x| x.len() as u32)
            .unwrap_or(0);
        let overvote = self
            .allocations
            .get(&Choice::Overvote)
            .map(|x| x.len() as u32)
            .unwrap_or(0);
        let continuing_ballots = allocations.continuing();

        TabulatorRound {
            allocations: allocations.to_vec(),
            undervote,
            overvote,
            continuing_ballots,
            transfers: self.transfers.clone(),
        }
    }

    pub fn new(ballots: &Vec<Ballot>) -> TabulatorState {
        let mut allocations: HashMap<Choice, Vec<Vec<Choice>>> = HashMap::new();
        for ballot in ballots {
            if let Some(c) = ballot.choices.get(0) {
                allocations
                    .entry(*c)
                    .or_insert_with(|| Vec::new())
                    .push(ballot.choices.clone())
            } else {
                allocations
                    .entry(Choice::Undervote)
                    .or_insert_with(|| Vec::new())
                    .push(Vec::new())
            }
        }
        TabulatorState {
            allocations,
            transfers: Vec::new(),
            eliminated: HashSet::new(),
        }
    }

    pub fn allocations(&self) -> Allocations {
        let mut alloc: HashMap<CandidateId, u32> = HashMap::new();
        let mut exhausted: u32 = 0;
        for (choice, ballots) in &self.allocations {
            let count = ballots.len() as u32;
            match choice {
                Choice::Undervote => exhausted += count,
                Choice::Overvote => exhausted += count,
                Choice::Vote(c) => {
                    alloc.insert(*c, count);
                }
            }
        }

        let mut votes: Vec<(CandidateId, u32)> = alloc.into_iter().collect();
        votes.sort_by(|a, b| (b.1).cmp(&a.1));

        Allocations { votes, exhausted }
    }

    pub fn do_elimination(mut self) -> TabulatorState {
        let votes = self.allocations();

        // Determine how many eliminations to do.
        //assert!(votes.votes.len() > 2);
        let candidates_to_eliminate = {
            let mut candidates = votes.votes;
            let mut eliminate: Vec<CandidateId> = Vec::new();

            let mut total_eliminated = 0;
            loop {
                match candidates.as_slice() {
                    [.., (_, c1), (_, c2)] => {
                        if total_eliminated + *c2 > *c1 {
                            break;
                        }
                    }
                    _ => break,
                }

                let (cid, c) = candidates.pop().unwrap();
                eliminate.push(cid);
                total_eliminated += c;
            }

            assert!(eliminate.len() >= 1);
            eliminate
        };

        let mut transfers: Vec<Transfer> = Vec::new();
        self.eliminated.extend(candidates_to_eliminate.iter());

        let mut bb = self.allocations;

        for to_eliminate in &candidates_to_eliminate {
            let mut transfer_map: HashMap<Allocatee, u32> = HashMap::new();

            let ballots = bb.remove(&Choice::Vote(*to_eliminate)).unwrap();

            for mut ballot in ballots {
                // TODO: this is O(n)
                loop {
                    ballot.remove(0);
                    if let Some(Choice::Vote(c)) = ballot.first() {
                        if !self.eliminated.contains(c) {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let new_choice = if let Some(c) = ballot.get(0) {
                    c
                } else {
                    &Choice::Undervote
                };

                bb
                    .entry(*new_choice)
                    .or_insert_with(|| Vec::new())
                    .push(ballot.clone());

                match new_choice {
                    Choice::Vote(v) => {
                        *transfer_map.entry(Allocatee::Candidate(*v)).or_default() += 1
                    }
                    _ => *transfer_map.entry(Allocatee::Exhausted).or_default() += 1,
                }
            }
            
            transfers.append(&mut transfer_map.into_iter().map(|(a, count)| Transfer {
                from: *to_eliminate,
                to: a,
                count,
            }).collect());
        }

        TabulatorState {
            allocations: bb,
            transfers,
            eliminated: self.eliminated,
        }
    }
}

pub fn tabulate(ballots: &Vec<Ballot>) -> Vec<TabulatorRound> {
    let mut state = TabulatorState::new(ballots);
    let mut rounds = Vec::new();

    loop {
        let allocations = state.allocations();
        rounds.push(state.as_round());

        if allocations.is_final() {
            break;
        }

        state = state.do_elimination();
    }

    rounds
}
