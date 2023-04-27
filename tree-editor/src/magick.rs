use std::process::Command;

pub fn magick<TLog>(args: &Vec<String>, log: &TLog) where TLog : Fn(&str) {
    let mut cmd = Command::new("convert");
    let mut str_to_log = String::from("Running command: convert ");
    for arg in args {
        cmd.arg(arg);
        str_to_log.push_str(format!("{arg} ").as_str());
    }
    log(str_to_log.as_str());
    cmd.output().expect("Ohno");
}
