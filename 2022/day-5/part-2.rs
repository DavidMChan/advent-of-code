// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

struct StackSet {
    stacks: Vec<Vec<char>>,
}

impl StackSet {
    fn new() -> StackSet {
        StackSet { stacks: Vec::new() }
    }

    fn apply_action(&mut self, from: usize, to: usize, num: usize) {
        // Move num items from stack from to stack to

        let mut processing_stack = Vec::new();
        for _ in 0..num {
            // Pop the top item from the from stack and push it onto the to stack
            let item = self.stacks[from].pop().unwrap();
            processing_stack.push(item);
        }

        // Pop the items off the processing stack and push them onto the to stack
        while let Some(item) = processing_stack.pop() {
            self.stacks[to].push(item);
        }
    }
}

fn parse_stacks(input: &str) -> (StackSet, Vec<(usize, usize, usize)>) {
    // Parse the layout from the actions
    let mut segments = input.split("\n\n");
    let layout_str = segments.next().unwrap();
    let actions_str = segments.next().unwrap();

    // Parse the stacks
    let layout_lines = layout_str
        .split("\n")
        .filter(|line| line.len() > 0)
        .collect::<Vec<&str>>();

    let mut stacks = StackSet::new();
    for line in layout_lines.iter().rev().skip(1) {
        // We know the that line is a graphic representation of the stack layout
        for (index, c) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            // C is a chunk of 4 characters which is either "[X] " or "    ".
            if c[1] != ' ' {
                while stacks.stacks.len() <= index {
                    // We need to add a new stack
                    stacks.stacks.push(Vec::new());
                }
                // The middle character is not a space, so it's a box
                stacks.stacks[index].push(c[1]);
            }
        }
    }

    // Parse the actions as tuples of (from, to, num)
    let actions_lines = actions_str.split("\n").filter(|line| line.len() > 0);
    let mut actions = Vec::new();
    for line in actions_lines {
        // Split the line by whitespace
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;
        let num = split[1].parse::<usize>().unwrap();
        actions.push((from, to, num));
    }

    return (stacks, actions);
}

fn main() {
    // Read the input file from the first arg into a string
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let (mut stacks, actions) = parse_stacks(&contents);

    // Apply the actions to the stacks
    for (from, to, num) in actions {
        stacks.apply_action(from, to, num);
    }

    // Print the top of each stack without newliens
    for stack in stacks.stacks {
        print!("{}", stack[stack.len() - 1]);
    }
}
