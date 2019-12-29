use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::solution::Solution;

pub struct Day6Solve {}

impl Solution for Day6Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let orbits = read_orbits(input_file_path)?;
        let orbit_children = orbit_children(&orbits);
        let count = count_orbits(&orbit_children, "COM", 0);
        Ok(count.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let orbits = read_orbits(input_file_path)?;
        let orbit_parents = orbit_parents(&orbits);
        let orbit_children = orbit_children(&orbits);
        let distance = min_distance(&orbit_parents, &orbit_children, "YOU", "SAN")
            .expect("No path exists between 'YOU' and 'SAN' for given input");
        let min_orbital_transfers = distance - 2;
        Ok(min_orbital_transfers.to_string())
    }
}

/**
 * Modeld as a graph, return the minimum distance between two planets.
 */
fn min_distance(orbit_parents: &HashMap<&str, &str>,
                orbit_children: &HashMap<&str, Vec<&str>>,
                start: &str,
                end: &str) -> Option<u32> {
    let mut queue: VecDeque<(&str, u32)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    queue.push_back((start, 0));
    while let Some(node) = queue.pop_front() {
        if node.0 == end {
            return Some(node.1);
        }
        if let Some(parent) = orbit_parents.get(node.0) {
            if !visited.contains(parent) {
                queue.push_back((parent, node.1 + 1));
            }
        }
        if let Some(children) = orbit_children.get(node.0) {
            for child in children {
                if !visited.contains(child) {
                    queue.push_back((child, node.1 + 1));
                }
            }
        }
        visited.insert(node.0);
    }

    return None;
}

fn count_orbits(orbit_children: &HashMap<&str, Vec<&str>>, curr: &str, depth: u32) -> u32 {
    match orbit_children.get(curr) {
        Some(children) => {
            let mut count:u32 = depth;
            for obj in children {
                count += count_orbits(orbit_children, obj, depth + 1);
            }
            count
        },
        None => depth
    }
}

/**
 * Given a collection of orbit tuples (parent to child relationship) return a HashMap of (c, p)
 * where c directly orbits p (child to parent index).
 */
fn orbit_parents<'a>(orbits: &'a Vec<(String, String)>) -> HashMap<&'a str, &'a str> {
    let mut orbit_parents: HashMap<&str, &str> = HashMap::new();
    for orbit in orbits {
        orbit_parents.insert(&orbit.1, &orbit.0);
    }
    orbit_parents
}

/**
 * Given a collection of orbit tuples (parent to child relationship) return a HashMap of (p, [c])
 * where p is a planet and [c] is a collection of planets that directly orbit k (children indexed
 * by parent).
 */
fn orbit_children<'a>(orbits: &'a Vec<(String, String)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut orbit_children: HashMap<&str, Vec<&str>> = HashMap::new();
    for orbit in orbits {
        match orbit_children.get_mut(orbit.0.as_str()) {
            Some(children) => children.push(&orbit.1),
            None => {
                let mut children: Vec<&str> = Vec::new();
                children.push(&orbit.1);
                orbit_children.insert(&orbit.0, children);
            },
        }
    }

    orbit_children
}

/**
 * Read orbit input data into tuples (A, B) where B orbits A.
 */
fn read_orbits(input_file_path: &str) -> std::io::Result<Vec<(String, String)>> {
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);
    let orbits: Vec<(String, String)> = reader.lines()
        .map(|l| {
            let line = l.unwrap();
            let split: Vec<&str> = line.split(")").collect();
            (split[0].to_owned(), split[1].to_owned())
        })
        .collect();

    Ok(orbits)
}
