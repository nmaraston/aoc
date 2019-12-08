use std::fs;

use crate::solution::Solution;

pub struct Day2Solve {}

impl Solution for Day2Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let mut ints = read_ints(input_file_path)?;

        ints[1] = 12;
        ints[2] = 2;

        let mut index: usize = 0;
        while ints[index] != 99 {
            let op1_register = ints[index + 1] as usize;
            let op2_register = ints[index + 2] as usize;
            let register = ints[index + 3] as usize;

            ints[register] = match ints[index] {
                1 => ints[op1_register] + ints[op2_register],
                2 => ints[op1_register] * ints[op2_register],
                _ => panic!("Unknown op code found.")
            };

            index += 4;
        }

        Ok(ints[0].to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
    }
}

fn read_ints(input_file_path: &str) -> std::io::Result<Vec<i32>> {
    let ints: Vec<i32> = fs::read_to_string(input_file_path)?
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    Ok(ints)
}

