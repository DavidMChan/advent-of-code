// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    // Read the input
    let filename = env::args().nth(1).expect("No filename given");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // Parse the input to a set of beacons and a set of sensors
    let mut beacons = HashSet::new();
    let mut sensors = Vec::new();

    for line in contents.lines().filter(|line| line.len() > 0) {
        // Line contains:
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let mut parts = line.split("closest beacon is at x=");
        let sensor = parts.next().unwrap();
        let beacon = parts.next().unwrap();

        // Parse the beacon
        let mut beacon_parts = beacon.split(", y=");
        let bx = beacon_parts.next().unwrap().parse::<i32>().unwrap();
        let by = beacon_parts
            .next()
            .unwrap()
            .split(":")
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        beacons.insert((bx, by));

        // Parse the sensor
        let mut sensor_parts = sensor.split(", y=");
        let sx = sensor_parts
            .next()
            .unwrap()
            .split("x=")
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let sy = sensor_parts
            .next()
            .unwrap()
            .split(":")
            .nth(0)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // Get the distance (manhattan distance) from the sensor to the beacon
        let distance = (bx - sx).abs() + (by - sy).abs();

        // Add the sensor to the list of sensors
        sensors.push((sx, sy, distance));
    }

    let max_x = 4000000;
    let max_y = 4000000;

    // Precompute the in-range sensors for each row
    println!("Precomputing in-range sensors for each row...");
    let mut in_range_sensors = Vec::new();
    for row in 0..max_y {
        let mut sns = Vec::new();
        for (sx, sy, distance) in &sensors {
            // If we're within range of the sensor, this isn't possible
            if (sy - row).abs() <= *distance {
                sns.push((*sx, *sy, *distance));
            }
        }
        in_range_sensors.push(sns);
        if row % 100000 == 0 {
            println!(
                "Precomuting Row {} done ({:0.4}%)",
                row,
                row as f32 * 100.0 / max_y as f32
            );
        }
    }

    // Test for how many indices on the row fall within (distance) of a sensor
    let mut possible_beacon_locations = Vec::new();
    'major: for row in 0..max_y {
        let start = Instant::now();
        // Get the sensors in range of this row
        let in_range_sensors = &in_range_sensors[row as usize];

        // check each column, but skip
        let mut col = 0;
        'outer: while col < max_x {
            for (ssx, ssy, sdistance) in in_range_sensors {
                // If we're within range of the sensor, the beacon isn't here
                if (col - ssx).abs() + (row - ssy).abs() <= *sdistance {
                    // We can skip ahead to the next possible location
                    // (row - ssy).abs() distance is used vertically, so we can skip whatever is remaining
                    // horizontally
                    // println!(
                    //     "Tested {} {}, skipping to {} {}",
                    //     col,
                    //     row,
                    //     *ssx + *sdistance,
                    //     row
                    // );
                    col = *ssx + *sdistance - (row - ssy).abs() + 1;
                    continue 'outer;
                }
            }
            // If we get here, we're not within range of any sensors
            possible_beacon_locations.push((col, row));
            break 'major;
        }

        // 'outer: for col in 0..max_x {
        //     // Check if the cell is within the distance of any sensors
        //     for (ssx, ssy, sdistance) in in_range_sensors {
        //         // If we're within range of the sensor, this isn't possible
        //         if (col - ssx).abs() + (row - ssy).abs() <= *sdistance {
        //             continue 'outer;
        //         }
        //     }

        //     possible_beacon_locations.push((col, row));
        //     break 'major;
        // }
        // Print progress
        if row % 10000 == 0 {
            let elapsed = start.elapsed();
            println!(
                "Row {} done ({:0.4}%, ETA: {:?})",
                row,
                row as f32 * 100.0 / max_y as f32,
                Duration::from_secs_f32(elapsed.as_secs_f32() * (max_y - row) as f32)
            );
        }
    }

    // Print the grid of cells
    // for y in 0..21 {
    //     for x in 0..21 {
    //         if beacons.contains(&(x, y)) {
    //             print!("B");
    //             continue;
    //         }
    //         let mut printed = false;
    //         for (sx, sy, distance) in &sensors {
    //             if *sx == x && *sy == y {
    //                 print!("S");
    //                 printed = true;
    //                 break;
    //             }
    //             if (x - sx).abs() + (y - sy).abs() <= *distance {
    //                 print!("#");
    //                 printed = true;
    //                 break;
    //             }
    //         }
    //         if printed {
    //             continue;
    //         }
    //         print!(".");
    //     }
    //     println!();
    // }

    // Print the possible beacon locations
    for (x, y) in &possible_beacon_locations {
        println!(
            "Possible beacon location: {}, {}",
            x,
            y,
            // x * 4000000 + y
        );
    }
}
