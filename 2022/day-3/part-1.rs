// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

struct Rucksack {
    items_lh: Vec<char>,
    items_rh: Vec<char>,
}

impl Rucksack {
    fn new(contents: &str) -> Rucksack {
        // First half of the contents string is the left hand items
        let items_lh: Vec<char> = contents.chars().take(contents.len() / 2).collect();

        // Second half of the contents string is the right hand items
        let items_rh: Vec<char> = contents.chars().skip(contents.len() / 2).collect();

        return Rucksack { items_lh, items_rh };
    }

    fn get_shared_priority(&self) -> usize {
        // Find objects in left and right half
        let mut shared_items: Vec<char> = Vec::new();
        for item in &self.items_lh {
            if self.items_rh.contains(item) && !shared_items.contains(item) {
                shared_items.push(*item);
            }
        }

        // Get the priority for each
        let mut shared_priority: Vec<usize> = Vec::new();
        for item in shared_items {
            shared_priority.push(self.get_priority(item));
        }

        // Return the sum of the priorities
        return shared_priority.iter().sum();
    }

    fn get_priority(&self, item: char) -> usize {
        // Lower case letters get a priority of 1-26
        // Upper case letters get a priority of 27-52

        let priority = if item.is_lowercase() {
            item as usize - 96
        } else {
            item as usize - 38
        };

        return priority;
    }
}

fn main() {
    // Read the input file from the first arg into a string
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Split the file based on lines
    let lines = contents.split("\n").filter(|line| line.len() > 0);

    // Construct a rucksack for each line
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for line in lines {
        rucksacks.push(Rucksack::new(line));
    }

    // Get the shared priority for each rucksack
    let mut shared_priority: Vec<usize> = Vec::new();
    for rucksack in rucksacks {
        shared_priority.push(rucksack.get_shared_priority());
    }

    // Print out the sum of the shared priorities
    println!("{}", shared_priority.iter().sum::<usize>());
}
