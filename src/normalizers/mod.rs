pub mod simple;

use crate::model::election::Ballot;

pub type BallotNormalizer<'a> = dyn Fn(Ballot) -> Ballot;
