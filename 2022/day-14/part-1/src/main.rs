// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Grid of characters
    let MAX_X = 200;
    let MAX_Y = 200;
    let OFFSET = 400; // This is just because it makes it a bit easier to visualize
    let mut grid = vec![vec!['.'; MAX_X]; MAX_Y];

    // Parse the input to set up the grid
    for line in contents.lines().filter(|line| line.len() > 0) {
        // Parse the line into a sequence of instructions
        let mut instructions = line.split("->").map(|s| s.trim());

        // For each pair of instructions, draw code on the grid
        for (source, dest) in instructions.clone().zip(instructions.skip(1)) {
            // Draw a "#" on the grid from the source to the destination
            let mut source_coords = source
                .split(',')
                .map(|s| s.trim().parse::<usize>().unwrap());
            let mut dest_coords = dest.split(',').map(|s| s.trim().parse::<usize>().unwrap());
            let x1 = source_coords.next().unwrap() - OFFSET;
            let y1 = source_coords.next().unwrap();
            let x2 = dest_coords.next().unwrap() - OFFSET;
            let y2 = dest_coords.next().unwrap();

            println!("({}, {}) -> ({}, {})", x1, y1, x2, y2);
            if x1 == x2 {
                println!("Vertical line");
                // Vertical line
                for y in y1..=y2 {
                    grid[y][x1] = '#';
                }
                for y in y2..=y1 {
                    grid[y][x1] = '#';
                }
            } else if y1 == y2 {
                println!("Horizontal line");
                // Horizontal line
                for x in x1..=x2 {
                    grid[y1][x] = '#';
                }
                for x in x2..=x1 {
                    grid[y1][x] = '#';
                }
            } else {
                panic!("Invalid line");
            }
        }
    }

    // Create a frame of the animation
    let mut base_image: RgbImage = ImageBuffer::new(MAX_X as u32, MAX_Y as u32);
    for (x, y, pixel) in base_image.enumerate_pixels_mut() {
        let c = grid[y as usize][x as usize];
        if c == '#' {
            *pixel = Rgb([0, 0, 0]);
        } else if c == 'X' {
            *pixel = Rgb([0, 255, 0]);
        } else if c == 'o' {
            *pixel = Rgb([255, 0, 0]);
        } else {
            *pixel = Rgb([255, 255, 255]);
        }
    }
    base_image.save(format!("GRID.png")).unwrap();

    // Generate sand until some sand falls off the bottom
    let mut sand_source = (500 - OFFSET, 0);
    let mut sand_falling = false;
    let mut current_sand_location = sand_source;
    let mut units_at_rest = 0;
    let mut frame = 0;
    while true {
        if !sand_falling {
            // Create a new piece of sand
            grid[sand_source.1][sand_source.0] = 'o';
            sand_falling = true;
            current_sand_location = sand_source;
        } else {
            // Move the sand down
            let (x, y) = current_sand_location;
            if y + 1 >= MAX_Y {
                // Note: This can cause some issues, if we're not careful
                // Sand has fallen off the bottom
                break;
            }
            if grid[y + 1][x] == '.' {
                // Move the sand down
                grid[y][x] = '.';
                grid[y + 1][x] = 'o';
                current_sand_location = (x, y + 1);
            } else if grid[y + 1][x] == '#' || grid[y + 1][x] == 'o' {
                // The sand is blocked, so we need to move it down + left or down + right
                if grid[y + 1][x - 1] == '.' {
                    // Move the sand down + left
                    grid[y][x] = '.';
                    grid[y + 1][x - 1] = 'o';
                    current_sand_location = (x - 1, y + 1);
                } else if grid[y + 1][x + 1] == '.' {
                    // Move the sand down + right
                    grid[y][x] = '.';
                    grid[y + 1][x + 1] = 'o';
                    current_sand_location = (x + 1, y + 1);
                } else {
                    // The sand is blocked on both sides, so it's at rest
                    sand_falling = false;
                    units_at_rest += 1;
                }
            }
        }
        // Create a frame of the animation
        // let mut image: RgbImage = ImageBuffer::new(MAX_X as u32, MAX_Y as u32);
        // for (x, y, pixel) in image.enumerate_pixels_mut() {
        //     let c = grid[y as usize][x as usize];
        //     if c == '#' {
        //         *pixel = Rgb([0, 0, 0]);
        //     } else if c == 'o' {
        //         *pixel = Rgb([255, 0, 0]);
        //     } else {
        //         *pixel = Rgb([255, 255, 255]);
        //     }
        // }
        // image.save(format!("frames/image-{}.png", frame)).unwrap();
        // frame += 1;
    }

    // Count the number of units at rest
    println!("Units at rest: {}", units_at_rest);
    println!("Frame: {}", frame);
}
