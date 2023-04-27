mod hash;
mod magick;
mod meta;
mod tree;
use std::{path::{PathBuf, Path}, fs, thread::panicking, hash::Hash};

use clap::{arg, command, value_parser, ArgAction, Command};
use tree::read_graph;

use crate::{meta::init_meta, tree::Node};

const METAFILE_NAME: &str = "tri-meta";

fn main() {
    let metafile_path = Path::new(METAFILE_NAME);
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
        )
        .subcommand(
            Command::new("log")
                .about("Print history of changes from HEAD to the Root")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Initializing the repo...");
        if metafile_path.exists() {
            // TODO: message
            panic!("Can't init");
        }
        if let Some(img_path) = matches.get_one::<PathBuf>("path") {
            init_meta(metafile_path, &hash::Hash::new(img_path));
            println!("Initialized");
            return;
        }
        panic!("Path to image not provided!");
    }

    if let Some(matches) = matches.subcommand_matches("log") {
        let graph = read_graph(metafile_path).unwrap();
        println!("Latest commits are at the top");
        fn crawl_graph(graph: &Node) {
            match graph {
                Node::Image(hash) => println!("Image {}", hash),
                Node::Commit(parent, cmd, hash) => {
                    println!("Commit {cmd} ({hash})");
                    crawl_graph(parent)
                }
            }
        }
    }
}
