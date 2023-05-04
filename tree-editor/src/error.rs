use std::{path::PathBuf};

use crate::hash::Hash;

#[derive(Debug)]
pub enum TRIError {
    CLIMetaAlreadyExists,
    CLIPathNotProvided,
    HashFromFileError(PathBuf),
    HashMismatch(Hash, Hash),
    IOCopyFile(PathBuf, PathBuf),
    IOCreateDir(PathBuf),
    IOReadFile(PathBuf),
    IOWriteFile(PathBuf),
    MagickFailure(Option<i32>),
    MagickNotRan,
    MetaInvalidCommitKind(String),
    MetaInvalidLine(csv::Error),
    MetaReadingError,
    MetaTooFewColumns,
    MetaTooManyColumns,
    ConfigSerializationError,
    ConfigDeserializationError,
    GraphHEADNotUnique(usize),
    GraphNoCommandFound,
    IOWatchFS,
    CLIArgNotProvided(String),
    GraphBadCommitAddr,
    CLIDontKnowWhatToDo
}