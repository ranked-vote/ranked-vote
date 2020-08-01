mod schema;

use crate::model::election::{CandidateId, Choice, NormalizedBallot};
pub use crate::tabulator::schema::{Allocatee, TabulatorAllocation, TabulatorRound, Transfer};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// Represents the number of ballots considered to be allocated to
/// each candidate at a particular stage of tabulation.
struct Allocations {
    exhausted: u32,
    votes: Vec<(CandidateId, u32)>,
}

impl Allocations {
    pub fn new(mut votes: Vec<(CandidateId, u32)>, exhausted: u32) -> Allocations {
        // Sort descending by number of votes.
        votes.sort_by(|a, b| (b.1).cmp(&a.1));

        Allocations { votes, exhausted }
    }

    /// Returns true if a winner can be declared from this allocation.
    pub fn is_final(&self) -> bool {
        match self.votes.first() {
            Some((_, first_votes)) => {
                let rest_votes = self.continuing() - first_votes;
                *first_votes > rest_votes
            }
            _ => panic!("The contest should have at least one candidate."),
        }
    }

    /// Turn into a `TabulatorAllocation` vector.
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

    /// Return the number of continuing (non-exhausted) ballots in this round's allocation.
    pub fn continuing(&self) -> u32 {
        self.votes.iter().map(|(_, v)| v).sum()
    }
}

struct TabulatorState {
    /// Map from candidate to ballots attributed to that candidate at this round.
    /// Eliminated candidates ranking above the top non-eliminated candidate have
    /// been removed from each ballot.
    pub candidate_ballots: BTreeMap<Choice, Vec<NormalizedBallot>>,

    /// Transfers incoming from the prior round.
    pub transfers: Vec<Transfer>,

    /// Set of candidates who have already been eliminated prior to this round.
    eliminated: HashSet<CandidateId>,
}

impl TabulatorState {
    /// Obtain the `TabulatorRound` representation of a `TabulatorState`.
    /// The `TabulatorRound` representation is the one that is serialized
    /// into the report.
    pub fn as_round(&self) -> TabulatorRound {
        let allocations = self.allocations();
        let undervote = self
            .candidate_ballots
            .get(&Choice::Undervote)
            .map(|x| x.len() as u32)
            .unwrap_or(0);
        let overvote = self
            .candidate_ballots
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

    pub fn new(ballots: &Vec<NormalizedBallot>) -> TabulatorState {
        let mut allocations: BTreeMap<Choice, Vec<NormalizedBallot>> = BTreeMap::new();
        for ballot in ballots {
            let choice = ballot.top_vote();
            allocations
                .entry(choice)
                .or_insert_with(|| Vec::new())
                .push(ballot.clone());
        }
        TabulatorState {
            candidate_ballots: allocations,
            transfers: Vec::new(),
            eliminated: HashSet::new(),
        }
    }

    /// Count the ballots attributed to each candidate at this round, as well as the
    /// number of exhausted ballots.
    pub fn allocations(&self) -> Allocations {
        let mut alloc: BTreeMap<CandidateId, u32> = BTreeMap::new();
        let mut exhausted: u32 = 0;
        for (choice, ballots) in &self.candidate_ballots {
            let count = ballots.len() as u32;
            match choice {
                Choice::Undervote => exhausted += count,
                Choice::Overvote => exhausted += count,
                Choice::Vote(c) => {
                    alloc.insert(*c, count);
                }
            }
        }

        let votes: Vec<(CandidateId, u32)> = alloc.into_iter().collect();

        Allocations::new(votes, exhausted)
    }

    pub fn do_elimination(self) -> TabulatorState {
        let allocations = self.allocations();

        // Determine which candidates to eliminate.
        let candidates_to_eliminate: BTreeSet<CandidateId> = {
            let mut ai = allocations.votes.iter();
            let mut remaining_votes = allocations.continuing();

            while let Some((_, votes)) = ai.next() {
                remaining_votes -= votes;
                if votes > &remaining_votes {
                    break;
                }
            }

            ai.map(|d| d.0).collect()
        };

        let mut transfers: BTreeSet<Transfer> = BTreeSet::new();
        let mut eliminated = self.eliminated;
        eliminated.extend(candidates_to_eliminate.iter());

        let mut candidate_ballots = self.candidate_ballots;

        // For each eliminated candidate, re-allocate their votes.
        for to_eliminate in &candidates_to_eliminate {
            // Keep track of which candidate the eliminated candidate's votes go to,
            // so that we can keep track of transfers.
            let mut transfer_map: BTreeMap<Allocatee, u32> = BTreeMap::new();

            let ballots = candidate_ballots
                .remove(&Choice::Vote(*to_eliminate))
                .unwrap();

            for mut ballot in ballots {
                // Remove the top candidate from the ballot until we find one who has
                // not been eliminated.
                let new_choice = loop {
                    ballot = ballot.pop_top_vote();
                    let next_choice = ballot.top_vote();

                    if let Choice::Vote(c) = next_choice {
                        if !eliminated.contains(&c) {
                            break next_choice;
                        }
                    } else {
                        break next_choice;
                    }
                };

                candidate_ballots
                    .entry(new_choice)
                    .or_insert_with(|| Vec::new())
                    .push(ballot.clone());

                *transfer_map
                    .entry(Allocatee::from_choice(new_choice))
                    .or_default() += 1;
            }

            // Add data about transfers from the eliminated candidate to the transfers list.
            transfers.append(
                &mut transfer_map
                    .into_iter()
                    .map(|(a, count)| Transfer {
                        from: *to_eliminate,
                        to: a,
                        count,
                    })
                    .collect(),
            );
        }

        // Collect transfers and sort them such that the transfers into the candidates
        // with more votes come first.
        // TODO: it might be cleaner to move this into a constructor of TabulatorState.
        let mut transfers: Vec<Transfer> = transfers.into_iter().collect();
        transfers.sort_by_key(|x| match x.to {
            Allocatee::Exhausted => 0,
            Allocatee::Candidate(c) => {
                -(candidate_ballots.get(&Choice::Vote(c)).unwrap().len() as i32)
            }
        });

        TabulatorState {
            candidate_ballots,
            transfers,
            eliminated,
        }
    }
}

pub fn tabulate(ballots: &Vec<NormalizedBallot>) -> Vec<TabulatorRound> {
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
