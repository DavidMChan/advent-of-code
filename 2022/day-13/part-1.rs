// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::env;
use std::fs;
use std::io;

struct Packet {
    values: Vec<Packet>,
    value: Option<usize>,
}

impl Packet {
    fn new(input: &str) -> Packet {
        // Check to see if the input is a list of values
        if input.starts_with('[') {
            // Remove the brackets, and parse the internal string
            let mut values: Vec<Packet> = Vec::new();

            // Split the string with nested brackets into chunks
            let mut chunks = Vec::new();
            let mut chunk = String::new();
            let mut depth = 0;
            for c in input[1..input.len() - 1].chars() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                }

                if depth == 0 && c == ',' {
                    chunks.push(chunk);
                    chunk = String::new();
                } else {
                    // Add the character to the chunk
                    chunk.push(c);
                }
            }
            if chunk.len() > 0 {
                chunks.push(chunk);
            }
            // Construct the packets from the chunks
            for chunk in chunks {
                values.push(Packet::new(&chunk));
            }

            return Packet {
                values,
                value: None,
            };
        }
        // Print
        Packet {
            values: Vec::new(),
            value: Some(input.trim().parse().unwrap()),
        }
    }

    fn new_from_value(value: usize) -> Packet {
        Packet {
            values: Vec::new(),
            value: Some(value),
        }
    }

    fn new_list_from_value(value: usize) -> Packet {
        Packet {
            values: vec![Packet::new_from_value(value)],
            value: None,
        }
    }

    // Print a packet
    fn print(&self) {
        if self.value.is_some() {
            print!("{}", self.value.unwrap());
        } else {
            print!("[");
            for (i, value) in self.values.iter().enumerate() {
                if i > 0 {
                    print!(",");
                }
                value.print();
            }
            print!("]");
        }
    }
}

fn in_order(packet1: &Packet, packet2: &Packet) -> i32 {
    // Print
    print!("Comparing ");
    packet1.print();
    print!(" => ");
    packet2.print();
    println!();

    // If both values are integers, lower should be first
    if packet1.value.is_some() && packet2.value.is_some() {
        println!(
            "Integer:: Comparing {} and {}",
            packet1.value.unwrap(),
            packet2.value.unwrap()
        );
        if (packet1.value.unwrap() < packet2.value.unwrap()) {
            println!("List in order!");
            return 1;
        } else if (packet1.value.unwrap() > packet2.value.unwrap()) {
            println!("List not in order!");
            return -1;
        }
    }

    // If both values are lists, then check to see if each element is in order
    if !packet1.value.is_some() && !packet2.value.is_some() {
        // If both values are lists, check if the first element is in order
        for (i, value1) in packet1.values.iter().enumerate() {
            if i >= packet2.values.len() {
                println!(
                    "List:: {} is longer than {}",
                    packet1.values.len(),
                    packet2.values.len()
                );
                return -1;
            }
            let order = in_order(value1, &packet2.values[i]);
            if order != 0 {
                return order;
            }
        }
        if packet1.values.len() < packet2.values.len() {
            println!(
                "List:: {} is shorter than {}",
                packet1.values.len(),
                packet2.values.len()
            );
            return 1;
        }
        // If the first list is shorter than the second, then the lists are in order
        return 0;
    }

    // If one value is a list and the other is an integer, then convert the integer to a list
    if !packet1.value.is_some() && packet2.value.is_some() {
        println!("1:: Converting integer to list: {}", packet2.value.unwrap());
        // Convert the integer to a list
        let p2 = Packet::new_list_from_value(packet2.value.unwrap());
        return in_order(packet1, &p2);
    } else if packet1.value.is_some() && !packet2.value.is_some() {
        println!("2:: Converting integer to list: {}", packet1.value.unwrap());
        // Convert the integer to a list
        let p1 = Packet::new_list_from_value(packet1.value.unwrap());
        return in_order(&p1, packet2);
    }

    return 0;
}

fn main() {
    // Read in the file contents from the first args
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Error reading file");

    // Parse the pairs of packets
    let mut packet_pairs = Vec::new();
    for pair in contents.split("\n\n") {
        let packets = pair.split("\n").collect::<Vec<&str>>();
        if packets.len() == 2 {
            let packet1 = Packet::new(packets[0]);
            let packet2 = Packet::new(packets[1]);
            packet_pairs.push((packet1, packet2));
        }
    }

    // Print the packets
    // for (packet1, packet2) in packet_pairs {
    //     packet1.print();
    //     print!(" => ");
    //     packet2.print();
    //     println!();
    // }

    // For every pair of chunks, check if it's in the right order
    let mut in_order_value = 0;
    for (i, pair) in packet_pairs.iter().enumerate() {
        let (packet1, packet2) = pair;
        if in_order(&packet1, &packet2) > 0 {
            println!("{} is in order", i + 1);
            in_order_value += (i + 1)
        } else {
            println!("{} is not in order", i + 1);
        }
        println!();

        // Wait for user input
        // let mut input = String::new();
        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Error reading input");
    }
    println!("In order value: {}", in_order_value);
}
