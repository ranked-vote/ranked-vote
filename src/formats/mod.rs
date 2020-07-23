mod dominion_rcr;
mod nist_sp_1500;
mod us_ca_sfo;
mod us_me;
mod us_vt_btv;
mod util;

use crate::model::election::Election;
use std::collections::BTreeMap;
use std::path::Path;

pub type BallotReader = dyn Fn(&Path, BTreeMap<String, String>) -> Election;

pub fn get_reader_for_format(format: &str) -> &'static BallotReader {
    match format {
        "us_ca_sfo" => &us_ca_sfo::sfo_ballot_reader,
        "nist_sp_1500" => &nist_sp_1500::nist_ballot_reader,
        "us_vt_btv" => &us_vt_btv::btv_ballot_reader,
        "dominion_rcr" => &dominion_rcr::dominion_rcr_ballot_reader,
        "us_me" => &us_me::maine_ballot_reader,
        _ => panic!("The format {} is not implemented.", format),
    }
}

pub fn read_election(format: &str, path: &Path, params: BTreeMap<String, String>) -> Election {
    let reader = get_reader_for_format(format);
    reader(path, params)
}
