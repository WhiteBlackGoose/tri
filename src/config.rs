use serde;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
#[derive(Clone)]
pub struct Config {
    pub img_path: String
}

