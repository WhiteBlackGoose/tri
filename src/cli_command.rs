use std::{path::PathBuf, fmt::Display};

use clap::{Command, command, arg, value_parser, Arg, CommandFactory};

pub fn get_cli_command() -> Command {
    command!()
        .about("CLI tool to manipulate images, preserving the history of changes. Like git, but for images. Powered by imagemagick.")
        .subcommand(
            Command::new("init")
                .about("Initialize the TRI metafile in the current folder. Will create files starting with `tri-`.")
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
                .about("Make a commit based on the current one and bump HEAD to it.")
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
                .about("Print history of changes from HEAD to the Root. The ones on the top are the most recent one. The last line corresponds to root (initial image) and doesn't have commands.")
        )
        .subcommand(
            Command::new("tree")
                .about("Visualizes the tree of commits")
        )
        .subcommand(
            Command::new("tree-watch")
                .about("Visualizes the tree of commits interactively. Recommended use: open another terminal, run this command there, and in your first terminal, keep performing commits. The displayed tree will automatically update as you make commits.")
        )
        .subcommand(
            Command::new("reset")
                .about("Reset HEAD to another commit and regenerate the output file.")
                .arg(Arg::new("addr"))
                .arg(
                    arg!(
                    -p --path <FILE> "Specify the path to the initial image"
                    )
                    .required(false)
                    .value_parser(value_parser!(PathBuf))
                )
        )
        .subcommand(
            Command::new("status")
                .about("Displays the state of the directory in terms of tri repo")
        )
}
