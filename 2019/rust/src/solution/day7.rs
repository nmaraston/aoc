use std::cmp::max;
use std::fs;

use crate::solution::Solution;
use super::intcode::Computer;

pub struct Day7Solve {}

impl Solution for Day7Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let program = load_program(input_file_path)?;
        let mut max_final_output = 0;
        let mut output = 0;
        for permutation in permutations(&vec![0, 1, 2, 3, 4]) {
            for input in permutation {
                output = run_program(program.clone(), (input as i32, output));
            }
            max_final_output = max(max_final_output, output);
            output = 0;
        }

        Ok(max_final_output.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
    }
}

fn load_program(file_path: &str) -> std::io::Result<Vec<i32>> {
    let program = fs::read_to_string(file_path)?
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    Ok(program)
}

fn permutations(values: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms = Vec::new();
    if values.len() <= 1 {
        perms.push(values.clone());
    } else {
        for i in 0..values.len() {
            let sub_values = [&values[..i], &values[i+1..]].concat();
            for sub_perm in permutations(&sub_values) {
                let mut perm: Vec<i32> = Vec::new();
                perm.push(values[i]);
                perm.extend(sub_perm);
                perms.push(perm)
            }
        }
    }
    perms 
}

fn run_program(program: Vec<i32>, inputs: (i32, i32)) -> i32 {
    let mut input_counter = 0;
    let mut input_device = || {
        input_counter += 1;
        match input_counter {
            1 => inputs.0,
            2 => inputs.1,
            _ => panic!("Only two input prompts expected"),
        }
    };

    let mut outputs = Vec::new();
    let mut output_device = |val| outputs.push(val); // Store outputs in local Vec

    let mut intcode_computer = Computer::new(&mut input_device, &mut output_device, program);
    intcode_computer.run().unwrap();

    *outputs.last().unwrap()
}

