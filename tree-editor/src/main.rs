mod hash;
mod magick;
mod meta;
mod tree;

use std::{path::{PathBuf, Path}, fs, thread::{panicking, self}, hash::Hash, time::Duration};

use clap::{arg, command, value_parser, ArgAction, Command, ArgMatches, Arg};
use magick::{MagickCommand, magick};
use meta::{meta_visualize, behead_meta, meta_find_line, CommitKind};
use tree::{read_graph, INTER_STEPS_PATH};

use crate::{meta::init_meta, tree::Node};
use crate::meta::{Line, read_meta, write_meta};
use crate::meta::CommitKind::{HEAD, Normal};

use colored::Colorize;

const METAFILE_NAME: &str = "tri-meta";

use notify::{Watcher, RecommendedWatcher, RecursiveMode, Result};

fn main() {
    let metafile_path = Path::new(METAFILE_NAME);
    fn log(s: &str) {
        println!("{}", s)
    }
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
                        .required(true)
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
        fn crawl_graph<T1, T2>(graph: &Node, ima: &mut T1, imc: &mut T2) where T1: FnMut(&hash::Hash), T2: FnMut(&MagickCommand, &hash::Hash) {
            match graph {
                Node::Image(hash) => ima(hash),
                Node::Commit(parent, cmd, hash) => {
                    imc(cmd, hash);
                    crawl_graph(parent, ima, imc)
                }
            }
        }
        let mut count = 1;
        crawl_graph(&graph, &mut |_| (), &mut |_,_| count += 1);
        let mut iter = 0;
        crawl_graph(&graph, &mut |hash| println!("{}", format!("{}", hash).truecolor(90, 90, 90)), 
            &mut |cmd, hash|
            {
                let col = 255 - (iter * 180 / count) as u8;
                iter += 1;
                println!("{}", format!("{} {}", hash, cmd).truecolor(col, col, col));
            });
    }

    if let Some(matches) = matches.subcommand_matches("commit") {
        let mut meta = read_meta(metafile_path);
        let graph = read_graph(&meta).unwrap();
        let mut trail: Vec<_> = matches.get_many::<String>("cmd").unwrap().map(String::from).collect();
        if trail.len() > 0 && trail[0].find("-") != Some(0) {
            let mut new_cmd = String::from("-");
            new_cmd.push_str(trail[0].as_str());
            trail[0] = new_cmd;
        }
        let mag = MagickCommand { args: trail };
        if let Some(img_path) = matches.get_one::<PathBuf>("path") {
            let hash = graph.materialize(img_path, &log);
            let new_hash = magick(
                Path::new(INTER_STEPS_PATH).join(format!("{}", hash)).to_str().unwrap(),
                Path::new(INTER_STEPS_PATH).join("tmp").to_str().unwrap(),
                &mag,
                &log);
            fs::rename(Path::new(INTER_STEPS_PATH).join("tmp"), Path::new(INTER_STEPS_PATH).join(format!("{}", new_hash))).expect("Ohno!");
            if hash.eq(&new_hash) {
                println!("Nothing changed");
                return;
            }
            let new_graph = Node::new(Box::new(graph), mag.clone(), new_hash);
            new_graph.materialize(img_path, &log);

            behead_meta(&mut meta);

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

    if let Some(_) = matches.subcommand_matches("tree-watch") {
        let meta = read_meta(metafile_path);
        meta_visualize(&meta);
        let mut watcher = notify::recommended_watcher(|res: Result<notify::Event>| {
            match res {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Access(notify::event::AccessKind::Close(_)) => {
                            print!("{}[2J", 27 as char);
                            let meta = read_meta(metafile_path);
                            meta_visualize(&meta);
                        },
                        _ => ()
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }).unwrap();
        watcher.watch(metafile_path, RecursiveMode::NonRecursive).unwrap();
        loop {
            thread::sleep(Duration::from_millis(100));
        }
    }

    if let Some(matches) = matches.subcommand_matches("reset") {
        let mut meta = read_meta(metafile_path);
        if let Some(commit_addr) = matches.get_one::<String>("addr") {
        if let Some(img_path) = matches.get_one::<PathBuf>("path") {
            let line_id = meta_find_line(&meta, commit_addr)
                .expect("Either non-existent or ambiguous commit provided!");
            let img_path = img_path.as_path();
            // let old_hash = graph.materialize(img_path, &log);
            behead_meta(&mut meta);
            meta[line_id].kind = CommitKind::HEAD;
            let new_graph = read_graph(&meta).unwrap();
            let new_hash = new_graph.materialize(img_path, &log);
            write_meta(metafile_path, &meta);
            println!("HEAD reset to {}", new_hash);
        }
        }
    }
}
