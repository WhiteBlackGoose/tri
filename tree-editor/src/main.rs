use std::{cell::RefCell, rc::Rc};

struct Geometry {
    width: i32, height: i32,
    x_off: i32, y_off: i32,
}

enum Action {
    Crop(Geometry),
    Annotate(Geometry, String)
}

enum Node {
    Commit()
}

struct Node {
    pub parent: Rc<RefCell<Node>>,
    pub action: Action,
    pub hash: String
}

impl Node {
    fn new(prev: &Node, action: Action) {
        
    }
}

fn main() {
    println!("Hello, world!");
}
