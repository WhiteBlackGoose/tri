use serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub img_path: String
}

