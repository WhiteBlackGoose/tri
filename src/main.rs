mod test_utility;
mod hash;
mod magick;
mod meta;
mod tree;
mod config;
mod io;
mod cli;
mod error;
mod cli_command;

use std::path::PathBuf;

use crate::io::{RealIO, Logger};

fn main() {
    // https://docs.rs/clap/latest/clap/_tutorial/
    let matches =
            cli_command::get_cli_command()
            .get_matches();

    let logger = Logger { };
    let io = RealIO {
        path_meta: PathBuf::from("tri-meta"),
        path_config: PathBuf::from("tri-config.yaml"),
        path_inter: PathBuf::from("tri-cache"),
        path_out: PathBuf::from("tri-out"),
        log: logger
    };

    match cli::process_matches(&matches, io, logger) {
        Ok(()) => (),
        Err(err) => {
            logger.tri_error(&err)
        }
    }
}

