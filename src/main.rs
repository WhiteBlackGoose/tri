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
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-rotate", "15"]);
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-crop", "100x200+0x0"]);
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-monochrome"]);
    // .get_matches_from(vec!["", "log"]);
    // .get_matches_from(vec!["", "tree"]);
    // .get_matches_from(vec!["", "init", "--path", "../meme-example.png"]);
    // .get_matches_from(vec!["", "reset", "--path", "../meme-example.png", "0d606f"]);
    // .get_matches_from(vec!["", "reset", "0d606f", "--path", "../meme-example.png"]);
    // .get_matches_from(vec!["", "reset", "--help"]);


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
            logger.tri_error(&err);
            std::process::exit(1);
        }
    }
}

