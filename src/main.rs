mod commands;
mod formats;
mod model;
mod normalizers;
mod read_metadata;
mod report;
mod tabulator;
mod util;

use crate::commands::{info, report, sync};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate and dump info about election.
    Info {
        /// Input directory to validate and dump.
        meta_dir: PathBuf,
    },
    /// Sync raw data files with metadata.
    Sync {
        /// Metadata directory
        meta_dir: PathBuf,
        /// Raw data directory
        raw_data_dir: PathBuf,
    },
    /// Generate reports
    Report {
        /// Metadata directory
        meta_dir: PathBuf,
        /// Raw data directory
        raw_data_dir: PathBuf,
        /// Preprocessed file output directory
        preprocessed_dir: PathBuf,
        /// Report output directory
        report_dir: PathBuf,
        /// Whether to force preprocessing even if preprocessed files exist
        force_preprocess: bool,
        force_report: bool,
    },
}

fn main() {
    let opts = Opts::parse();

    match opts.command {
        Command::Info { meta_dir } => {
            info(&meta_dir);
        }
        Command::Sync {
            meta_dir,
            raw_data_dir,
        } => {
            sync(&meta_dir, &raw_data_dir);
        }
        Command::Report {
            meta_dir,
            raw_data_dir,
            preprocessed_dir,
            report_dir,
            force_preprocess,
            force_report,
        } => {
            report(
                &meta_dir,
                &raw_data_dir,
                &report_dir,
                &preprocessed_dir,
                force_preprocess,
                force_report,
            );
        }
    }
}
