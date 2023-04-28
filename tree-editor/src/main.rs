mod hash;
mod magick;
mod meta;
mod tree;

use std::{path::{PathBuf, Path}, fs, thread::panicking, hash::Hash};

use clap::{arg, command, value_parser, ArgAction, Command, ArgMatches, Arg};
use magick::{MagickCommand, magick};
use meta::meta_visualize;
use tree::{read_graph, INTER_STEPS_PATH};

use crate::{meta::init_meta, tree::Node};
use crate::meta::{Line, read_meta, write_meta};
use crate::meta::CommitKind::{HEAD, Normal};

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
                    .arg(
                        arg!(<cmd> ... "magick commands")
                            .trailing_var_arg(true)
                    )
                    .arg(
                        arg!(
                        -p --path <FILE> "Specify the path to the initial image"
                    )
                            .required(true)
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
            // .get_matches();
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-rotate", "60"]);
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-crop", "100x200+0x0"]);
    // .get_matches_from(vec!["", "commit", "--path", "../meme-example.png", "--", "-monochrome"]);
    // .get_matches_from(vec!["", "log"]);
    .get_matches_from(vec!["", "tree"]);
    // .get_matches_from(vec!["", "init", "--path", "../meme-example.png"]);


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

    if let Some(_) = matches.subcommand_matches("log") {
        let graph = read_graph(&read_meta(metafile_path)).unwrap();
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
        crawl_graph(&graph);
    }

    if let Some(matches) = matches.subcommand_matches("commit") {
        let mut meta = read_meta(metafile_path);
        let graph = read_graph(&meta).unwrap();
        let trail: Vec<_> = matches.get_many::<String>("cmd").unwrap().map(String::from).collect();
        let mag = MagickCommand { args: trail };
        if let Some(img_path) = matches.get_one::<PathBuf>("path") {
            let hash = graph.materialize(img_path, &|s| println!("{}", s));
            let new_hash = magick(
                Path::new(INTER_STEPS_PATH).join(format!("{}", hash)).to_str().unwrap(),
                Path::new(INTER_STEPS_PATH).join("tmp").to_str().unwrap(),
                &mag,
                &|s| println!("{}", s));
            fs::rename(Path::new(INTER_STEPS_PATH).join("tmp"), Path::new(INTER_STEPS_PATH).join(format!("{}", new_hash))).expect("Ohno!");
            if hash.eq(&new_hash) {
                println!("Nothing changed");
                return;
            }
            let new_graph = Node::new(Box::new(graph), mag.clone(), new_hash);
            new_graph.materialize(img_path, &|s| println!("{}", s));

            for line in &mut meta {
                if line.kind == HEAD {
                    line.kind = Normal;
                }
            }

            meta.push(Line {
                commit: new_hash,
                parent: Some(hash),
                command: Some(mag.clone()),
                kind: HEAD,
            });

            write_meta(metafile_path, &meta);

        }
    }

    if let Some(_) = matches.subcommand_matches("tree") {
        let meta = read_meta(metafile_path);
        meta_visualize(&meta);
    }
}
