mod parser;

use crate::formats::dominion_rcr::parser::rcr_file;
use crate::model::election::Election;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;

struct ReaderOptions {
    rcr: String,
}

impl ReaderOptions {
    pub fn from_params(params: BTreeMap<String, String>) -> ReaderOptions {
        let rcr = params.get("rcr").unwrap().clone();

        ReaderOptions { rcr }
    }
}

pub fn dominion_rcr_ballot_reader(path: &Path, params: BTreeMap<String, String>) -> Election {
    let options = ReaderOptions::from_params(params);

    let raw = read_to_string(path.join(options.rcr)).unwrap();

    rcr_file(&raw)
}
