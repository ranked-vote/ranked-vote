use crate::formats::read_election;
use crate::model::election::{CandidateId, ElectionInfo, ElectionPreprocessed};
use crate::model::metadata::{Contest, ElectionMetadata, Jurisdiction};
use crate::model::report::{CandidateVotes, ContestReport};
use crate::normalizers::normalize_election;
use crate::tabulator::{tabulate, Allocatee, TabulatorRound};
use std::collections::BTreeMap;
use std::path::Path;

pub fn winner(rounds: &Vec<TabulatorRound>) -> CandidateId {
    rounds
        .last()
        .unwrap()
        .allocations
        .first()
        .unwrap()
        .allocatee
        .candidate_id()
        .unwrap()
}

pub fn total_votes(rounds: &Vec<TabulatorRound>) -> Vec<CandidateVotes> {
    let candidate_to_initial_votes: BTreeMap<CandidateId, u32> = rounds[0]
        .allocations
        .iter()
        .flat_map(|x| match x.allocatee {
            Allocatee::Candidate(y) => Some((y, x.votes)),
            _ => None,
        })
        .collect();

    let mut candidate_to_final_votes: BTreeMap<CandidateId, u32> =
        candidate_to_initial_votes.clone();

    let mut round_eliminated: BTreeMap<CandidateId, u32> = BTreeMap::new();

    for (i, round) in rounds[1..].iter().enumerate() {
        for alloc in &round.allocations {
            if let Allocatee::Candidate(c) = alloc.allocatee {
                candidate_to_final_votes.insert(c, alloc.votes);
            }
        }

        for transfer in &round.transfers {
            round_eliminated.insert(transfer.from, (i + 1) as u32);
        }
    }

    let mut result: Vec<CandidateVotes> = candidate_to_initial_votes.into_iter().map(|(candidate, first_round_votes)| {
        CandidateVotes {
            candidate,
            first_round_votes,
            transfer_votes: candidate_to_final_votes[&candidate] - first_round_votes,
            round_eliminated: round_eliminated.get(&candidate).cloned()
        }
    }).collect();

    result.sort_by_key(|d| -((d.first_round_votes + d.transfer_votes) as i32));

    result
}

/// Generate a `ContestReport` from preprocessed election data.
pub fn generate_report(election: &ElectionPreprocessed) -> ContestReport {
    let rounds = tabulate(&election.ballots.ballots);
    let winner = winner(&rounds);
    let num_candidates = election
        .ballots
        .candidates
        .iter()
        .filter(|d| !d.write_in)
        .count() as u32;

    let total_votes = total_votes(&rounds);

    ContestReport {
        info: election.info.clone(),
        ballot_count: election.ballots.ballots.len() as u32,
        candidates: election.ballots.candidates.clone(),
        winner,
        num_candidates,
        rounds,
        total_votes,
    }
}

/// Preprocess an election by reading and normalizing the raw ballot data according
/// to the rules given in the metadata for this contest.
pub fn preprocess_election(
    raw_base: &Path,
    metadata: &ElectionMetadata,
    election_path: &str,
    ec: &Jurisdiction,
    contest: &Contest,
) -> ElectionPreprocessed {
    let election = read_election(
        &metadata.data_format,
        &raw_base.join(&election_path),
        contest.loader_params.clone().unwrap_or_default(),
    );
    let office = ec.offices.get(&contest.office).unwrap();

    let normalized_election = normalize_election(&metadata.normalization, election);

    ElectionPreprocessed {
        info: ElectionInfo {
            name: office.name.clone(),
            office: contest.office.clone(),
            date: metadata.date.clone(),
            data_format: metadata.data_format.clone(),
            tabulation: metadata.tabulation.clone(),
            loader_params: contest.loader_params.clone(),
            jurisdiction_path: ec.path.clone(),
            election_path: election_path.to_string(),
            jurisdiction_name: ec.name.clone(),
            office_name: office.name.clone(),
            election_name: metadata.name.clone(),
        },
        ballots: normalized_election,
    }
}
