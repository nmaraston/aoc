use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::solution::Solution;

pub struct Day1Solve {}

impl Solution for Day1Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let sum:i32 = read_ints(input_file_path)?
            .iter()
            .map(|n| compute_fuel_1(*n))
            .sum();
        Ok(sum.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let sum:i32 = read_ints(input_file_path)?
            .iter()
            .map(|n| compute_fuel_2(*n))
            .sum();
        Ok(sum.to_string())
    }
}

fn compute_fuel_1(mass: i32) -> i32 {
   (mass / 3)  - 2
}

fn compute_fuel_2(mass: i32) -> i32 {
    if mass < 9 {
        0
    } else {
        let sub_fuel = compute_fuel_1(mass);
        sub_fuel + compute_fuel_2(sub_fuel)
    }
}

fn read_ints(input_file_path: &str) -> std::io::Result<Vec<i32>> {
    let file = File::open(input_file_path)?;
    let reader = BufReader::new(file);
    let ints: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    Ok(ints)
}

