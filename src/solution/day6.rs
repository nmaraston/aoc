use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::solution::Solution;

pub struct Day6Solve {}

impl Solution for Day6Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let orbit_pairs = read_orbit_pairs(input_file_path)?;

        let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
        for orbit in orbit_pairs {
            match orbits.get_mut(&orbit.0) {
                Some(children) => children.push(orbit.1),
                None => {
                    let mut children: Vec<String> = Vec::new();
                    children.push(orbit.1);
                    orbits.insert(orbit.0, children);
                },
            }
        }

        let count = count_orbits(&orbits, &"COM", 0);
        Ok(count.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
    }
}

fn count_orbits(orbits: &HashMap<String, Vec<String>>, curr: &str, depth: u32) -> u32 {
    match orbits.get(curr) {
        Some(children) => {
            let mut count:u32 = depth;
            for obj in children {
                count += count_orbits(orbits, obj, depth + 1);
            }
            count
        },
        None => depth
    }
}

fn read_orbit_pairs(input_file_path: &str) -> std::io::Result<Vec<(String, String)>> {
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);
    let orbit_pairs: Vec<(String, String)> = reader.lines()
        .map(|l| {
            let line = l.unwrap();
            let split: Vec<&str> = line.split(")").collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();

    Ok(orbit_pairs)
}
