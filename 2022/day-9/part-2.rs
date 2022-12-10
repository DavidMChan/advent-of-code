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

fn move_tail(head_location: (i32, i32), tail_location: (i32, i32)) -> (i32, i32) {
    // This function needs to work a bit differently in part 2, since we don't know what direction the tail is moving
    // in.

    let mut new_tail_location = tail_location;

    // If the head is touching the tail, then we don't need to move the tail
    if is_touching(&head_location, &new_tail_location) {
        return new_tail_location;
    }

    // If the head is directly above the tail, then we need to move the tail up
    if head_location.0 == tail_location.0 && head_location.1 > tail_location.1 {
        new_tail_location.1 += 1;
        return new_tail_location;
    }
    // If the head is directly below the tail, then we need to move the tail down
    if head_location.0 == tail_location.0 && head_location.1 < tail_location.1 {
        new_tail_location.1 -= 1;
        return new_tail_location;
    }
    // If the head is directly to the right of the tail, then we need to move the tail right
    if head_location.1 == tail_location.1 && head_location.0 > tail_location.0 {
        new_tail_location.0 += 1;
        return new_tail_location;
    }
    // If the head is directly to the left of the tail, then we need to move the tail left
    if head_location.1 == tail_location.1 && head_location.0 < tail_location.0 {
        new_tail_location.0 -= 1;
        return new_tail_location;
    }

    // Handle the diagonal cases
    if head_location.0 > tail_location.0 && head_location.1 > tail_location.1 {
        // Head is to the top right of the tail
        new_tail_location.0 += 1;
        new_tail_location.1 += 1;
        return new_tail_location;
    }
    if head_location.0 > tail_location.0 && head_location.1 < tail_location.1 {
        // Head is to the bottom right of the tail
        new_tail_location.0 += 1;
        new_tail_location.1 -= 1;
        return new_tail_location;
    }
    if head_location.0 < tail_location.0 && head_location.1 > tail_location.1 {
        // Head is to the top left of the tail
        new_tail_location.0 -= 1;
        new_tail_location.1 += 1;
        return new_tail_location;
    }
    if head_location.0 < tail_location.0 && head_location.1 < tail_location.1 {
        // Head is to the bottom left of the tail
        new_tail_location.0 -= 1;
        new_tail_location.1 -= 1;
        return new_tail_location;
    }

    panic!("Invalid tail location");
}

fn print_locations(locations: &Vec<(i32, i32)>) {
    for yy in -10..10 {
        let y = -yy;
        for x in -10..10 {
            if locations.contains(&(x, y)) {
                // Get the index of the location
                let index = locations.iter().position(|&r| r == (x, y)).unwrap();
                if index == 0 {
                    print!("H");
                } else {
                    print!("{}", index);
                }
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    // Read input file from first argument
    let filename = env::args().nth(1).expect("No input file given");
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut tail_visited_cells = HashSet::new();
    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push((0, 0));
    }

    tail_visited_cells.insert(knots[knots.len() - 1]);
    for line in contents.lines().filter(|line| line.len() > 0) {
        let mut direction = line.chars().nth(0).unwrap();
        let dist_str = &line[1..].trim();
        let distance = dist_str.parse::<i32>().unwrap();

        // Move the head in the direction
        for _ in 0..distance {
            match direction {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => panic!("Invalid direction"),
            }
            // Print the location of the head
            for i in 1..knots.len() {
                // Move the node as if it's a tail
                if !is_touching(&knots[i - 1], &knots[i]) {
                    knots[i] = move_tail(knots[i - 1], knots[i]);
                }
                if i == knots.len() - 1 {
                    // Add the new tail to the visited cells
                    tail_visited_cells.insert(knots[i]);
                }
            }
        }
    }

    // Print the length of the visited cells
    println!("{}", tail_visited_cells.len());
}
