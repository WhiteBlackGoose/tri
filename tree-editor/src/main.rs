mod hash;
mod magick;
mod meta;
mod tree;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    // https://docs.rs/clap/latest/clap/_tutorial/
    let matches = 
        command!()
        .subcommand(
            Command::new("init")
                .about("Initialize the TRI metafile in the current folder")
        )
        .subcommand(
            Command::new("commit")
                .about("Make a commit based on the current one and bump HEAD to it")
        )
        .subcommand(
            Command::new("log")
                .about("Print history of changes from HEAD to the Root")
        )
        .get_matches();
}
