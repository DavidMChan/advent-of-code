// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

fn main() {
    // Read the input file from the first arg into a string
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Put the first three characters into a vector
    let mut chars: Vec<char> = Vec::new();
    for c in contents.chars().take(3) {
        chars.push(c);
    }

    // For each character after the third, add the character to the vector, and
    // if the vector is made up of unique characters, return the index, otherwise
    // remove the first character and continue
    let mut index = 4;
    for c in contents.chars().skip(3) {
        chars.push(c);
        if chars.len() == chars.iter().collect::<std::collections::HashSet<_>>().len() {
            break;
        }
        chars.remove(0);
        index += 1;
    }

    // Print out the index
    println!("{}", index);
}
