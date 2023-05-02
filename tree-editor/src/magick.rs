use std::{fmt::Display};

#[derive(Clone)]
#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct MagickCommand {
    pub args: Vec<String>
}

impl MagickCommand {
    pub fn from_string(s: &str) -> MagickCommand {
        MagickCommand {
            args: s.split(' ').map(String::from).collect::<Vec<_>>()
        }
    }
}

impl Display for MagickCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
        f.write_str(str.join(" ").as_str()).unwrap();
        std::fmt::Result::Ok(())
    }
}
