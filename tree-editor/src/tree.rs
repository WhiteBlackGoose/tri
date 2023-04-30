use std::fs;
use std::path::Path;

use crate::hash::Hash;
use crate::magick::{MagickCommand, self};
use crate::meta::{CommitKind, read_meta, Line, Meta};

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

    fn materialize_inner<TLog>(&self, path: &Path, log: &TLog, inter_path: &Path) -> Hash where TLog: Fn(&str) {
        match self {
            Node::Image(hash) => {
                log(format!("Image {} initialized!", hash).as_str());
                let h = Hash::new(path);
                hash_verify(hash, &h);
                fs::copy(path, inter_path.join(format!("{}", hash))).unwrap();
                hash.clone()
            },
            Node::Commit(prev, action, hash) => {
                if (inter_path.join(hash.to_string())).exists() {
                    log(format!("Cache for {} is found, exiting", hash).as_str());
                    return hash.clone();
                }
                let prev = prev.materialize_inner(path, log, inter_path);
                let out = inter_path.join("tmp");
                let out_path = out.clone().into_os_string().into_string().unwrap();
                let in_ = inter_path.join(format!("{}", prev));
                let in_path = in_.clone().into_os_string().into_string().unwrap();
                let out_hash = magick::magick(in_path.as_str(), out_path.as_str(), action, log);
                hash_verify(hash, &out_hash);
                fs::rename(out_path, inter_path.join(format!("{}", out_hash))).expect("Ohno!");
                out_hash
            }
        }
    }

    pub fn materialize<TLog>(&self, path: &Path, log: &TLog, inter_path: &Path) -> Hash where TLog: Fn(&str) {
        let h = self.materialize_inner(path, log, inter_path);
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
        fs::copy(inter_path.join(h.to_string()), out_path).expect("sdfdf");
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

pub fn read_graph(meta: &Meta) -> Result<Node, String> {
    let mut head_found = meta.iter().filter(|line| line.kind == CommitKind::HEAD);
    if head_found.clone().count() != 1 {
        return Err(String::from(format!("HEAD should only be 1, not {}", head_found.clone().count())))
    }
    let head = head_found.next().ok_or_else(|| "Ohno")?;
    if let Some(par) = head.parent {
        Ok(Node::new(
                   Box::new(collect_nodes(&par, &meta)?),
                   // TODO: expect message
                   head.command.clone().expect("ohno"),
                   head.commit))
    } else {
        Ok(Node::Image(head.commit))
    }
}
