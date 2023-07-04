
use std::path::Path;

use crate::error::{self, TRIError};
use crate::hash::Hash;
use crate::io::{Logger, IO};
use crate::magick::{MagickCommand};
use crate::meta::{CommitKind, Meta};

pub fn hash_verify(expected: &Hash, actual: &Hash) -> Result<(), TRIError> {
    if !expected.eq(actual) {
        Err(error::TRIError::HashMismatch(*expected, *actual))
    } else {
        Ok(())
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

    fn materialize_inner<TIO>(&self, path: &Path, logger: &Logger, io: &mut TIO) -> Result<Hash, TRIError>
        where TIO: crate::io::IO
        {
        match self {
            Node::Image(hash) => {
                if io.is_materialized(hash) {
                    logger.info(format!("Cache for {} is found, exiting", hash).as_str());
                    return Ok(hash.clone());
                }
                let h = Hash::new(path)?;
                hash_verify(hash, &h)?;
                let _actual_hash = io.materialize(path)?;
                logger.info(format!("Image {} initialized!", hash).as_str());
                Ok(hash.clone())
            },
            Node::Commit(prev, action, hash) => {
                if io.is_materialized(hash) {
                    logger.info(format!("Cache for {} is found, exiting", hash).as_str());
                    return Ok(hash.clone());
                }
                let prev_hash = prev.materialize_inner(path, logger, io)?;
                let new_hash = io.materialize_magick(&prev_hash, action)?;
                hash_verify(hash, &new_hash)?;
                Ok(new_hash)
            }
        }
    }

    pub fn materialize<TIO>(&self, path: &Path, logger: &Logger, io: &mut TIO) -> Result<Hash, TRIError>
        where TIO: IO {
        let hash = self.materialize_inner(path, logger, io)?;
        logger.info("Materializing into out file");
        io.expose(&hash, path.extension().map(|s| String::from(s.to_str().unwrap())))?;
        Ok(hash)
    }
}


fn collect_nodes(hash: &Hash, lines: &Vec<crate::meta::Line>) -> Result<Node, TRIError> {
    let mut found = lines.iter().filter(|line| line.commit.eq(hash));
    if found.clone().count() != 1 {
        return Err(TRIError::GraphHEADNotUnique(found.clone().count()));
    }
    let found = found.next().unwrap();
    match found.parent {
        None => Ok(Node::Image(found.commit)),
        Some(par) => Ok(Node::Commit(
            Box::new(collect_nodes(&par, lines)?), 
            found.command.clone().ok_or(TRIError::GraphNoCommandFound)?,
            found.commit
            ))
    }
}

pub fn read_graph(meta: &Meta) -> Result<Node, TRIError> {
    let mut head_found = meta.iter().filter(|line| line.kind == CommitKind::HEAD);
    if head_found.clone().count() != 1 {
        return Err(TRIError::GraphHEADNotUnique(head_found.clone().count()));
    }
    let head = head_found.next().unwrap();
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

#[cfg(test)]
mod tests {
    use crate::{hash::Hash, meta::{Meta, CommitKind}, config::Config, io::IO};

    use super::read_graph;
    use crate::tree::Node::{Commit, Image};
    use crate::meta::Line;
    use crate::test_utility::FakeIO;

    #[test]
    fn test_read_graph() {
        let mut io = FakeIO {
            f_is_materialized: true,
            f_meta_exists: true,
            f_config_exists: true,
            f_hash: Hash::from_string("187329fcf591320b1c0df4e6c832d982").unwrap(),
            f_config: Config { img_path: String::from("quack") },
            f_meta: vec![
                Line { commit: Hash::from_string("187329fcf591320b1c0df4e6c832d982").unwrap(), parent: None, command: None, kind: CommitKind::HEAD }
                ]
        };
        let graph = read_graph(&io.meta_read().unwrap());
        assert!(graph.is_ok());
        match graph.unwrap() {
            Commit(_, _, _) => panic!("Expected image"),
            Image(_) => ()
        };
    }
}
