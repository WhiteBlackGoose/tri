use serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub img_path: String
}

pub enum ConfigState {
    Some(Config),
    DoesNotExist,
    OtherProblems
}

use std::{vec, io::Write};
use std::{path::Path, fs::File};

pub fn init_config(path: &Path, config: &Config) -> Result<(), serde_yaml::Error> {
    let serialized = serde_yaml::to_string(config)?;
    let mut out = File::create(path).expect("");
    writeln!(out, "{}", serialized).unwrap();
    out.flush().unwrap();
    Ok(())
}

pub fn read_config(path: &Path) -> ConfigState {
    if !path.exists() {
        return ConfigState::DoesNotExist;
    }
    match std::fs::read_to_string(path) {
        Err(_) => ConfigState::OtherProblems,
        Ok(read) => {
            let deserialized: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&read);
            match deserialized {
                Ok(config) => ConfigState::Some(config),
                Err(_) => ConfigState::OtherProblems
            }
        }
    }

}
