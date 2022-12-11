// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

fn main() {
    // Read the input file from the first argument to a string
    let filename = env::args().nth(1).expect("No input file given");
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut register_value = 1;
    let mut cycle_index = 1;
    let mut signal_strength = 0;
    let mut pixels = Vec::new();

    for line in contents.lines().filter(|line| line.len() > 0) {
        let op = line.split(" ").nth(0).unwrap();
        match op {
            "addx" => {
                // Parse the argument
                let arg = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
                // Takes two cycles to execute
                if cycle_index == 20 || (cycle_index > 20 && (cycle_index - 20) % 40 == 0) {
                    // Add to the signal strength
                    signal_strength += cycle_index * register_value;
                }
                if ((pixels.len() as i32 % 40) - register_value).abs() < 2 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle_index += 1;
                if cycle_index == 20 || (cycle_index > 20 && (cycle_index - 20) % 40 == 0) {
                    // Add to the signal strength
                    signal_strength += cycle_index * register_value;
                }
                if ((pixels.len() as i32 % 40) - register_value).abs() < 2 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle_index += 1;
                // Add the argument to the register
                register_value += arg;
            }
            "noop" => {
                // Takes one cycle to execute
                if cycle_index == 20 || (cycle_index > 20 && (cycle_index - 20) % 40 == 0) {
                    // Add to the signal strength
                    signal_strength += cycle_index * register_value;
                }
                if ((pixels.len() as i32 % 40) - register_value).abs() < 2 {
                    pixels.push('#');
                } else {
                    pixels.push('.');
                }
                cycle_index += 1;
            }
            _ => {
                // TODO: Handle invalid op
                println!("Invalid op: {}", op)
            }
        }
    }

    // Print the pixels as a 40-wide image
    for i in 0..(pixels.len() / 40) {
        println!(
            "{}",
            pixels[i * 40..(i + 1) * 40].iter().collect::<String>()
        );
    }

    println!("Signal strength: {}", signal_strength);
}
