use std::path::PathBuf;

use clap::Parser;

use super::cli_utils;

/// Unshittify bandcamp album structure and file names.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ArgumentParser {
    /// Path to the downloaded album (should be unzipped).
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    source: PathBuf,

    /// Path to the destination directory.
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    destination: PathBuf,

    /// Should the source folder be deleted.
    #[arg(short, long)]
    remove_source: bool,
}

#[derive(Debug)]
pub struct Arguments {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub remove_source: bool,
}

impl ArgumentParser {
    pub fn arguments() -> Arguments {
        let args = ArgumentParser::parse();

        Arguments {
            source: args.source,
            destination: args.destination,
            remove_source: args.remove_source,
        }
    }
}
