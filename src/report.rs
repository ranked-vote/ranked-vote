use crate::formats::read_election;
use crate::model::election::{ElectionInfo, ElectionPreprocessed};
use crate::model::metadata::{Contest, ElectionCommission, ElectionMetadata};
use crate::model::report::ContestReport;
use crate::normalizers::normalize_election;
use crate::tabulator::tabulate;
use std::path::Path;

pub fn generate_report(election: &ElectionPreprocessed) -> ContestReport {
    let rounds = tabulate(&election.ballots.ballots);

    ContestReport {
        info: election.info.clone(),
        ballot_count: election.ballots.ballots.len() as u32,
        candidates: election.ballots.candidates.clone(),
        rounds,
    }
}

pub fn preprocess_election(
    raw_base: &Path,
    metadata: &ElectionMetadata,
    election_path: &str,
    ec: &ElectionCommission,
    contest: &Contest,
) -> ElectionPreprocessed {
    let mut ballots = read_election(
        &metadata.data_format,
        &raw_base.join(&election_path),
        contest.loader_params.clone().unwrap_or_default(),
    );
    let office = ec.offices.get(&contest.office).unwrap();

    ballots = normalize_election(&metadata.normalization, ballots);

    ElectionPreprocessed {
        info: ElectionInfo {
            name: office.name.clone(),
            office: contest.office.clone(),
            date: metadata.date.clone(),
            data_format: metadata.data_format.clone(),
            tabulation: metadata.tabulation.clone(),
            loader_params: contest.loader_params.clone(),
        },
        ballots,
    }
}
