use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range};
use sha256::{self, digest};

type Sha256 = [char; 32];

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

enum Node {
    Commit(Rc<RefCell<Node>>, Action, Sha256),
    Image(Sha256)
}

impl Node {
    fn hash(self) -> String {
        match self {
            Self::Commit(_, _, sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            },
            Self::Image(sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            }
        }
    }

    fn new(prev: Rc<RefCell<Node>>, action: Action) -> Node {
        let copy = action.hash();
        let b = prev.borrow();
        let sha256 = match b {
            Self::Commit(_, _, sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            },
            Self::Image(sha256) => {
                let s: String = sha256.iter().collect();
                digest(s.as_str())
            }
        };
        let mut ret_sha = [' '; 32];
        for i in 0..32 {
            ret_sha[i] =  sha256.as_bytes()[i] as char;
        }
        Node::Commit(prev, action, ret_sha)
    }
}

fn main() {
    println!("Hello, world!");
}
