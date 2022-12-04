// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use std::env;
use std::fs;

fn get_priority(item: char) -> usize {
    // Lower case letters get a priority of 1-26
    // Upper case letters get a priority of 27-52

    let priority = if item.is_lowercase() {
        item as usize - 96
    } else {
        item as usize - 38
    };

    return priority;
}

fn main() {
    // Read in the file from the first arg
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut sum = 0;
    let mut idx = 0;
    let mut hash_set = HashSet::new();
    for line in contents.lines() {
        if idx % 3 == 0 {
            // Add the first line to the set
            for item in line.chars() {
                hash_set.insert(item);
            }
        } else {
            // Remove the items that are not in the set
            hash_set.retain(|&item| line.contains(item));
        }
        if idx % 3 == 2 {
            // Get the priority of the last remaining character
            // (There should only be one item)
            sum += get_priority(*hash_set.iter().next().unwrap());
            hash_set.clear();
        }
        idx += 1;
    }

    println!("Sum of priorities: {} ({})", sum, idx);
}
