use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, hash::Hash, process::Command, fs::File, iter::Enumerate};
use sha256::{self, digest};

type Sha256 = [char; 32];

type Image = i32;

#[derive(Clone, Copy)]
struct Geometry {
    width: i32, height: i32,
    x_off: i32, y_off: i32,
}

impl Geometry {
    fn to_magick(&self) -> String {
        let mut res = String::new();
        res.push_str(self.width.to_string().as_str());
        res.push('x');
        res.push_str(self.height.to_string().as_str());
        res.push('+');
        res.push_str(self.x_off.to_string().as_str());
        res.push('x');
        res.push_str(self.y_off.to_string().as_str());
        res
    }
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
    print!("Inv: ");
    for arg in args {
        cmd.arg(arg);
        print!("{arg} ");
    }
    println!("");
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

    fn apply(&self, path: &str, out: &str) -> Result<Image, String> {
        std::fs::copy(path, out);
        let mut actions = vec![];
        let img = self.collect_actions(&mut actions);
        actions.reverse();
        for (i, action) in actions.iter().enumerate() {
            match action {
                Action::Monochrome => {
                    let v = vec![String::from(out), String::from("-monochrome"), String::from(out)];
                    magick(&v)
                },
                Action::Crop(geo) => {
                    let v = vec![String::from(out), String::from("-crop"), geo.to_magick(), String::from(out)];
                    magick(&v)
                },
                _ => panic!("sdfd")
            }
        }
        Ok(img)
    }
}

fn main() {
    let c = Node::Image(0, ['s'; 32]);
    // let c = Node::new(Box::new(c), Action::Monochrome);
    let c = Node::new(Box::new(c), Action::Crop(Geometry { width: 400, height: 400, x_off: 300, y_off: 0 }));
    c.apply("/home/goose/Pictures/meme-turing.png", "out.png").expect("");
}
