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
    // let mut options = VecDeque::with_capacity(500);
    let mut options = Vec::new();
    options.push((
        26,
        HashSet::new(),
        0,
        *valve_map.get(&"AA".to_string()).unwrap(),
        *valve_map.get(&"AA".to_string()).unwrap(),
    )); // 30 minutes, no open valves, 0 pressure released, at location 0, elephant at location 0
    let mut explored = 0;

    // We should always be moving towards a valve or opening it
    while !options.is_empty() {
        let current_location = options.pop().unwrap();
        // Get all possible options
        // We can choose to move to a valve and open it for the remaining time
        let mut num_new_options = 0;
        if current_location.1.len() != valves.len() {
            // Either the elephant or the human (or both) will open a valve in the next step

            // Human opens valve
            for i in 0..valves.len() {
                // if the target flow rate is 0, then we don't need to bother going to this node
                if valves[i].1 == 0 {
                    continue;
                }
                // if we've already opened the valve, no need to open it again
                if current_location.1.contains(&i) {
                    continue;
                }
                // If it takes too long to move to this valve, then we can't go there
                if distances[current_location.3][i] > current_location.0 {
                    continue;
                }

                // Create new options for all possible elephants
                for j in 0..valves.len() {
                    // If the distance j is further than the distance to i, then we can't go there
                    // We actually have 1 extra minute to move to the valve, since we're not opening
                    // one...
                    if distances[current_location.4][j] - 1 > distances[current_location.3][i] {
                        continue;
                    }
                    // Add the option
                    let mut open_valves = current_location.1.clone();
                    open_valves.insert(i);
                    options.push((
                        current_location.0 - distances[current_location.3][i], // Time Remaining after moving to the valve and opening
                        open_valves,                                           // Open valves
                        current_location.2
                            + (valves[i].1
                                * ((current_location.0 as i32)
                                    - (distances[current_location.3][i] as i32))), // New pressure released at the end of 30 min
                        i, // New location
                        j, // New elephant location
                    ));
                    num_new_options += 1;
                }
            }

            // Elephant opens valve
            for j in 0..valves.len() {
                // if the target flow rate is 0, then we don't need to bother going to this node
                if valves[j].1 == 0 {
                    continue;
                }
                // if we've already opened the valve, no need to open it again
                if current_location.1.contains(&j) {
                    continue;
                }
                // If it takes too long to move to this valve, then we can't go there
                if distances[current_location.4][j] > current_location.0 {
                    continue;
                }
                // Create new options for all possible humans
                for i in 0..valves.len() {
                    // If the distance i is further than the distance to j, then we can't go there
                    // We actually have 1 extra minute to move to the valve, since we're not opening
                    // one...
                    if distances[current_location.3][i] - 1 > distances[current_location.4][j] {
                        continue;
                    }
                    // Add the option
                    let mut open_valves = current_location.1.clone();
                    open_valves.insert(j);
                    options.push((
                        current_location.0 - distances[current_location.4][j], // Time Remaining after moving to the valve and opening
                        open_valves,                                           // Open valves
                        current_location.2
                            + (valves[j].1
                                * ((current_location.0 as i32)
                                    - (distances[current_location.4][j] as i32))), // New pressure released at the end of 30 min
                        i, // New human location
                        j, // New location
                    ));
                    num_new_options += 1;
                }
            }

            // Both human and elephant open valve
            for i in 0..valves.len() {
                for j in 0..valves.len() {
                    // If they're not the same distance away, then we can't open both valves
                    if distances[current_location.3][i] != distances[current_location.4][j] {
                        continue;
                    }
                    // if they're the same valve, then we can't open it twice
                    if i == j {
                        continue;
                    }
                    // if the target flow rate is 0, then we don't need to bother going to this node
                    if valves[i].1 == 0 || valves[j].1 == 0 {
                        continue;
                    }
                    // if we've already opened the valve, no need to open it again
                    if current_location.1.contains(&i) || current_location.1.contains(&j) {
                        continue;
                    }
                    // If it takes too long to move to this valve, then we can't go there
                    if distances[current_location.3][i] > current_location.0 {
                        continue;
                    }
                    // Add the option
                    let mut open_valves = current_location.1.clone();
                    open_valves.insert(i);
                    open_valves.insert(j);
                    options.push((
                        current_location.0 - distances[current_location.3][i], // Time Remaining after moving to the valve and opening
                        open_valves,                                           // Open valves
                        current_location.2
                            + (valves[i].1
                                * ((current_location.0 as i32)
                                    - (distances[current_location.3][i] as i32))) // New pressure released at the end of 30 min
                            + (valves[j].1
                                * ((current_location.0 as i32)
                                    - (distances[current_location.3][i] as i32))), // New pressure released at the end of 30 min
                        i, // New human location
                        j, // New elephant location
                    ));
                    num_new_options += 1;
                }
            }
        }

        if num_new_options == 0 {
            // We have reached a terminal state
            terminated[current_location.3] =
                std::cmp::max(terminated[current_location.3], current_location.2);

            if terminated[current_location.3] == current_location.2 {
                terminated_solns[current_location.3] = current_location.clone();
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
    for i in 0..terminated.len() {
        max_terminated = std::cmp::max(max_terminated, terminated[i]);
    }
    println!(
        "Explored: {}, Max: {}, QL: {}",
        explored,
        max_terminated,
        options.len()
    );
    // Print the valves visited in the optimal solution
    for i in 0..terminated.len() {
        if terminated[i] == max_terminated {
            println!("Valves: {:?}", valves_visited[i]);
        }
    }

    // println!("Released: {}", pressure_released);
}
