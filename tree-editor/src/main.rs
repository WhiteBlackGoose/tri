use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, hash::Hash, process::Command};
use sha256::{self, digest};

type Sha256 = [char; 32];

type Image = i32;

#[derive(Clone, Copy)]
struct Geometry {
    width: i32, height: i32,
    x_off: i32, y_off: i32,
}

#[derive(Clone, Copy)]
enum Action {
    Crop(Geometry),
    Monochrome
}

impl Action {
    fn hash(self) -> String {
        match self {
            Self::Crop(Geometry) => String::from("aaa"),
            Self::Monochrome => String::from("bbb")
        }
    }
}

fn magick(args: &Vec<String>) {
    let mut cmd = Command::new("convert");
    for arg in args {
        cmd.arg(arg);
    }
    cmd.arg("out.png");
    cmd.output().expect("Ohno");
}

enum Node {
    Commit(Box<Node>, Action, Sha256),
    Image(Image, Sha256)
}

impl Node {
    fn hash(&self) -> String {
        match self {
            Self::Commit(_, _, sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            },
            Self::Image(_, sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            }
        }
    }

    fn new(prev: Box<Node>, action: Action) -> Node {
        let copy = action.hash();
        let b: &Node = prev.borrow();
        let mut ret_sha = [' '; 32];
        for i in 0..32 {
            ret_sha[i] =  b.hash().as_bytes()[i] as char;
        }
        Node::Commit(prev, action, ret_sha)
    }

    fn collect_actions(&self, actions: &mut Vec<Action>) -> Image {
        match self {
            Node::Image(img, _) => img.clone(),
            Node::Commit(prev, action, _) => {
                actions.push(action.clone());
                let prevb: &Node = prev.borrow();
                prevb.collect_actions(actions)
            }
        }
    }

    fn apply(&self, path: &str) -> Result<Image, String> {
        let mut actions = vec![];
        let img = self.collect_actions(&mut actions);
        for action in actions {
            match action {
                Action::Monochrome => {
                    let v = vec![String::from(path), String::from("-monochrome")];
                    magick(&v)
                },
                _ => panic!("sdfd")
            }
        }
        Ok(img)
    }
}

fn main() {
    let img = Node::Image(0, ['s'; 32]);
    let c1 = Node::new(Box::new(img), Action::Monochrome);
    c1.apply("/home/goose/Pictures/meme-turing.png").expect("");
}
