use std::{cell::RefCell, rc::Rc, borrow::{BorrowMut, Borrow}, ops::Range, process::Command, fs::{File, self}, iter::Enumerate, path::{Path, PathBuf}, fmt::{Display, Write}};
use std::fmt::format;
use sha256::{self, digest, digest_file, try_digest};

type Sha256 = [u8; 32];

const INTER_STEPS_PATH: &str = "inter";

#[derive(Clone, Copy)]
struct Hash {
    pub sha256: Sha256,
}

impl Hash {
    fn new(path: &Path) -> Hash {
        let mut r: Sha256 = [0; 32];
        let sha = sha256::try_digest(path).expect("Problems computing hash");
        for i in 0..32 {
            r[i] = sha.as_bytes()[i];
        }
        Hash { sha256: r }
    }

    fn from_string(sha: String) -> Hash {
        assert_eq!(sha.len(), 32);
        let mut r: Sha256 = [0; 32];
        for i in 0..32 {
            r[i] = sha.as_bytes()[i];
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
            f.write_char(c as char).expect("Error");
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

fn magick(args: &Vec<String>) {
    let mut cmd = Command::new("convert");
    print!("Inv: ");
    for arg in args {
        cmd.arg(arg);
        print!("{arg} ");
    }
    println!("");
    cmd.output().expect("Ohno");
}

fn hash_verify(expected: &Hash, actual: &Hash) {
    if !expected.eq(actual) {
        panic!("Expected: {expected}. Actual: {actual}");
    }
}

enum Node {
    Commit(Box<Node>, Action, Hash),
    Image(Hash)
}

impl Node {

    fn new(prev: Box<Node>, action: Action, hash: Hash) -> Node {
        Node::Commit(prev, action, hash)
    }

    fn materialize<TLog>(&self, path: &Path, log: &TLog) -> Hash where TLog: Fn(&str) {
        match self {
            Node::Image(hash) => {
                let h = Hash::new(path);
                hash_verify(hash, &h);
                fs::copy(path, Path::new(INTER_STEPS_PATH).join(format!("{}.png", hash))).unwrap();
                hash.clone()
            },
            Node::Commit(prev, action, hash) => {
                if (Path::new(INTER_STEPS_PATH).join(hash.to_string()).with_extension("png")).exists() {
                    log(format!("Cache for {} is found, exiting", hash).as_str());
                    return hash.clone();
                }
                let prev = prev.materialize(path, log);
                let out = Path::new(INTER_STEPS_PATH).join("tmp.png");
                let out_path = out.clone().into_os_string().into_string().unwrap();
                let inw = Path::new(INTER_STEPS_PATH).join(format!("{}.png", prev));
                let in_path = inw.clone().into_os_string().into_string().unwrap();
                match action {
                    Action::Monochrome => {
                        let v = vec![in_path.clone(), String::from("-monochrome"), out_path.clone()];
                        magick(&v);
                    },
                    Action::Crop(geo) => {
                        let v = vec![in_path.clone(), String::from("-crop"), geo.to_magick(), out_path.clone()];
                        magick(&v);
                    },
                    _ => panic!("sdfd")
                };
                let out_hash = Hash::new(out.as_path());
                hash_verify(hash, &out_hash);
                fs::rename(out_path, Path::new(INTER_STEPS_PATH).join(format!("{}.png", out_hash)));
                out_hash
            }
        }
    }
}

fn main() {
    let c = Node::Image(Hash::from_string(String::from("f985573a7735881f58c8679dcd3a062c")));
    let c = Node::new(Box::new(c), Action::Monochrome, Hash::from_string(String::from("54f85854ca6d77d50bcd5e338e78ce15")));
    let c = Node::new(Box::new(c),
        Action::Crop(Geometry { width: 100, height: 200, x_off: 300, y_off: 0 }),
        Hash::from_string(String::from("e330efab74317d4b98eb30b03df73fa6")));
    c.materialize(Path::new("../meme-example.png"), &|s| println!("LOG: {}", s));
}
