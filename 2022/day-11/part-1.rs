// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;

struct Monkey {
    inspections: usize,
    items: Vec<usize>,
    operation: (char, Option<usize>, Option<usize>),
    test: usize,
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
                Some(op_arg_1 as usize)
            },
            if op_arg_2 == -1 {
                None
            } else {
                Some(op_arg_2 as usize)
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

    let rounds = 20;
    for _ in 0..rounds {
        // Simulate the monkeys

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
                println!(
                    "Monkey {} inspected item {}, worry grew to {}",
                    m, monkeys[m].items[idx], new_items[idx]
                )
            }
            // Decrease the value of each new item by 1/3
            for idx in 0..new_items.len() {
                println!(
                    "Monkey {} decreased worry of item {} to {}",
                    m,
                    new_items[idx],
                    new_items[idx] / 3
                );
                new_items[idx] = new_items[idx] / 3;
            }

            // Test each of the items, and move them to the appropriate target
            for idx in 0..new_items.len() {
                if new_items[idx] % monkeys[m].test == 0 {
                    let target = monkeys[m].true_target;
                    monkeys[target].items.push(new_items[idx]);
                    println!(
                        "Test True: Monkey {} moved item {} to monkey {}",
                        m, new_items[idx], target
                    )
                } else {
                    let target = monkeys[m].false_target;
                    monkeys[target].items.push(new_items[idx]);
                    println!(
                        "Test False: Monkey {} moved item {} to monkey {}",
                        m, new_items[idx], target
                    )
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
