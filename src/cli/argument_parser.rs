use clap::Parser;

use super::cli_utils;

/// Unshittify bandcamp album structure and file names.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ArgumentParser {
    /// Path to the downloaded album (should be unzipped).
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    source: String,

    /// Path to the destination directory.
    #[arg(short, long, value_parser = cli_utils::parse_tilde)]
    destination: String,

    /// Should the source folder be deleted.
    #[arg(short, long)]
    remove_source: bool,
}

pub struct Arguments {
    pub source: String,
    pub destination: String,
}

impl ArgumentParser {
    pub fn arguments() -> Arguments {
        let args = ArgumentParser::parse();

        Arguments {
            source: args.source,
            destination: args.destination,
        }
    }
}
