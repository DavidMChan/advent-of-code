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
                    println!(
                        "Cycle {}: strength: {}, rval: {}",
                        cycle_index, signal_strength, register_value
                    );
                }
                cycle_index += 1;
                if cycle_index == 20 || (cycle_index > 20 && (cycle_index - 20) % 40 == 0) {
                    // Add to the signal strength
                    signal_strength += cycle_index * register_value;
                    println!(
                        "Cycle {}: strength: {}, rval: {}",
                        cycle_index, signal_strength, register_value
                    );
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

                    println!(
                        "Cycle {}: strength: {}, rval: {}",
                        cycle_index, signal_strength, register_value
                    );
                }
                cycle_index += 1;
            }
            _ => {
                // TODO: Handle invalid op
                println!("Invalid op: {}", op)
            }
        }
    }

    println!("Signal strength: {}", signal_strength);
}
