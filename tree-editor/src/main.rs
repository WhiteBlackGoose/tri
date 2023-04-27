mod hash;
mod magick;
mod meta;
mod tree;
use std::{path::{PathBuf, Path}, fs, thread::panicking, hash::Hash};

use clap::{arg, command, value_parser, ArgAction, Command};

use crate::meta::init_meta;

const METAFILE_NAME: &str = "tri-meta";

fn main() {
    // https://docs.rs/clap/latest/clap/_tutorial/
    let matches = 
        command!()
        .subcommand(
            Command::new("init")
                .about("Initialize the TRI metafile in the current folder")
                .arg(arg!(-p --path "Specify the path to the initial image")
                    .required(true)
                    .value_parser(value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("commit")
                .about("Make a commit based on the current one and bump HEAD to it")
        )
        .subcommand(
            Command::new("log")
                .about("Print history of changes from HEAD to the Root")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Initializing the repo...");
        if Path::new(METAFILE_NAME).exists() {
            // TODO: message
            panic!("Can't init");
        }
        if let Some(img_path) = matches.get_one::<PathBuf>("path") {
            init_meta(img_path, &hash::Hash::new(img_path));
            println!("Initialized");
        }
        panic!("Path to image not provided!");
    }
}
