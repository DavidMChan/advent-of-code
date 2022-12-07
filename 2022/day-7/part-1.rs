// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

struct FileSystem {
    name: String,
    directories: Vec<usize>,
    files: Vec<(String, usize)>,
    parent: usize,
}

impl FileSystem {
    pub fn new(name: String, parent: usize) -> FileSystem {
        FileSystem {
            name,
            directories: Vec::new(),
            files: Vec::new(),
            parent,
        }
    }
}

fn get_directory_size(filesystems: &Vec<FileSystem>, index: usize) -> usize {
    let mut size = 0;
    for (_, file_size) in &filesystems[index].files {
        size += file_size;
    }
    for directory in &filesystems[index].directories {
        size += get_directory_size(filesystems, *directory);
    }
    size
}

fn main() {
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Get each of the lines
    let lines: Vec<&str> = contents.split("\n").filter(|line| line.len() > 0).collect();

    let mut filesystems = Vec::new();

    // Add the root filesystem
    filesystems.push(FileSystem::new("/".to_string(), 0));
    let mut cwd = 0;

    // Parse each line
    let mut line_num = 0;
    for line in lines {
        // Command is "$ cd" or "$ ls" or a file descriptor
        if line.starts_with("$") {
            // Get the command
            let command = line.split(" ").nth(1).unwrap();
            match command {
                "cd" => {
                    // Get the directory name
                    let directory = line.split(" ").nth(2).unwrap();
                    if directory.eq("..") {
                        // Go up a directory
                        cwd = filesystems[cwd].parent;
                        continue;
                    } else if directory.eq("/") {
                        cwd = 0;
                    } else {
                        // Find the directory in the current working directory
                        let index = filesystems[cwd]
                            .directories
                            .iter()
                            .position(|&x| filesystems[x].name.eq(directory))
                            .unwrap();
                        // Set the current working directory to the index of the directory
                        cwd = filesystems[cwd].directories[index];
                    }
                }
                "ls" => {
                    // Ignore this - since it doesn't matter.
                }
                _ => panic!("Unknown command"),
            }
        } else {
            // File descriptor
            let (size, name) = line.split_once(" ").unwrap();
            if size.eq("dir") {
                // Check to see if the directory already exists
                if filesystems[cwd]
                    .directories
                    .iter()
                    .any(|&x| filesystems[x].name.eq(name))
                {
                    continue;
                }
                // Create a new directory
                filesystems.push(FileSystem::new(name.to_string(), cwd));
                // Add the index of the directory to the current working directory
                let index = filesystems.len() - 1;
                filesystems[cwd].directories.push(index);
            } else {
                // Check to see if the file already exists
                if filesystems[cwd].files.iter().any(|x| x.0.eq(name)) {
                    continue;
                }
                // Add the file to the current working directory
                // Print the file
                filesystems[cwd]
                    .files
                    .push((name.to_string(), size.parse::<usize>().unwrap()));
            }
        }
        line_num += 1;
    }

    // Get all of the directories with size < 100000
    let mut small_dirs = Vec::new();
    for (i, filesystem) in filesystems.iter().enumerate() {
        let size = get_directory_size(&filesystems, i);
        if size < 100000 {
            small_dirs.push((i, size));
        }
    }

    // Print the sum of the sizes of all of the directories with size < 100000
    let mut total = 0;
    for (_, size) in small_dirs {
        total += size;
    }

    println!("{}", total);
}
