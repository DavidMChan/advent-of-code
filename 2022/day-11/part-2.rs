// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

struct Monkey {
    inspections: u128,
    items: Vec<u128>,
    operation: (char, Option<u128>, Option<u128>),
    test: u128,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            inspections: 0,
            items: Vec::new(),
            operation: ('.', None, None),
            test: 0,
            true_target: 0,
            false_target: 0,
        }
    }
}

fn main() {
    // Read the input file from the first argument to a string
    let filename = env::args().nth(1).expect("No input file given");
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    // Parse the input into a vector of monkeys
    let mut monkeys: Vec<Monkey> = Vec::new();
    for lines in contents.split("\n\n") {
        // Create a new monkey
        monkeys.push(Monkey::new());
        let idx = monkeys.len() - 1;
        // First line is the monkey's name (We don't care)
        // Second line is the items
        for item in lines
            .split("\n")
            .nth(1)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
        {
            // Parse the item into a usize
            monkeys[idx].items.push(item.trim().parse().unwrap());
        }
        // Third line is the operation
        let op_str = lines
            .split("\n")
            .nth(2)
            .unwrap()
            .split("=")
            .nth(1)
            .unwrap()
            .trim();
        let ops = op_str.split(" ").clone().collect::<Vec<&str>>();
        let op_arg_1 = ops[0].trim().parse().unwrap_or(-1);
        let op_arg_2 = ops[2].trim().parse().unwrap_or(-1);
        monkeys[idx].operation = (
            ops[1].chars().nth(0).unwrap(),
            if op_arg_1 == -1 {
                None
            } else {
                Some(op_arg_1 as u128)
            },
            if op_arg_2 == -1 {
                None
            } else {
                Some(op_arg_2 as u128)
            },
        );
        // Fourth line is the test
        monkeys[idx].test = lines
            .split("\n")
            .nth(3)
            .unwrap()
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        // Fifth line is the true target
        monkeys[idx].true_target = lines
            .split("\n")
            .nth(4)
            .unwrap()
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        // Sixth line is the false target
        monkeys[idx].false_target = lines
            .split("\n")
            .nth(5)
            .unwrap()
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .parse()
            .unwrap();
    }

    let monkey_test_prod = monkeys.iter().fold(1, |acc, m| acc * m.test);

    let rounds = 10000;
    for r in 0..rounds {
        // Simulate the monkeys
        println!("Simulating monkeys... Round {}", r);
        // Inspect
        for m in 0..monkeys.len() {
            // Clone the vector of items to be inspected
            let mut new_items = Vec::new();

            // Apply the operation to each item value
            for idx in 0..monkeys[m].items.len() {
                match monkeys[m].operation.0 {
                    '+' => new_items.push(
                        monkeys[m].operation.1.unwrap_or(monkeys[m].items[idx])
                            + monkeys[m].operation.2.unwrap_or(monkeys[m].items[idx]),
                    ),
                    '*' => new_items.push(
                        monkeys[m].operation.1.unwrap_or(monkeys[m].items[idx])
                            * monkeys[m].operation.2.unwrap_or(monkeys[m].items[idx]),
                    ),
                    _ => panic!("Invalid operation"),
                }
                monkeys[m].inspections += 1;
            }

            // Divide the new values by the GCD of the monkey tests
            for idx in 0..new_items.len() {
                new_items[idx] = new_items[idx] % monkey_test_prod;
            }

            // Test each of the items, and move them to the appropriate target
            for idx in 0..new_items.len() {
                if new_items[idx] % monkeys[m].test == 0 {
                    let target = monkeys[m].true_target;
                    monkeys[target].items.push(new_items[idx]);
                } else {
                    let target = monkeys[m].false_target;
                    monkeys[target].items.push(new_items[idx]);
                }
            }

            // Clear the items
            monkeys[m].items.clear();
        }
    }

    // Print the monkey business (the number of inspections of the top two monkeys multiplied together)
    let mut monkey_inspections = Vec::new();
    for m in 0..monkeys.len() {
        monkey_inspections.push(monkeys[m].inspections);
    }
    monkey_inspections.sort();
    println!(
        "Monkey business: {}",
        monkey_inspections[monkey_inspections.len() - 1]
            * monkey_inspections[monkey_inspections.len() - 2]
    );
}
