use crate::formats::read_election;
use crate::model::election::{ElectionPreprocessed,ElectionInfo};
use crate::model::metadata::{ElectionMetadata, Contest, ElectionCommission};
use crate::model::report::ContestReport;
use std::collections::BTreeMap;
use std::path::Path;
use crate::normalizers::normalize_election;

pub fn generate_report(election: &ElectionPreprocessed) -> ContestReport {
    ContestReport {
        info: election.info.clone(),
        ballot_count: election.ballots.ballots.len() as u32,
    }
}

pub fn preprocess_election(
    raw_base: &Path,
    metadata: &ElectionMetadata,
    election_path: &str,
    ec: &ElectionCommission,
    contest: &Contest
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
