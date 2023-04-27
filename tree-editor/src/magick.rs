use std::process::Command;

#[derive(Clone)]
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

pub fn magick<TLog>(from: &str, to: &str, mc: &MagickCommand, log: &TLog) where TLog : Fn(&str) {
    let mut cmd = Command::new("convert");
    let mut str_to_log = String::from("Running command: convert ");
    cmd.arg(from);
    str_to_log.push_str(format!("{from} ").as_str());
    for arg in &mc.args {
        cmd.arg(arg);
        str_to_log.push_str(format!("{arg} ").as_str());
    }
    cmd.arg(to);
    str_to_log.push_str(format!("{to} ").as_str());
    log(str_to_log.as_str());
    cmd.output().expect("Ohno");
}
