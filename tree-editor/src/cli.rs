


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{path::PathBuf};
use crate::magick::MagickCommand;
use crate::tree::{read_graph, Node};
use crate::{hash::Hash, config};
use crate::meta::{CommitKind, behead_meta, meta_visualize, meta_find_line};

use colored::Colorize;
use clap::ArgMatches;
use notify::Watcher;
use crate::{io::{IO, Logger}, error::TRIError, meta::{Meta, Line}};


pub fn process_matches<TIO>(matches: &ArgMatches, io: TIO, logger: Logger) -> Result<(), TRIError> 
    where TIO: IO + Send + 'static {
    let mut io = io;
    let logger = logger;

    notify::recommended_watcher(|_| println!("Aaa")).unwrap().watch(std::path::Path::new("aaa.txt"), notify::RecursiveMode::NonRecursive).unwrap();
    loop {
        thread::sleep(Duration::from_millis(100));
    }

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Initializing the repo...");
        let img_path = matches.get_one::<PathBuf>("path")
            .ok_or(TRIError::CLIPathNotProvided)?;
        if io.meta_exists() {
            logger.warning("Can't initialize meta: file already exists");
        } else {
            let meta: Meta = vec![
                Line { 
                    commit: Hash::new(img_path)?,
                    parent: None,
                    command: None,
                    kind: CommitKind::HEAD,
                    }
            ];
            io.meta_write(&meta)?;
            logger.info("Meta initialized");
        }
        if io.config_exists() {
            logger.warning("Can't initialize config: file already exists");
        } else {
            let config = config::Config { img_path: img_path.to_path_buf().into_os_string().into_string().unwrap() };
            io.config_write(&config)?;
            logger.info("Config initialized");
        }
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("log") {
        let graph = read_graph(&io.meta_read()?)?;
        fn crawl_graph<T1, T2>(graph: &Node, ima: &mut T1, imc: &mut T2) where T1: FnMut(&Hash), T2: FnMut(&MagickCommand, &Hash) {
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
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("commit") {
        let mut meta = io.meta_read()?;
        let graph = read_graph(&meta)?;
        let mut trail: Vec<_> = matches.get_many::<String>("cmd").unwrap().map(String::from).collect();
        if trail.len() > 0 && trail[0].find("-") != Some(0) {
            let mut new_cmd = String::from("-");
            new_cmd.push_str(trail[0].as_str());
            trail[0] = new_cmd;
        }
        let mag = MagickCommand { args: trail };
        let img_path = get_img_path(matches, &mut io)?;
        let img_path = img_path.as_path();
        let hash = graph.materialize(img_path, &logger, &mut io)?;
        let new_hash = io.materialize_magick(&hash, &mag)?;
        if hash.eq(&new_hash) {
            logger.warning("No commit made: image has not changed");
            return Ok(());
        }
        let new_graph = Node::new(Box::new(graph), mag.clone(), new_hash);
        new_graph.materialize(img_path, &logger, &mut io)?;

        behead_meta(&mut meta);

        meta.push(Line {
            commit: new_hash,
            parent: Some(hash),
            command: Some(mag.clone()),
            kind: CommitKind::HEAD,
        });

        io.meta_write(&meta)?;
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("tree") {
        let meta = io.meta_read()?;
        meta_visualize(&meta);
        return Ok(());
    }

    if let Some(_) = matches.subcommand_matches("tree-watch") {
        let meta = io.meta_read()?;
        meta_visualize(&meta);
        let arc_io = Arc::new(Mutex::new(io));
        let arc_logger = Arc::new(Mutex::new(logger));
        let arc_io_1 = Arc::clone(&arc_io);
        let watcher = move |res: notify::Result<notify::Event>| {
            match res {
                Ok(event) => {
                    match event.kind {
                        notify::EventKind::Access(notify::event::AccessKind::Close(_)) => {
                            match arc_io_1.lock().unwrap().meta_read() {
                                Ok(meta) => { 
                                    print!("{}[2J", 27 as char);
                                    meta_visualize(&meta);
                                },
                                Err(err) => 
                                    arc_logger.lock().unwrap()
                                        .tri_error(&err)
                            }
                        },
                        _ => ()
                    }
                },
                Err(e) => arc_logger
                    .lock().unwrap()
                    .error(format!("Watch error: {:?}", e).as_str()),
            }
        };
        let arc_io_clone = Arc::clone(&arc_io);
        arc_io_clone.lock().unwrap().watch_meta(watcher)?;
        loop {
            thread::sleep(Duration::from_millis(100));
        }
        return Ok(());
    }

    if let Some(matches) = matches.subcommand_matches("reset") {
        let mut meta = io.meta_read()?;
        let commit_addr = matches.get_one::<String>("addr")
            .ok_or(TRIError::CLIArgNotProvided(String::from("addr")))?;
        let img_path = get_img_path(matches, &mut io)?;
        let img_path = img_path.as_path();
        let line_id = meta_find_line(&meta, commit_addr)
            .ok_or(TRIError::GraphBadCommitAddr)?;
        // let old_hash = graph.materialize(img_path, &log);
        behead_meta(&mut meta);
        meta[line_id].kind = CommitKind::HEAD;
        let new_graph = read_graph(&meta)?;
        let new_hash = new_graph.materialize(img_path, &logger, &mut io)?;
        io.meta_write(&meta);
        logger.info(format!("HEAD reset to {}", new_hash).as_str());
        return Ok(());
    }
    Err(TRIError::CLIDontKnowWhatToDo)
}


fn get_img_path<TIO>(matches: &ArgMatches, io: &mut TIO) -> Result<PathBuf, TRIError> where TIO : IO {
    let path = matches.get_one::<PathBuf>("path");
    if path.is_some() {
        return Ok(path.unwrap().clone());
    }
    let config = io.config_read()?;
    Ok(PathBuf::from(config.img_path))
}
