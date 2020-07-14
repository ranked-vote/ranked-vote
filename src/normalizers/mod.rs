pub mod simple;

use crate::model::election::{Ballot, Election};
use crate::normalizers::simple::simple_normalizer;

pub type BallotNormalizer = dyn Fn(Ballot) -> Ballot;

pub fn get_normalizer_for_format(format: &str) -> &'static BallotNormalizer {
    match format {
        "simple" => &simple_normalizer,
        _ => panic!("The normalizer {} is not implemented.", format),
    }
}

pub fn normalize_election(format: &str, election: Election) -> Election {
    let normalizer = get_normalizer_for_format(format);
    let ballots = election.ballots.into_iter().map(normalizer).collect();

    Election {
        candidates: election.candidates,
        ballots: ballots,
    }
}
