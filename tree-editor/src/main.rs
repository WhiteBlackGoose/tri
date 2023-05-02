mod hash;
mod magick;
mod meta;
mod tree;
mod config;
mod io;
mod cli;
mod error;

use std::{path::{PathBuf}};

use clap::{arg, command, value_parser, Command, Arg};

use crate::io::{RealIO, Logger};

fn main() {
    // https://docs.rs/clap/latest/clap/_tutorial/
    let matches =
        command!()
            .subcommand(
                Command::new("init")
                    .about("Initialize the TRI metafile in the current folder")
                    .arg(
                        arg!(
                        -p --path <FILE> "Specify the path to the initial image"
                    )
                            .required(true)
                            .value_parser(value_parser!(PathBuf))
                    )
            )
            .subcommand(
                Command::new("commit")
                    .about("Make a commit based on the current one and bump HEAD to it")
                    .arg(
                        arg!(<cmd> ... "magick commands")
                            .trailing_var_arg(true)
                    )
                    .arg(
                        arg!(
                        -p --path <FILE> "Specify the path to the initial image"
                    )
                            .required(false)
                            .value_parser(value_parser!(PathBuf))
                    )
            )
            .subcommand(
                Command::new("log")
                    .about("Print history of changes from HEAD to the Root")
            )
            .subcommand(
                Command::new("tree")
                    .about("Visualizes the tree of commits")
            )
            .subcommand(
                Command::new("tree-watch")
                    .about("Visualizes the tree of commits")
            )
            .subcommand(
                Command::new("reset")
                    .about("Reset HEAD to another commit")
                    .arg(Arg::new("addr"))
                    .arg(
                        arg!(
                        -p --path <FILE> "Specify the path to the initial image"
                        )
                        .required(false)
                        .value_parser(value_parser!(PathBuf))
                    )
            )
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
            logger.tri_error(&err)
        }
    }
}

