use std::env;
use std::fs;

fn main() {
    // Load the file from the first argument
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Each line is an integer, so parse it
    let numbers: Vec<i32> = contents.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Count the number of increases in the file
    let mut  num_increases = 0;
    for i in 1..numbers.len() {
        if numbers[i-1] < numbers[i] {
            num_increases += 1;
        }
    }

    println!("Number of increases: {}", num_increases);
}
