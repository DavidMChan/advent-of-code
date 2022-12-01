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

    // Split the file based on pairs of newline characters
    let mut lines = contents.split("\n\n");

    // Each group of lines has a list of integers separated by newlines
    // We can split each group into a vector of strings
    let mut groups: Vec<Vec<&str>> = Vec::new();
    while let Some(line) = lines.next() {
        groups.push(line.split_whitespace().collect());
    }

    // Get the sum of each group
    let mut sum: Vec<usize> = Vec::new();
    for group in groups {
        let mut group_sum = 0;
        for line in group {
            group_sum += line.parse::<usize>().unwrap();
        }
        sum.push(group_sum);
    }

    // Print the sum of the largest three groups
    let mut sum = sum;
    sum.sort();
    sum.reverse();
    println!("{}", sum[0] + sum[1] + sum[2]);
}
