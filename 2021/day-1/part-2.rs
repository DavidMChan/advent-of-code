use std::env;
use std::fs;

fn main() {
    // Load the file from the first argument
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Each line is an integer, so parse it
    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Count the number of increases in the file
    let mut num_increases = 0;
    let mut prev_window_sum = -1;
    let mut window_sum = -1;
    for i in 2..numbers.len() {
        if prev_window_sum < 0 {
            prev_window_sum = numbers[i - 2] + numbers[i - 1] + numbers[i];
        } else {
            window_sum = numbers[i - 2] + numbers[i - 1] + numbers[i];
            if window_sum > prev_window_sum {
                num_increases += 1;
            }
            prev_window_sum = window_sum;
        }
    }

    println!("Number of increases: {}", num_increases);
}
