use std::collections::HashSet;
use std::{path::Path, vec, fs::File, io::Write};
use colored::Colorize;

use crate::{hash::Hash, magick::MagickCommand};

#[derive(Clone)]
#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub enum CommitKind {
    Normal,
    HEAD,
}

pub type Meta = Vec<Line>;

impl CommitKind {
    fn from_string(s: &str) -> CommitKind {
        match s {
            "" => CommitKind::Normal,
            "HEAD" => CommitKind::HEAD,
            // TODO: expect message
            _ => panic!("Ohno!")
        }
    }
}

#[derive(Clone)]
#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct Line {
    pub commit: Hash,
    pub parent: Option<Hash>,
    pub command: Option<MagickCommand>,
    pub kind: CommitKind,
}

pub fn read_meta(path: &Path) -> Meta {
    // TODO: Expect message
    let mut rdr = csv::Reader::from_path(path).expect("Problem");
    let mut res: Meta = vec![];
    for line in rdr.records() {
        // TODO: Expect message
        let line = line.expect("ohno");
        let mut iter = line.iter();
        let commit = Hash::from_string(iter.next().expect("ohno"));
        let parent_text = iter.next().expect("ohno");
        let parent = if parent_text.is_empty() {
            None
        } else {
            Some(Hash::from_string(parent_text))
        };
        let mt_text = iter.next().expect("ohno");
        let command = if mt_text.is_empty() {
            None
        } else {
            Some(MagickCommand::from_string(mt_text))
        };
        let kind = CommitKind::from_string(iter.next().expect("ohno"));
        res.push(Line { commit, parent, command, kind });
    }
    res
}

pub fn write_meta(path: &Path, meta: &Meta) {
    // TODO: message
    let mut out = File::create(path).expect("");
    writeln!(out, "{}", "commit,parent,command,node_status").unwrap();
    let mut copy = (*meta).clone();
    copy.sort();
    for line in copy {
        write!(out, "{},", line.commit).unwrap();
        match &line.parent {
            Some(parent) => write!(out, "{},", parent).unwrap(),
            None => write!(out, ",").unwrap()
        }
        match &line.command {
            Some(command) => write!(out, "{},", command).unwrap(),
            None => write!(out, ",").unwrap()
        }
        match &line.kind {
            CommitKind::Normal => (),
            CommitKind::HEAD => write!(out, "HEAD").unwrap()
        }
        writeln!(out).unwrap();
    }
    out.flush().unwrap();
}

pub fn init_meta(path: &Path, hash: &Hash) {
    // TODO: change to call write_meta
    let mut out = File::create(path).expect("");
    writeln!(out, "{}", "commit,parent,command,node_status").unwrap();
    writeln!(out, "{},,,HEAD", hash).unwrap();
    out.flush().unwrap();
}

const COMMIT_COLORS: [(u8, u8, u8); 6] = [
(0xFF, 0x1E, 0x26),
(0xFE, 0x94, 0x1E),
(0xFF, 0xFF, 0x00),
(0x06, 0xBD, 0x00),
(0x00, 0x1A, 0x98),
(0x76, 0x00, 0x88),
];

pub fn meta_visualize(meta: &Meta) {
    let mut mentioned: HashSet<Hash> = HashSet::new();
    fn vis(mentioned: &mut HashSet<Hash>, line: &Line, meta: &Meta, depth: u32, prefix: &str) {
        if mentioned.contains(&line.commit) {
            return
        }
        mentioned.insert(line.commit);
        let children = meta.iter().filter(|child| match child.parent { None => false, Some(par) => par.eq(&line.commit) } ).collect::<Vec<_>>();
        let hash_str = format!("{}", line.commit);
        let color = COMMIT_COLORS[depth as usize % COMMIT_COLORS.len()];

        let hash_str = hash_str[..6].truecolor(color.0, color.1, color.2);
        let hash_str = if line.kind == CommitKind::HEAD { hash_str.bold().underline() } else { hash_str };

        if children.len() > 0 {
            print!("─┬{}", hash_str);
        }
        else {
            print!("──{}", hash_str);
        }

        if line.command.is_some() {
            print!(" {}", format!("{}", line.command.clone().unwrap()).as_str().truecolor(127, 127, 127));
        }
        if line.kind == CommitKind::HEAD {
            print!(" ▶▶▶");
        }
        println!();

        for (i, child) in children.iter().enumerate() {
            print!("{}", prefix);
            let mut new_prefix = String::from(prefix);
            if i == children.len() - 1 {
                print!("╰");
                new_prefix.push_str("  ");
            } else {
                print!("├");
                new_prefix.push_str("│ ");
            }
            vis(mentioned, &child, meta, depth + 1, new_prefix.as_str());
        }
    }
    let root = meta.iter().filter(|line| line.parent.is_none()).next().unwrap();
    vis(&mut mentioned, root, meta, 0, " ");
}

pub fn behead_meta(meta: &mut Meta) {
    for line in meta {
        if line.kind == CommitKind::HEAD {
            line.kind = CommitKind::Normal;
        }
    }
}

pub fn meta_find_line(meta: &Meta, query: &str) -> Option<usize> {
    let mut found_substrings = meta.iter()
        .enumerate()
        .filter(|line| format!("{}", line.1.commit).find(query) == Some(0));

    let first = found_substrings.next();
    if first.is_none() { return None; }
    let second = found_substrings.next();
    if second.is_some() { return None; }
    Some(first.unwrap().0)
}
