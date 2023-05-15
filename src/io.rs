use std::{process::{Command}, path::{Path, PathBuf}, fs::{self, File}, iter::Empty};
use crate::{magick::MagickCommand, error::TRIError, meta::{Meta, CommitKind}, config::Config};
use std::{io::Write};

use super::hash::Hash;
use colored::Colorize;
use notify::{Watcher, INotifyWatcher};

pub trait IO {
    fn materialize(&mut self, from: &Path) -> Result<Hash, TRIError>;
    fn materialize_magick(&mut self, from: &Hash, cmd: &MagickCommand) -> Result<Hash, TRIError>;
    fn is_materialized(&mut self, hash: &Hash) -> bool;
    fn expose(&mut self, hash: &Hash, ext: Option<String>) -> Result<(), TRIError>;

    fn meta_exists(&mut self) -> bool;
    fn meta_write(&mut self, meta: &Meta) -> Result<(), TRIError>;
    fn meta_read(&mut self) -> Result<Meta, TRIError>;

    fn config_exists(&mut self) -> bool;
    fn config_write(&mut self, config: &Config) -> Result<(), TRIError>;
    fn config_read(&mut self) -> Result<Config, TRIError>;

    fn list_materialized(&mut self) -> Vec<Hash>;

    fn watch_meta<TWatch>(&mut self, watch: TWatch) -> Result<INotifyWatcher, TRIError> where TWatch : FnMut(notify::Result<notify::Event>) + Send + 'static;
}

#[derive(Clone, Copy)]
pub struct Logger {
}

impl Logger {
    fn print(&self, word: &str, color: &(u8, u8, u8), msg: &str) {
        let s = format!("{}: {}", word, msg).truecolor(color.0, color.1, color.2);
        println!("{}", s);
    }

    pub fn trace(&self, msg: &str) {
        self.print("TRACE", &(200, 200, 200), msg);
    }

    pub fn info(&self, msg: &str) {
        self.print("INFO", &(150, 150, 255), msg);
    }

    pub fn warning(&self, msg: &str) {
        self.print("WARNING", &(255, 255, 50), msg);
    }

    pub fn error(&self, msg: &str) {
        self.print("ERROR", &(255, 100, 100), msg);
    }

    pub fn tri_error(&self, err: &TRIError) {
        let msg: String = match err {
            TRIError::CLIPathNotProvided          => format!("There's no --path or path in config file provided"),
            TRIError::HashFromFileError(file)     => format!("Could not take hash of file {:?}", file),
            TRIError::HashMismatch(exp, act)      => format!("Graph is corrupt: expected {}, actual {}", exp, act),
            TRIError::IOCopyFile(from, to)        => format!("Could not copy file from {:?} to {:?}", from, to),
            TRIError::IOCreateDir(dir)            => format!("Could not create directory {:?}", dir),
            TRIError::IOReadFile(file)            => format!("Problem reading file {:?}", file),
            TRIError::IOWriteFile(file)           => format!("Problem writing to file {:?}", file),
            TRIError::MagickNotRan                => format!("Problem running `convert` executable"),
            TRIError::MetaInvalidCommitKind(st)   => format!("Unexpected commit kind: `{}`", st),
            TRIError::MetaInvalidLine(err)        => format!("Problem reading line in meta: {}", err),
            TRIError::MetaTooFewColumns           => format!("Too few columns in meta (expected: 4)"),
            TRIError::MetaTooManyColumns          => format!("Too many columns in meta (expected: 4)"),
            TRIError::ConfigSerializationError    => format!("Error serializing config"),
            TRIError::ConfigDeserializationError  => format!("Error deserializing config"),
            TRIError::GraphHEADNotUnique(count)   => format!("There must be 1 HEAD, not {}", count),
            TRIError::GraphNoCommandFound         => format!("Commit should have a command"),
            TRIError::IOWatchFS                   => format!("Error watching events of the filesystem"),
            TRIError::CLIArgNotProvided(arg)      => format!("CLI argument {} was not supplied", arg),
            TRIError::GraphBadCommitAddr          => format!("Commit was either not found or not unique, try longer prefix and make sure it exists"),
            TRIError::CLIDontKnowWhatToDo         => format!("I don't know what to do"),
            TRIError::HashFromStringError(s)      => format!("String `{}` is not valid hash", s),
            TRIError::TRIOuterError(count)        => format!("Error: {} errors were encountered", count),
            TRIError::MagickFailure(exit)         => match exit {
                    Some(code) => format!("imagemagick failed to perform, error code: {}", code),
                    None => format!("imagemagick failed without error code (e. g. was terminated)")
                },
        };
        self.print("ERROR", &(255, 50, 50), msg.as_str());
    }
}

pub struct RealIO {
    pub path_inter: PathBuf,
    pub path_out: PathBuf,
    pub path_meta: PathBuf,
    pub path_config: PathBuf,
    pub log: Logger
}


impl RealIO {
    fn path_tmp(&self) -> PathBuf {
        self.path_inter.join(Path::new("tmp"))
    }
    fn path_from_hash(&self, hash: &Hash) -> PathBuf {
        self.path_inter.join(Path::new(format!("{}", hash).as_str()))
    }
}

impl IO for RealIO {
    fn materialize(&mut self, from: &Path) -> Result<Hash, TRIError> {
        if !self.path_inter.exists() {
            fs::create_dir(&self.path_inter)
                .map_err(|_| TRIError::IOCreateDir(self.path_inter.to_path_buf()))?;
        }
        let hash = Hash::new(from)?;
        let dest_path = self.path_inter.join(format!("{}", hash));
        self.log.trace(format!("Copying from {} to {}", from.display(), dest_path.display()).as_str());
        fs::copy(from, &dest_path)
            .map_err(|_| TRIError::IOCopyFile(from.to_path_buf(), dest_path.to_path_buf()))?;
        Ok(hash)
    }

    fn materialize_magick(&mut self, from: &Hash, mc: &MagickCommand) -> Result<Hash, TRIError> {
        let mut cmd = Command::new("convert");
        cmd.arg(self.path_from_hash(from));
        for arg in &mc.args {
            cmd.arg(arg);
        }
        cmd.arg(self.path_tmp());
        match (cmd.output(), cmd.status().map(|st| st.code())) {
            (Ok(_), Ok(Some(0))) => self.materialize(self.path_tmp().as_path()),
            (Ok(_out), Ok(exit)) => Err(TRIError::MagickFailure(exit)),
            _ => Err(TRIError::MagickNotRan)
        }
    }

    fn is_materialized(&mut self, hash: &Hash) -> bool {
        self.path_from_hash(hash).exists()
    }

    fn expose(&mut self, hash: &Hash, ext: Option<String>) -> Result<(), TRIError> {
        let path_in = self.path_from_hash(hash);
        let path_out = match ext {
            Some(ext) => self.path_out.with_extension(ext),
            None => self.path_out.clone()
        };
        fs::copy(path_in.clone(), path_out.clone())
            .map_err(|_| TRIError::IOCopyFile(path_in.clone(), path_out.clone()))
            .map(|_| ())
    }

    fn meta_write(&mut self, meta: &Meta) -> Result<(), TRIError> {
        let mut out = File::create(&self.path_meta)
            .map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?;
        writeln!(out, "{}", "commit,parent,command,node_status")
            .map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?;
        let mut copy = (*meta).clone();
        copy.sort();
        for line in copy {
            write!(out, "{},", line.commit).map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?;
            match &line.parent {
                Some(parent) => write!(out, "{},", parent).map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?,
                None => write!(out, ",").map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?
            }
            match &line.command {
                Some(command) => write!(out, "{},", command).map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?,
                None => write!(out, ",").map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?
            }
            match &line.kind {
                CommitKind::Normal => (),
                CommitKind::HEAD => write!(out, "HEAD").map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?
            }
            writeln!(out).map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?;
        }
        out.flush().map_err(|_| TRIError::IOWriteFile(self.path_meta.clone()))?;
        Ok(())
    }

    fn meta_read(&mut self) -> Result<Meta, TRIError> {
        let mut rdr = csv::Reader::from_path(&self.path_meta)
            .map_err(|_| TRIError::IOReadFile(self.path_meta.clone()))?;
        let mut res: Meta = vec![];
        for line in rdr.records() {
            let line = line.map_err(|err| TRIError::MetaInvalidLine(err))?;
            let mut iter = line.iter();
            let commit = Hash::from_string(iter.next().ok_or(TRIError::MetaTooFewColumns)?)?;
            let parent_text = iter.next().ok_or(TRIError::MetaTooFewColumns)?;
            let parent = if parent_text.is_empty() {
                None
            } else {
                Some(Hash::from_string(parent_text)?)
            };
            let mt_text = iter.next().ok_or(TRIError::MetaTooFewColumns)?;
            let command = if mt_text.is_empty() {
                None
            } else {
                Some(MagickCommand::from_string(mt_text))
            };
            let kind = CommitKind::from_string(iter.next().ok_or(TRIError::MetaTooFewColumns)?)?;
            if iter.next().is_some() { return Err(TRIError::MetaTooManyColumns); }
            res.push(super::meta::Line { commit, parent, command, kind });
        }
        Ok(res)
    }

    fn meta_exists(&mut self) -> bool {
        self.path_meta.exists()
    }

    fn config_exists(&mut self) -> bool {
        self.path_config.exists()
    }

    fn config_write(&mut self, config: &Config) -> Result<(), TRIError> {
        let serialized = serde_yaml::to_string(config)
            .map_err(|_| TRIError::ConfigSerializationError)?;
        let mut out = File::create(&self.path_config)
            .map_err(|_| TRIError::IOWriteFile(self.path_config.clone()))?;
        writeln!(out, "{}", serialized)
            .map_err(|_| TRIError::IOWriteFile(self.path_config.clone()))?;
        out.flush()
            .map_err(|_| TRIError::IOWriteFile(self.path_config.clone()))?;
        Ok(())
    }

    fn config_read(&mut self) -> Result<Config, TRIError> {
        match std::fs::read_to_string(&self.path_config) {
            Err(_) => Err(TRIError::IOReadFile(self.path_config.clone())),
            Ok(read) => {
                let deserialized: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&read);
                match deserialized {
                    Ok(config) => Ok(config),
                    Err(_) => Err(TRIError::ConfigDeserializationError)
                }
            }
        }
    }

    fn watch_meta<TWatch>(&mut self, watch: TWatch) -> Result<INotifyWatcher, TRIError> where TWatch : FnMut(notify::Result<notify::Event>) + Send + 'static {
        let mut watcher = notify::recommended_watcher(watch)
            .map_err(|_| TRIError::IOWatchFS)?;
        watcher.watch(self.path_meta.as_path(), notify::RecursiveMode::NonRecursive)
            .map_err(|_| TRIError::IOWatchFS)?;
        Ok(watcher)
    }

    fn list_materialized(&mut self) -> Vec<Hash> {
        if !self.path_inter.exists() {
            return vec![];
        }
        let dir = fs::read_dir(&self.path_inter).unwrap();
        let mut res = vec![];
        for entry in dir {
            let Ok(entry) = entry else {continue; };
            let filename = entry.file_name();
            let Some(filename) = filename.to_str() else { continue; };
            let Ok(hash) = Hash::from_string(filename) else { continue; };
            res.push(hash);
        }
        res
    }
}
