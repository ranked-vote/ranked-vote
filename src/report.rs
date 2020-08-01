use crate::formats::read_election;
use crate::model::election::{CandidateId, ElectionInfo, ElectionPreprocessed, NormalizedBallot};
use crate::model::metadata::{Contest, ElectionMetadata, Jurisdiction};
use crate::model::report::{CandidatePairEntry, CandidatePairTable, CandidateVotes, ContestReport};
use crate::normalizers::normalize_election;
use crate::tabulator::{tabulate, Allocatee, TabulatorRound};
use colored::*;
use std::collections::{BTreeMap, HashMap, HashSet};
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

    let mut result: Vec<CandidateVotes> = candidate_to_initial_votes
        .into_iter()
        .map(|(candidate, first_round_votes)| CandidateVotes {
            candidate,
            first_round_votes,
            transfer_votes: candidate_to_final_votes[&candidate] - first_round_votes,
            round_eliminated: round_eliminated.get(&candidate).cloned(),
        })
        .collect();

    result.sort_by_key(|d| -((d.first_round_votes + d.transfer_votes) as i32));

    result
}

pub fn generate_pairwise_counts(
    candidates: &Vec<CandidateId>,
    ballots: &Vec<NormalizedBallot>,
) -> HashMap<(CandidateId, CandidateId), u32> {
    let mut preference_map: HashMap<(CandidateId, CandidateId), u32> = HashMap::new();
    let all_candidates: HashSet<CandidateId> = candidates.iter().copied().collect();

    for ballot in ballots {
        let mut above_ranked: HashSet<CandidateId> = HashSet::new();

        for vote in ballot.choices() {
            for arc in &above_ranked {
                *preference_map.entry((*arc, vote)).or_insert(0) += 1;
            }

            above_ranked.insert(vote);
        }

        let remaining = all_candidates.difference(&above_ranked);

        for candidate in remaining {
            for arc in &above_ranked {
                *preference_map.entry((*arc, *candidate)).or_insert(0) += 1;
            }
        }
    }

    preference_map
}

pub fn generate_pairwise_preferences(
    candidates: &Vec<CandidateId>,
    preference_map: &HashMap<(CandidateId, CandidateId), u32>,
) -> CandidatePairTable {
    let axis: Vec<Allocatee> = candidates
        .iter()
        .map(|d| Allocatee::Candidate(*d))
        .collect();

    let entries: Vec<Vec<Option<CandidatePairEntry>>> = candidates
        .iter()
        .map(|c1| {
            candidates
                .iter()
                .map(|c2| {
                    let m1 = preference_map.get(&(*c1, *c2)).unwrap_or(&0);
                    let m2 = preference_map.get(&(*c2, *c1)).unwrap_or(&0);
                    let count = m1 + m2;

                    if count == 0 {
                        None
                    } else {
                        let frac = *m1 as f32 / count as f32;

                        Some(CandidatePairEntry { frac, count })
                    }
                })
                .collect()
        })
        .collect();

    CandidatePairTable {
        entries,
        rows: axis.clone(),
        cols: axis,
    }
}

pub fn generate_first_alternate(
    candidates: &Vec<CandidateId>,
    ballots: &Vec<NormalizedBallot>,
) -> CandidatePairTable {
    let mut first_choice_count: HashMap<CandidateId, u32> = HashMap::new();
    let mut alternate_map: HashMap<(CandidateId, Allocatee), u32> = HashMap::new();

    for ballot in ballots {
        let choices = ballot.choices();
        if let Some(first) = choices.first() {
            let second = choices
                .get(1)
                .map(|d| Allocatee::Candidate(*d))
                .unwrap_or(Allocatee::Exhausted);
            *alternate_map.entry((*first, second)).or_insert(0) += 1;
            *first_choice_count.entry(*first).or_insert(0) += 1;
        }
    }

    let rows: Vec<Allocatee> = candidates
        .iter()
        .map(|d| Allocatee::Candidate(*d))
        .collect();
    let mut cols = rows.clone();
    cols.push(Allocatee::Exhausted);

    let entries: Vec<Vec<Option<CandidatePairEntry>>> = candidates
        .iter()
        .map(|c1| {
            let denominator = *first_choice_count.get(c1).unwrap_or(&0);

            cols.iter()
                .map(|c2| {
                    let count = *alternate_map.get(&(*c1, *c2)).unwrap_or(&0);
                    if count == 0 {
                        None
                    } else {
                        let frac = count as f32 / denominator as f32;

                        Some(CandidatePairEntry {
                            frac,
                            count: denominator,
                        })
                    }
                })
                .collect()
        })
        .collect();

    CandidatePairTable {
        entries,
        rows,
        cols,
    }
}

pub fn generate_first_final(
    candidates: &Vec<CandidateId>,
    ballots: &Vec<NormalizedBallot>,
    final_round_candidates: &HashSet<CandidateId>,
) -> CandidatePairTable {
    let mut first_final: HashMap<(CandidateId, Allocatee), u32> = HashMap::new();
    let mut first_total: HashMap<CandidateId, u32> = HashMap::new();

    for ballot in ballots {
        let choices = ballot.choices();
        if let Some(first) = choices.first() {
            if !final_round_candidates.contains(first) {
                let final_choice = match choices.iter().find(|x| final_round_candidates.contains(x))
                {
                    Some(v) => Allocatee::Candidate(*v),
                    _ => Allocatee::Exhausted,
                };

                *first_final.entry((*first, final_choice)).or_insert(0) += 1;
                *first_total.entry(*first).or_insert(0) += 1;
            }
        }
    }

    let rows: Vec<Allocatee> = candidates
        .iter()
        .filter(|x| !final_round_candidates.contains(x))
        .map(|d| Allocatee::Candidate(*d))
        .collect();

    let mut cols: Vec<Allocatee> = candidates
        .iter()
        .filter(|x| final_round_candidates.contains(x))
        .map(|d| Allocatee::Candidate(*d))
        .collect();
    cols.push(Allocatee::Exhausted);

    let entries: Vec<Vec<Option<CandidatePairEntry>>> = rows
        .iter()
        .map(|c1| {
            let total = *first_total.get(&c1.candidate_id().unwrap()).unwrap();

            cols.iter()
                .map(|c2| {
                    let count = *first_final
                        .get(&(c1.candidate_id().unwrap(), *c2))
                        .unwrap_or(&0);
                    if count == 0 {
                        None
                    } else {
                        let frac = count as f32 / total as f32;

                        Some(CandidatePairEntry { frac, count: total })
                    }
                })
                .collect()
        })
        .collect();

    CandidatePairTable {
        entries,
        rows,
        cols,
    }
}

pub fn graph(
    candidates: &Vec<CandidateId>,
    preference_map: &HashMap<(CandidateId, CandidateId), u32>,
) -> HashMap<CandidateId, Vec<CandidateId>> {
    let mut graph = HashMap::new();

    for c1 in candidates {
        for c2 in candidates {
            let c1v = preference_map.get(&(*c1, *c2)).unwrap_or(&0);
            let c2v = preference_map.get(&(*c2, *c1)).unwrap_or(&0);

            if c1v > c2v {
                graph.entry(*c2).or_insert_with(|| Vec::new()).push(*c1);
            }
        }
    }

    graph
}

pub fn smith_set(
    candidates: &Vec<CandidateId>,
    graph: &HashMap<CandidateId, Vec<CandidateId>>,
) -> HashSet<CandidateId> {
    let mut last_set: HashSet<CandidateId> = candidates.iter().cloned().collect();

    loop {
        let this_set: HashSet<CandidateId> = last_set
            .iter()
            .flat_map(|d| graph.get(d).cloned().unwrap_or(Vec::new()))
            .collect();

        if this_set.len() == 0 || this_set == last_set {
            break;
        }

        last_set = this_set;
    }

    last_set
}

/// Generate a `ContestReport` from preprocessed election data.
pub fn generate_report(election: &ElectionPreprocessed) -> ContestReport {
    let ballots = &election.ballots.ballots;
    let rounds = tabulate(&ballots);
    let winner = winner(&rounds);
    let num_candidates = election
        .ballots
        .candidates
        .iter()
        .filter(|d| !d.write_in)
        .count() as u32;

    let total_votes = total_votes(&rounds);
    let candidates: Vec<CandidateId> = total_votes.iter().map(|d| d.candidate).collect();

    let pairwise_counts: HashMap<(CandidateId, CandidateId), u32> =
        generate_pairwise_counts(&candidates, &ballots);

    let pairwise_preferences = generate_pairwise_preferences(&candidates, &pairwise_counts);
    let graph = graph(&candidates, &pairwise_counts);
    let smith_set = smith_set(&candidates, &graph);
    let condorcet = if smith_set.len() == 1 {
        smith_set.iter().next().copied()
    } else {
        None
    };

    if Some(winner) != condorcet {
        eprintln!("{}", "Non-condorcet!".purple());
    }

    let first_alternate = generate_first_alternate(&candidates, &ballots);

    let final_round_candidates: HashSet<CandidateId> = rounds
        .last()
        .unwrap()
        .allocations
        .iter()
        .flat_map(|a| a.allocatee.candidate_id())
        .collect();

    let first_final = generate_first_final(&candidates, &ballots, &final_round_candidates);

    ContestReport {
        info: election.info.clone(),
        ballot_count: election.ballots.ballots.len() as u32,
        candidates: election.ballots.candidates.clone(),
        winner,
        num_candidates,
        rounds,
        total_votes,
        pairwise_preferences,
        first_alternate,
        first_final,
        smith_set: smith_set.into_iter().collect(),
        condorcet,
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
