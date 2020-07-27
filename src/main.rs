mod commands;
mod formats;
mod model;
mod normalizers;
mod read_metadata;
mod report;
mod tabulator;
mod util;

use crate::commands::{info, report, sync};
use crate::model::metadata::Jurisdiction;
use crate::util::get_files_from_path;
use clap::{App, Arg, SubCommand};
use colored::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[macro_use]
extern crate lazy_static;

fn main() {
    let matches = App::new("ranked-vote cli")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("info")
                .about("validate dump info about election")
                .arg(
                    Arg::with_name("meta-dir")
                        .index(1)
                        .required(true)
                        .help("input directory to validate and dump"),
                ),
        )
        .subcommand(
            SubCommand::with_name("sync")
                .about("sync raw data files with metadata")
                .arg(
                    Arg::with_name("meta-dir")
                        .index(1)
                        .required(true)
                        .help("metadata directory"),
                )
                .arg(
                    Arg::with_name("raw-data-dir")
                        .index(2)
                        .required(true)
                        .help("raw data directory"),
                ),
        )
        .subcommand(
            SubCommand::with_name("report")
                .about("generate reports")
                .arg(
                    Arg::with_name("meta-dir")
                        .index(1)
                        .required(true)
                        .help("metadata directory"),
                )
                .arg(
                    Arg::with_name("raw-data-dir")
                        .index(2)
                        .required(true)
                        .help("raw data directory"),
                )
                .arg(
                    Arg::with_name("preprocessed-dir")
                        .index(3)
                        .required(true)
                        .help("preprocessed file output directory"),
                )
                .arg(
                    Arg::with_name("report-dir")
                        .index(4)
                        .required(true)
                        .help("report output directory"),
                )
                .arg(
                    Arg::with_name("force-preprocess")
                        .long("force-preprocess")
                        .short("p"),
                )
                .arg(
                    Arg::with_name("force-report")
                        .long("force-report")
                        .short("r")
                        .takes_value(false),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("validate") {
        // Validate command.
        let meta_dir = matches.value_of("meta-dir").unwrap();
        let files = get_files_from_path(Path::new(meta_dir));

        for file in files.unwrap() {
            eprint!("Checking file: {}", file.to_string_lossy().blue().bold());
            let file = File::open(file).unwrap();

            let reader = BufReader::new(file);
            let _: Jurisdiction = serde_json::from_reader(reader).unwrap();
            eprintln!(" {}", "ok!".green());
        }
    } else if let Some(matches) = matches.subcommand_matches("info") {
        let meta_dir = matches.value_of("meta-dir").unwrap();

        info(meta_dir);
    } else if let Some(matches) = matches.subcommand_matches("sync") {
        let meta_dir = matches.value_of("meta-dir").unwrap();
        let raw_dir = matches.value_of("raw-data-dir").unwrap();

        sync(meta_dir, raw_dir);
    } else if let Some(matches) = matches.subcommand_matches("report") {
        let meta_dir = matches.value_of("meta-dir").unwrap();
        let raw_dir = matches.value_of("raw-data-dir").unwrap();
        let report_dir = matches.value_of("report-dir").unwrap();
        let preprocessed_dir = matches.value_of("preprocessed-dir").unwrap();
        let force_preprocess = matches.is_present("force-preprocess");
        let force_report = matches.is_present("force-report");

        report(
            meta_dir,
            raw_dir,
            report_dir,
            preprocessed_dir,
            force_preprocess,
            force_report,
        );
    }
}
