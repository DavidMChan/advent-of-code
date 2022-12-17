// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn shortest_path_distance(
    valves: &Vec<(String, i32, Vec<&str>)>,
    valve_map: &HashMap<String, usize>,
    start: usize,
    end: usize,
) -> usize {
    // Compute the shortest path between the starting valve and the end valve
    // We can only move to valves that are connected to the current valve

    // Create a queue
    let mut queue = Vec::new();
    queue.push((start, 0));

    while !queue.is_empty() {
        // Get the next item in the queue
        let (valve_index, distance) = queue.remove(0);

        // Check if we have reached the end
        if valve_index == end {
            return distance + 1;
        }

        // Add all the valves that are connected to this valve
        for valve in &valves[valve_index].2 {
            // Get the index of the valve
            let valve_index = valve_map.get(&valve.to_string()).unwrap();

            // Add the valve to the queue
            queue.push((*valve_index, distance + 1));
        }
    }

    // We have not found a path
    panic!("No path found");
}

fn main() {
    // Read the input
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Create a vector of valves, a map from valves to valve_vector_indices
    // Each valve has a branching factor of at most 5
    let mut valves: Vec<(String, i32, Vec<&str>)> = Vec::new();
    let mut valve_map = HashMap::new();

    for input in contents.lines().filter(|line| line.len() > 0) {
        // Input is of the form "Valve XX has flow rate=YY; tunnels lead to valves AA, BB, CC, etc"
        let mut parts = input.split("has flow rate=");
        let valve_name = parts.next().unwrap().split(" ").nth(1).unwrap().trim();
        let mut second_part = parts.next().unwrap().split(";");
        let valve_flow_rate = second_part.next().unwrap().parse::<i32>().unwrap();
        let valve_tunnels = second_part.next().unwrap();
        let mut valve_tunnels = valve_tunnels
            .split("valves")
            .nth(1)
            .unwrap_or(valve_tunnels.split("valve").nth(1).unwrap())
            .split(",")
            .collect::<Vec<&str>>();
        // Trim the valve_tunnels
        valve_tunnels = valve_tunnels
            .iter()
            .map(|valve| valve.trim())
            .collect::<Vec<&str>>();

        // Create the valve
        valves.push((valve_name.to_string(), valve_flow_rate, valve_tunnels));
        valve_map.insert(valve_name.to_string(), valves.len() - 1);
    }

    // Get the pairwise distances between all the valves
    let mut distances = Vec::new();
    for i in 0..valves.len() {
        let mut distances_row = vec![0; valves.len()];
        // Generate the shortest path matrix by traversal
        let mut queue = Vec::new();
        queue.push((i, 1));
        while !queue.is_empty() {
            // Get the next item in the queue
            let (valve_index, distance) = queue.remove(0);

            // Check if we have reached the end
            if distances_row[valve_index] == 0 || distances_row[valve_index] > distance {
                distances_row[valve_index] = distance;
            } else {
                continue;
            }

            // Add all the valves that are connected to this valve
            for valve in &valves[valve_index].2 {
                // Get the index of the valve
                let valve_index = valve_map.get(&valve.to_string()).unwrap();

                // Add the valve to the queue
                queue.push((*valve_index, distance + 1));
            }
        }
        distances.push(distances_row);
    }

    // Print out the distances
    for i in 0..valves.len() {
        for j in 0..valves.len() {
            print!("{:3} ", distances[i][j]);
        }
        println!();
    }

    // Determine the most pressure that can be released by wandering around the graph
    let mut terminated = vec![0; valves.len()];
    let mut terminated_solns = vec![None; valves.len()];
    let mut options = VecDeque::with_capacity(500);
    options.push_back((
        26,
        HashSet::new(),
        0,
        *valve_map.get(&"AA".to_string()).unwrap(),
    )); // 30 minutes, no open valves, 0 pressure released, at location 0
    let mut explored = 0;

    // NOTE: The best the human can do in 26 minutes is 1290, opening valves ["YR", "QW", "DZ", "JB", "OO", "BH", "DW", "CA"]
    let human_filtered_valves = vec!["YR", "QW", "DZ", "JB", "OO", "BH", "DW", "CA"];
    let human_filtered_valves = human_filtered_valves
        .iter()
        .map(|valve| *valve_map.get(&valve.to_string()).unwrap())
        .collect::<HashSet<usize>>();

    // We should always be moving towards a valve or opening it
    while !options.is_empty() {
        let current_location = options.pop_front().unwrap();
        // Get all possible options
        // We can choose to move to a valve and open it for the remaining time
        let mut num_new_options = 0;
        if current_location.1.len() != valves.len() {
            for i in 0..valves.len() {
                if i == current_location.3 {
                    // We are already at this valve
                    continue;
                }
                // If the valve is already open, then we can't open it again
                if current_location.1.contains(&i) {
                    continue;
                }
                // If it takes too long to move to this valve, then we can't go there
                if distances[current_location.3][i] > current_location.0 {
                    continue;
                }
                // If the target flow rate is 0, then we don't need to bother going to this node
                if valves[i].1 == 0 {
                    continue;
                }
                // If the human has already opened this valve, then we don't need to bother going to this node
                if human_filtered_valves.contains(&i) {
                    continue;
                }

                let mut open_valves = current_location.1.clone();
                open_valves.insert(i);
                options.push_back((
                    current_location.0 - distances[current_location.3][i], // Time Remaining after moving to the valve and opening
                    open_valves,                                           // Open valves
                    current_location.2
                        + (valves[i].1
                            * ((current_location.0 as i32)
                                - (distances[current_location.3][i] as i32))), // New pressure released at the end of 30 min
                    i, // New location
                ));
                num_new_options += 1;
            }
        }

        if num_new_options == 0 {
            // We have reached a terminal state
            terminated[current_location.3] =
                std::cmp::max(terminated[current_location.3], current_location.2);
            if terminated[current_location.3] == current_location.2 {
                terminated_solns[current_location.3] = Some(current_location.clone());
            }
        }

        // Print the maximum value of the terminated states

        if explored % 100000 == 0 {
            let mut max_terminated = 0;
            for i in 0..terminated.len() {
                max_terminated = std::cmp::max(max_terminated, terminated[i]);
            }
            println!(
                "Explored: {}, Max: {}, QL: {}",
                explored,
                max_terminated,
                options.len()
            );
        }
        explored += 1;
    }

    let mut max_terminated = 0;
    let mut max_index = 0;
    for i in 0..terminated.len() {
        max_terminated = std::cmp::max(max_terminated, terminated[i]);
        if max_terminated == terminated[i] {
            max_index = i;
        }
    }
    println!(
        "Explored: {}, Max: {}, QL: {}",
        explored,
        max_terminated,
        options.len()
    );

    // Print the valves in the best solution
    println!(
        "Best Solution: {:?}",
        terminated_solns[max_index]
            .as_ref()
            .unwrap()
            .1
            .iter()
            .map(|i| valves[*i].0.clone())
            .collect::<Vec<String>>()
    );

    // println!("Released: {}", pressure_released);
}
