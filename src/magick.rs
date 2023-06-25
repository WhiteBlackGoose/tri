use std::fmt::Display;
use urlencoding::{encode, decode};

use crate::error::TRIError;

#[derive(Clone)]
#[derive(Ord, Eq, PartialOrd, PartialEq)]
pub struct MagickCommand {
    pub args: Vec<String>
}

impl MagickCommand {
    pub fn decode(s: &str) -> Result<MagickCommand, TRIError> {
        let decoded = s.split(' ').map(|a| decode(a));
        if decoded.clone().any(|d| d.is_err()) {
            return Err(TRIError::MagickDecodingError(String::from(s)));
        }
        Ok(
        MagickCommand {
            args:
                decoded
                .map(|v| v.unwrap().into_owned())
                .collect::<Vec<_>>()
        }
        )
    }
    pub fn encode(s: &MagickCommand) -> String {
        s.args
            .iter()
            .map(|a| encode(a).into_owned())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Display for MagickCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let strs: Vec<String> = self.args.iter().map(|s| format!("\"{}\"", s)).collect();
        let str: Vec<&str> = strs.iter().map(|s| s.as_str()).collect();
        f.write_str(str.join(" ").as_str()).unwrap();
        std::fmt::Result::Ok(())
    }
}
