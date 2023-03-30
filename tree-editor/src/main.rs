use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, process::Command, fs::{File, self}, iter::Enumerate, path::Path};
use sha256::{self, digest, digest_file, try_digest};

type Sha256 = [char; 32];

struct Hash {
    pub sha256: [char; 32],
}

impl Hash {
    fn new(path: &str) -> Hash {
        let r: Sha256 = [' '; 32];
        let sha = sha256::try_digest(Path::new(path)).expect("Problems computing hash");
        for i in 0..32 {
            r[i] = sha.as_bytes()[i] as char;
        }
        Hash { sha256: r }
    }
    fn eq(&self, other: &Hash) -> bool {
    }
}


fn hasheq(h1: &Sha256, h2: &Sha256) -> bool {
// fn eq(h1: &[char; 32], h2: &[char; 32]) -> bool {
    for (a, b) in h1.iter().zip(h2) {
        if a != b {
            return false;
        }
    }
    true
}

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
    Image(Sha256)
}

impl Node {
    fn hash(&self) -> String {
        match self {
            Self::Commit(_, _, sha256) => {
                let s: String = sha256.iter().collect();
                s
            },
            Self::Image(sha256) => {
                let s: String = sha256.iter().collect();
                s
            }
        }
    }

    fn new(prev: Box<Node>, action: Action) -> Node {
        let b: &Node = prev.borrow();
        let mut ret_sha = [' '; 32];
        Node::Commit(prev, action, ret_sha)
    }

    fn materialize(&self, path: &str) -> Sha256 {
        match self {
            Node::Image(hash) => {
                let h = chash(path);
                if !hasheq(&h, hash) {
                    panic!("Expected: {hash}. Actual: {h}");
                }
                fs::copy(path, to)
                hash
            },
            Node::Commit(prev, action, _) => {
                actions.push(action.clone());
                let prevb: &Node = prev.borrow();
                prevb.collect_actions(actions)
            }
        }
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
