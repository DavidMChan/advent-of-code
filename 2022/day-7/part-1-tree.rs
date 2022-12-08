// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

struct Node {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
    files: Vec<(String, usize)>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    // Construct a new node
    fn new(name: String, parent: Option<Rc<RefCell<Node>>>) -> Node {
        Node {
            name,
            children: Vec::new(),
            files: Vec::new(),
            parent,
        }
    }

    // Add a file to the node
    pub fn add_file(&mut self, name: &str, size: usize) {
        self.files.push((name.to_string(), size));
    }

    pub fn get_directory(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        for child in self.children.iter() {
            if child.borrow().name.eq(name) {
                return Some(child.clone());
            }
        }
        return None;
    }

    // Compute the size of the node
    pub fn size(&self) -> usize {
        let mut size = 0;
        for (_, file_size) in &self.files {
            size += file_size;
        }
        for child in &self.children {
            size += child.borrow().size();
        }
        size
    }
}

fn add_directory(cwd: &Rc<RefCell<Node>>, name: &str) -> Rc<RefCell<Node>> {
    let mut cwd_mut = cwd.borrow_mut();
    let new_dir = Rc::new(RefCell::new(Node::new(name.to_string(), Some(cwd.clone()))));
    cwd_mut.children.push(new_dir.clone());
    new_dir
}

// Traverse the tree and pretty print it with tabs
fn traverse(node: &Rc<RefCell<Node>>, depth: usize) {
    let dir = node.borrow();
    for _ in 0..depth {
        print!("\t");
    }
    println!("d {} {}", dir.name, dir.size());
    for child in &dir.children {
        traverse(child, depth + 1);
    }
    for (filename, size) in &dir.files {
        for _ in 0..depth + 1 {
            print!("\t");
        }
        println!("f {} {}", filename, size);
    }
}

fn main() {
    // Read the input file from the command line
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Create a new tree
    let root = Rc::new(RefCell::new(Node::new("/".to_string(), None)));
    let mut cwd = root.clone();

    // Parse the input files with commands
    for line in contents.lines() {
        let command = line.split_whitespace().next().unwrap();
        match command {
            "ls" => {
                let dir = cwd.borrow();
                for child in &dir.children {
                    println!("d {} {}", child.borrow().name, child.borrow().size());
                }
                for (filename, size) in &dir.files {
                    println!("f {} {}", filename, size);
                }
            }
            "mkdir" => {
                let dir_name = line.split_whitespace().nth(1).unwrap();
                add_directory(&cwd, dir_name);
            }
            "touch" => {
                let filename = line.split_whitespace().nth(1).unwrap();
                let size = line
                    .split_whitespace()
                    .nth(2)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                cwd.borrow_mut().add_file(filename, size);
            }
            "cd" => {
                let dir_name = line.split_whitespace().nth(1).unwrap();
                if dir_name.eq("..") {
                    let parent = cwd.borrow().parent.clone().unwrap();
                    cwd = parent;
                } else if dir_name.eq("/") {
                    cwd = root.clone();
                } else {
                    let target = cwd.borrow().get_directory(dir_name);
                    if target.is_some() {
                        cwd = target.unwrap();
                    } else {
                        println!("Directory not found: {}", dir_name);
                    }
                }
            }
            _ => {}
        }
    }

    // Print the tree
    traverse(&root, 0);

    // Print the size of the tree
    println!("Size of tree: {}", cwd.borrow().size());
}
