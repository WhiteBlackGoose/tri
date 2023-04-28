use std::fs;
use std::path::Path;

use crate::hash::Hash;
use crate::magick::{MagickCommand, self};
use crate::meta::{CommitKind, read_meta, Line};

const INTER_STEPS_PATH: &str = "inter";

pub fn hash_verify(expected: &Hash, actual: &Hash) {
    if !expected.eq(actual) {
        panic!("Expected: {expected}. Actual: {actual}");
    }
}

pub enum Node {
    Commit(Box<Node>, MagickCommand, Hash),
    Image(Hash)
}

impl Node {

    pub fn new(prev: Box<Node>, cmd: MagickCommand, hash: Hash) -> Node {
        Node::Commit(prev, cmd, hash)
    }

    fn materialize_inner<TLog>(&self, path: &Path, log: &TLog) -> Hash where TLog: Fn(&str) {
        match self {
            Node::Image(hash) => {
                log(format!("Image {} initialized!", hash).as_str());
                let h = Hash::new(path);
                hash_verify(hash, &h);
                fs::copy(path, Path::new(INTER_STEPS_PATH).join(format!("{}", hash))).unwrap();
                hash.clone()
            },
            Node::Commit(prev, action, hash) => {
                if (Path::new(INTER_STEPS_PATH).join(hash.to_string())).exists() {
                    log(format!("Cache for {} is found, exiting", hash).as_str());
                    return hash.clone();
                }
                let prev = prev.materialize_inner(path, log);
                let out = Path::new(INTER_STEPS_PATH).join("tmp");
                let out_path = out.clone().into_os_string().into_string().unwrap();
                let inw = Path::new(INTER_STEPS_PATH).join(format!("{}", prev));
                let in_path = inw.clone().into_os_string().into_string().unwrap();
                magick::magick(in_path.as_str(), out_path.as_str(), action, log);
                let out_hash = Hash::new(out.as_path());
                hash_verify(hash, &out_hash);
                fs::rename(out_path, Path::new(INTER_STEPS_PATH).join(format!("{}", out_hash))).expect("Ohno!");
                out_hash
            }
        }
    }

    pub fn materialize<TLog>(&self, path: &Path, log: &TLog) -> Hash where TLog: Fn(&str) {
        let h = self.materialize_inner(path, log);
        let out_path = 
            if let Some(ext) = path.extension() {
                let mut s = String::from("out");
                s.push('.');
                s.push_str(ext.to_str().unwrap());
                s
            } else {
                String::from("out")
            };
        log(format!("Returning to path {}", out_path).as_str());
        fs::copy(Path::new(INTER_STEPS_PATH).join(h.to_string()), out_path).expect("sdfdf");
        h
    }
}


fn collect_nodes(hash: &Hash, lines: &Vec<crate::meta::Line>) -> Result<Node, String> {
    let mut found = lines.iter().filter(|line| line.commit.eq(hash));
    if found.clone().count() != 1 {
        return Err(String::from("Oh no!"))
    }
    let found = found.next().unwrap();
    match found.parent {
        None => Ok(Node::Image(found.commit)),
        Some(par) => Ok(Node::Commit(
            Box::new(collect_nodes(&par, lines)?), 
            // TODO: expect message
            found.command.clone().ok_or_else(|| String::from("No command found"))?,
            found.commit
            ))
    }
}

pub fn read_graph(path: &Path) -> Result<Node, String> {
    let lines = read_meta(path);
    let mut head_found = lines.iter().filter(|line| line.kind == CommitKind::HEAD);
    if head_found.clone().count() != 1 {
        return Err(String::from(format!("HEAD should only be 1, not {}", head_found.clone().count())))
    }
    let head = head_found.next().ok_or_else(|| "Ohno")?;
    if let Some(par) = head.parent {
        Ok(Node::new(
                   Box::new(collect_nodes(&par, &lines)?),
                   // TODO: expect message
                   head.command.clone().expect("ohno"),
                   head.commit))
    } else {
        Ok(Node::Image(head.commit))
    }
}
