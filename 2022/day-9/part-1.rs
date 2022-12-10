// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use std::env;
use std::fs;

fn is_touching(a: &(i32, i32), b: &(i32, i32)) -> bool {
    let (ax, ay) = a;
    let (bx, by) = b;
    // Check if the two points are touching (i.e. distance of 1) in any direction or diagonal
    (ax - bx).abs() <= 1 && (ay - by).abs() <= 1
}

fn main() {
    // Read input file from first argument
    let filename = env::args().nth(1).expect("No input file given");
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut tail_visited_cells = HashSet::new();
    let mut head_location = (0, 0);
    let mut tail_location = (0, 0);

    tail_visited_cells.insert(tail_location);

    for line in contents.lines().filter(|line| line.len() > 0) {
        let direction = line.chars().nth(0).unwrap();
        let dist_str = &line[1..].trim();
        let distance = dist_str.parse::<i32>().unwrap();

        // Move the head in the direction
        for _ in 0..distance {
            match direction {
                'R' => {
                    head_location.0 += 1;
                    if !is_touching(&head_location, &tail_location) {
                        // Either we need to move the tail diagonally, or to the right
                        if tail_location.1 < head_location.1 {
                            tail_location.1 += 1;
                        } else if tail_location.1 > head_location.1 {
                            tail_location.1 -= 1;
                        }
                        tail_location.0 += 1;
                        tail_visited_cells.insert(tail_location);
                    }
                }
                'L' => {
                    head_location.0 -= 1;
                    if !is_touching(&head_location, &tail_location) {
                        // Either we need to move the tail diagonally, or to the left
                        if tail_location.1 < head_location.1 {
                            tail_location.1 += 1;
                        } else if tail_location.1 > head_location.1 {
                            tail_location.1 -= 1;
                        }
                        tail_location.0 -= 1;
                        tail_visited_cells.insert(tail_location);
                    }
                }
                'U' => {
                    head_location.1 += 1;
                    if !is_touching(&head_location, &tail_location) {
                        // Either we need to move the tail diagonally, or up
                        if tail_location.0 < head_location.0 {
                            tail_location.0 += 1;
                        } else if tail_location.0 > head_location.0 {
                            tail_location.0 -= 1;
                        }
                        tail_location.1 += 1;
                        tail_visited_cells.insert(tail_location);
                    }
                }
                'D' => {
                    head_location.1 -= 1;
                    if !is_touching(&head_location, &tail_location) {
                        // Either we need to move the tail diagonally, or down
                        if tail_location.0 < head_location.0 {
                            tail_location.0 += 1;
                        } else if tail_location.0 > head_location.0 {
                            tail_location.0 -= 1;
                        }
                        tail_location.1 -= 1;
                        tail_visited_cells.insert(tail_location);
                    }
                }
                _ => panic!("Invalid direction"),
            }

            // Assert that the head and the tail are touching
            assert!(is_touching(&head_location, &tail_location));
        }
    }

    // Print the length of the visited cells
    println!("{}", tail_visited_cells.len());
}
