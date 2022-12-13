// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;
use std::io;

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for v in row {
            print!("{}", v);
        }
        println!();
    }
}

fn print_grid_i(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for v in row {
            print!("{} ", v);
        }
        println!();
    }
}

fn get_neighbors(loc: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let (x, y) = loc;
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < grid.len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < grid[0].len() - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

fn main() {
    // Read in the grid of characters from the input file in the first arg
    let filename = env::args().nth(1).expect("No filename given");
    let input = fs::read_to_string(filename).expect("Error reading file");

    // Parse the input into a vector of strings
    let mut lines: Vec<&str> = input.lines().filter(|line| line.len() > 0).collect();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut exp: Vec<Vec<i32>> = Vec::new();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        let mut exp_row: Vec<i32> = Vec::new();
        for c in line.trim().chars() {
            // Check if it's the start or end location
            if c == 'S' {
                start = (grid.len(), row.len());
                row.push('a');
            } else if c == 'E' {
                end = (grid.len(), row.len());
                row.push('z');
            } else {
                row.push(c);
            }
            exp_row.push(-1);
        }
        grid.push(row);
        exp.push(exp_row);
    }

    print_grid(&grid);
    // Print the srtat and end
    println!("Start: ({}, {})", start.0, start.1);
    println!("End: ({}, {})", end.0, end.1);

    // Do a full BFS to find the shortest path

    let mut queue: Vec<(usize, usize)> = Vec::new();
    queue.push(start);

    let mut steps = 0;
    while !queue.is_empty() {
        let current = queue.remove(0);
        // Check if we've found the end
        if current == end {
            println!("Found the end!");
            break;
        }
        if exp[current.0][current.1] != -1 {
            continue;
        }
        // Set the current position to the current step
        exp[current.0][current.1] = steps;
        for n in get_neighbors(current, &grid) {
            if ((grid[n.0][n.1] as i32) - (grid[current.0][current.1] as i32)) <= 1 {
                queue.push(n);
            }
        }
        steps += 1;
    }

    // Print the grid of steps
    // print_grid_i(&exp);
    println!("Steps: {}", steps);

    // Extract the shortest path by backtracking
    let mut path: Vec<(usize, usize)> = Vec::new();
    path.push(end);

    println!("Path:");
    while !path.last().unwrap().eq(&start) {
        // for p in path.iter().rev() {
        //     print!("({}, {}) ", p.0, p.1);
        // }
        // println!();

        // Get the smallest legal neighbor
        let mut smallest = (grid.len(), grid[0].len());
        let mut smallest_value = 1000000;
        for n in get_neighbors(*path.last().unwrap(), &grid) {
            // Always go to the start if we can
            if n.eq(&start) {
                println!("Done!!!");
                smallest = n;
                break;
            }

            // println!("-- ({}, {}): {}", n.0, n.1, exp[n.0][n.1]);
            if exp[n.0][n.1] < smallest_value
                && exp[n.0][n.1] > 0
                && ((grid[path.last().unwrap().0][path.last().unwrap().1] as i32)
                    - grid[n.0][n.1] as i32)
                    <= 1
            {
                smallest = n;
                smallest_value = exp[n.0][n.1];
                // Print the value
                // println!("({}, {}): {}", n.0, n.1, exp[n.0][n.1]);
            }
        }

        path.push(smallest);
    }

    // Print the path
    // println!("Path:");
    // for p in path.iter().rev() {
    //     print!("({}, {}) ", p.0, p.1);
    // }
    // println!();
    // Print the number of steps
    println!("Path Length: {}", path.len() - 1);
}
