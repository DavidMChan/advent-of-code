// Copyright (c) 2022 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::collections::HashSet;
use std::env;
use std::fs;

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

    let ROW = env::args()
        .nth(2)
        .expect("No row given")
        .parse::<i32>()
        .unwrap();
    let mut cells = HashSet::new();

    // Test for how many indices on the row fall within (distance) of a sensor
    let mut count = 0;
    for (sx, sy, distance) in sensors {
        if (sy - ROW).abs() <= distance {
            // It's possible that this sensor is within the distance of this row
            // Get how many cells on the row are within the distance of the sensor
            for x in (sx - distance)..=(sx + distance) {
                if (x - sx).abs() + (sy - ROW).abs() <= distance && !beacons.contains(&(x, ROW)) {
                    // This cell is within the distance of the sensor
                    cells.insert((x, ROW));
                }
            }
        }
    }

    // Print the number of cells
    println!("{}", cells.len());
    // Print the cells sorted by x then y
    // let mut cs: Vec<(i32, i32)> = cells.into_iter().collect();
    // cs.sort_by(|a, b| {
    //     if a.0 == b.0 {
    //         a.1.cmp(&b.1)
    //     } else {
    //         a.0.cmp(&b.0)
    //     }
    // });
    // for (x, y) in cs {
    //     println!("{} {}", x, y);
    // }
}
