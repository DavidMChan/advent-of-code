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

    // You get 1 point for playing rock, 2 points for playing paper, and 3 points for playing scissors
    let choice_score = if round.ends_with("X") {
        1
    } else {
        if round.ends_with("Y") {
            2
        } else {
            3
        }
    };

    return choice_score
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
    // You get 0 points for losing, 3 points for draw, and 6 points for winning
    let possible_outcomes: HashMap<&str, usize> = HashMap::from([
        ("A X", 3), // Rock ties with Rock
        ("A Y", 6), // Rock loses to Paper
        ("A Z", 0), // Rock beats Scissors
        ("B X", 0), // Paper beats Rock
        ("B Y", 3), // Paper ties with Paper
        ("B Z", 6), // Paper loses to Scissors
        ("C X", 6), // Scissors loses to Rock
        ("C Y", 0), // Scissors beats Paper
        ("C Z", 3), // Scissors ties with Scissors
    ]);

    // Get the score of each point
    let mut score: Vec<usize> = Vec::new();
    for round in lines {
        score.push(get_score(round, &possible_outcomes));
    }

    // Print out the sum of the scores
    println!("{}", score.iter().sum::<usize>());
}
