use std::fs;

use crate::solution::Solution;
use super::intcode::Computer;

pub struct Day5Solve {}

impl Solution for Day5Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let program = fs::read_to_string(input_file_path)?
            .trim()
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let input_device = || 1; // Always input 1
        let mut outputs = Vec::new();
        let mut output_device = |val| outputs.push(val); // Store outputs in local Vec

        let mut intcode_computer = Computer::new(&input_device, &mut output_device, program);
        intcode_computer.run().unwrap();

        Ok(outputs.last().unwrap().to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
    }
}
