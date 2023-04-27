use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, process::Command, fs::{File, self}, iter::Enumerate, path::{Path, PathBuf}, fmt::{Display, Write}};
use std::fmt::format;
mod hash;
use hash::Hash;
mod magick;
use magick::{magick, MagickCommand};
mod meta;
mod tree; use tree::Node;

fn main() {
    let c = Node::Image(Hash::from_string("f985573a7735881f58c8679dcd3a062c"));
    let c = Node::new(Box::new(c), 
        MagickCommand { args: vec![
            String::from("monochrome")
        ] },
        Hash::from_string("54f85854ca6d77d50bcd5e338e78ce15"));
    let c = Node::new(Box::new(c),
        MagickCommand { args: vec![
            String::from("crop"),
            String::from("100x100"),
        ] },
        Hash::from_string("e330efab74317d4b98eb30b03df73fa6"));
    c.materialize(Path::new("../meme-example.png"), &|s| println!("LOG: {}", s));
}
