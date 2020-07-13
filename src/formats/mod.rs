mod us_ca_sfo;

use crate::model::election::Election;
use std::collections::BTreeMap;
use std::path::Path;

pub type BallotReader<'a> = dyn Fn(&Path, BTreeMap<String, String>) -> Election;

pub fn get_reader_for_format<'a, 'b>(format: &str) -> &'a BallotReader<'b> {
    match format {
        "us_ca_sfo" => &us_ca_sfo::sfo_ballot_reader,
        _ => panic!("The format {} is not implemented.", format),
    }
}

pub fn read_election(format: &str, path: &Path, params: BTreeMap<String, String>) -> Election {
    let reader = get_reader_for_format(format);
    reader(path, params)
}
