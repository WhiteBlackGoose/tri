use std::{path::Path, vec};

use crate::{hash::Hash, magick::MagickCommand};

#[derive(PartialEq)]
pub enum CommitKind {
    Normal,
    HEAD,
    ROOT
}

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

pub struct Line {
    pub commit: Hash,
    pub parent: Option<Hash>,
    pub command: Option<MagickCommand>,
    pub kind: CommitKind
}

pub fn read_meta(path: &Path) -> Vec<Line> {
    // TODO: Expect message
    let mut rdr = csv::Reader::from_path(path).expect("Problem");
    let mut res: Vec<Line> = vec![];
    for line in rdr.records().skip(1) {
        // TODO: Expect message
        let line = line.expect("ohno");
        let mut iter = line.iter();
        let commit = Hash::from_string(iter.next().expect("ohno"));
        let parent = Hash::from_string(iter.next().expect("ohno"));
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
