// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::env;
use std::fs;

fn get_score(round: &str, possible_outcomes: &HashMap<&str, usize>) -> usize {
    // Player A can pick from A, B, C (Rock, Paper Scissors)
    // Player B can pick from X, Y, Z (Rock, Paper Scissors)

    // You can determine the value score just by looking at the last character
    let value_score = if round.ends_with("X") {
        0
    } else {
        if round.ends_with("Y") {
            3
        } else {
            6
        }
    };

    // The choice score is complicated. Get this by looking at the table

    return value_score
        + possible_outcomes
            .get(round)
            .expect(format!("Could not find round in possible outcomes: {}", round).as_str());
}

fn main() {
    // Read the input file from the first arg into a string
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Could not read file");

    // Split the file based on lines
    let lines = contents.split("\n").filter(|line| line.len() > 0);

    // Setup the outcomes hashmap
    let possible_outcomes: HashMap<&str, usize> = HashMap::from([
        ("A X", 3), // Rock + Lose = Play Scissors = 3
        ("A Y", 1), // Rock + Draw = Play Rock = 1
        ("A Z", 2), // Rock + Win = Play Paper = 2
        ("B X", 1), // Paper + Lose = Play Rock = 1
        ("B Y", 2), // Paper + Draw = Play Paper = 2
        ("B Z", 3), // Paper + Win = Play Scissors = 3
        ("C X", 2), // Scissors + Lose = Play Paper = 2
        ("C Y", 3), // Scissors + Draw = Play Scissors = 3
        ("C Z", 1), // Scissors + Win = Play Rock = 1
    ]);

    // Get the score of each point
    let mut score: Vec<usize> = Vec::new();
    for round in lines {
        score.push(get_score(round, &possible_outcomes));
    }

    // Print out the sum of the scores
    println!("{}", score.iter().sum::<usize>());
}
