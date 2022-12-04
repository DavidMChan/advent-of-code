// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

fn parse_line(line: &str) -> ((usize, usize), (usize, usize)) {
    // Parse the line into a pair of tuples
    // Split on "," then on "-"

    let mut split = line.split(",");
    let start = split.next().unwrap();
    let end = split.next().unwrap();

    let mut start_split = start.split("-");
    let start_x = start_split.next().unwrap().parse::<usize>().unwrap();
    let start_y = start_split.next().unwrap().parse::<usize>().unwrap();

    let mut end_split = end.split("-");
    let end_x = end_split.next().unwrap().parse::<usize>().unwrap();
    let end_y = end_split.next().unwrap().parse::<usize>().unwrap();

    return ((start_x, start_y), (end_x, end_y));
}

fn main() {
    // Read the input file from the first arg into a string
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Split the contents into a vector of strings
    let lines: Vec<&str> = contents.split("\n").filter(|line| line.len() > 0).collect();

    let mut contained = 0;
    for ln in lines {
        let ((start_x, start_y), (end_x, end_y)) = parse_line(ln);
        // Determine if one interval is completely contained within the other
        if start_x <= end_x && end_y <= start_y || end_x <= start_x && start_y <= end_y {
            contained += 1;
        }
    }

    println!("Contained: {}", contained);
}
