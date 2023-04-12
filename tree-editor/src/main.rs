use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, process::Command, fs::{File, self}, iter::Enumerate, path::{Path, PathBuf}, fmt::{Display, Write}};
use sha256::{self, digest, digest_file, try_digest};

type Sha256 = [char; 32];

const INTER_STEPS_PATH: &str = "inter";

#[derive(Clone, Copy)]
struct Hash {
    pub sha256: [char; 32],
}

impl Hash {
    fn new(path: &Path) -> Hash {
        let r: Sha256 = [' '; 32];
        let sha = sha256::try_digest(path).expect("Problems computing hash");
        for i in 0..32 {
            r[i] = sha.as_bytes()[i] as char;
        }
        Hash { sha256: r }
    }

    fn from_string(sha: String) -> Hash {
        assert_eq!(sha.len(), 32);
        let mut r: Sha256 = [' '; 32];
        for i in 0..32 {
            r[i] = sha.as_bytes()[i] as char;
        }
        Hash { sha256: r }
    }

    fn eq(&self, other: &Hash) -> bool {
        for (a, b) in self.sha256.iter().zip(other.sha256) {
            if *a != b {
                return false;
            }
        }
        true
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.sha256 {
            f.write_char(c);
        }
        Ok(())
    }
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
    Commit(Box<Node>, Action, Hash),
    Image(Hash)
}

impl Node {
    fn hash(&self) -> String {
        match self {
            Self::Commit(_, _, hash) => {
                let s: String = hash.sha256.iter().collect();
                s
            },
            Self::Image(hash) => {
                let s: String = hash.sha256.iter().collect();
                s
            }
        }
    }

    fn new(prev: Box<Node>, action: Action, hash: Hash) -> Node {
        let b: &Node = prev.borrow();
        Node::Commit(prev, action, hash)
    }

    fn materialize(&self, path: &Path) -> Hash {
        match self {
            Node::Image(hash) => {
                let h = Hash::new(path);
                if !h.eq(hash) {
                    panic!("Expected: {hash}. Actual: {h}");
                }
                fs::copy(path, Path::new(INTER_STEPS_PATH).join("{hash}.png"));
                hash.clone()
            },
            Node::Commit(prev, action, _) => {
                let prev = prev.materialize(path);
                let out = Path::new(INTER_STEPS_PATH).join("tmp.png");
                let out_path = out.into_os_string().into_string().unwrap();
                match action {
                    Action::Monochrome => {
                        let v = vec![out_path, String::from("-monochrome"), out_path];
                        magick(&v);
                    },
                    Action::Crop(geo) => {
                        let v = vec![out_path, String::from("-crop"), geo.to_magick(), out_path];
                        magick(&v);
                    },
                    _ => panic!("sdfd")
                };
                Hash::new(out.as_path())
            }
        }
    }
}

fn main() {
    let c = Node::Image(['s'; 32]);
    // let c = Node::new(Box::new(c), Action::Monochrome);
    let c = Node::new(Box::new(c), Action::Crop(Geometry { width: 400, height: 400, x_off: 300, y_off: 0 }));
    c.materialize("/home/goose/Pictures/meme-turing.png");
}
