use std::path::PathBuf;

use clap::{Command, command, arg, value_parser, Arg};

pub fn get_cli_command() -> Command {
    command!()
        .subcommand(
            Command::new("init")
                .about("Initialize the TRI metafile in the current folder")
                .arg(
                    arg!(
                    -p --path <FILE> "Specify the path to the initial image"
                )
                        .required(true)
                        .value_parser(value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("commit")
                .about("Make a commit based on the current one and bump HEAD to it")
                .arg(
                    arg!(<cmd> ... "magick commands")
                        .trailing_var_arg(true)
                )
                .arg(
                    arg!(
                    -p --path <FILE> "Specify the path to the initial image"
                )
                        .required(false)
                        .value_parser(value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("log")
                .about("Print history of changes from HEAD to the Root")
        )
        .subcommand(
            Command::new("tree")
                .about("Visualizes the tree of commits")
        )
        .subcommand(
            Command::new("tree-watch")
                .about("Visualizes the tree of commits")
        )
        .subcommand(
            Command::new("reset")
                .about("Reset HEAD to another commit")
                .arg(Arg::new("addr"))
                .arg(
                    arg!(
                    -p --path <FILE> "Specify the path to the initial image"
                    )
                    .required(false)
                    .value_parser(value_parser!(PathBuf))
                )
        )
}
