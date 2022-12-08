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

    let mut viewing_distance = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            // Find the first tree to the left with higher height and calculate the distance
            let vl = match grid[row]
                .iter()
                .take(col)
                .rev()
                .position(|&c| c >= grid[row][col])
            {
                Some(vl) => vl + 1,
                None => col,
            };

            // Find the first tree to the right with higher height and calculate the distance
            let vr = match grid[row]
                .iter()
                .skip(col + 1)
                .position(|&c| c >= grid[row][col])
            {
                Some(vr) => vr + 1,
                None => grid[row].len() - col - 1,
            };

            // Find the first tree above with higher height and calculate the distance
            let vu = match grid
                .iter()
                .take(row)
                .rev()
                .position(|r| r[col] >= grid[row][col])
            {
                Some(vu) => vu + 1,
                None => row,
            };

            // Find the first tree below with higher height and calculate the distance
            let vd = match grid
                .iter()
                .skip(row + 1)
                .position(|r| r[col] >= grid[row][col])
            {
                Some(vd) => vd + 1,
                None => grid.len() - row - 1,
            };

            // Print the distance to the closest tree in each direction
            println!(
                "({},{}) -- {} {} {} {} -- {}",
                row,
                col,
                vu,
                vl,
                vr,
                vd,
                vl * vr * vu * vd
            );

            // Set the viewing distance to the maximum of the current viewing distance and the current tree's viewing distance
            viewing_distance = viewing_distance.max(vl * vr * vu * vd);
        }
    }

    println!("Max: {}", viewing_distance);
}
