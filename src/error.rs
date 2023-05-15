use std::{path::PathBuf};

use crate::hash::Hash;

#[derive(Debug)]
pub enum TRIError {
    CLIPathNotProvided,
    HashFromFileError(PathBuf),
    HashMismatch(Hash, Hash),
    HashFromStringError(String),
    IOCopyFile(PathBuf, PathBuf),
    IOCreateDir(PathBuf),
    IOReadFile(PathBuf),
    IOWriteFile(PathBuf),
    MagickFailure(Option<i32>),
    MagickNotRan,
    MetaInvalidCommitKind(String),
    MetaInvalidLine(csv::Error),
    MetaTooFewColumns,
    MetaTooManyColumns,
    ConfigSerializationError,
    ConfigDeserializationError,
    GraphHEADNotUnique(usize),
    GraphNoCommandFound,
    IOWatchFS,
    CLIArgNotProvided(String),
    GraphBadCommitAddr,
    CLIDontKnowWhatToDo,
    TRIOuterError(i32)
}
