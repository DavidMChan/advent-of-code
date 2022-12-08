// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

fn main() {
    // Read the input from the first argument
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let mut grid = Vec::new();
    for line in contents.trim().lines() {
        let mut row = Vec::new();
        for c in line.trim().chars() {
            // Convert the character to a number
            row.push(c.to_digit(10).unwrap());
        }
        grid.push(row);
    }

    let mut visible_trees = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if row == 0 || col == 0 || row == grid.len() - 1 || col == grid[row].len() - 1 {
                visible_trees += 1;
                continue;
            }
            // Find any trees to the left with higher height
            if grid[row]
                .iter()
                .take(col)
                .find(|&&c| c >= grid[row][col])
                .is_none()
            {
                visible_trees += 1;
                continue;
            }
            // Find any trees to the right with higher height
            if grid[row]
                .iter()
                .skip(col + 1)
                .find(|&&c| c >= grid[row][col])
                .is_none()
            {
                visible_trees += 1;
                continue;
            }
            // Find any trees above with higher height
            if grid
                .iter()
                .take(row)
                .find(|r| r[col] >= grid[row][col])
                .is_none()
            {
                visible_trees += 1;
                continue;
            }
            // Find any trees below with higher height
            if grid
                .iter()
                .skip(row + 1)
                .find(|r| r[col] >= grid[row][col])
                .is_none()
            {
                visible_trees += 1;
                continue;
            }
        }
    }

    println!("{}", visible_trees);
}
