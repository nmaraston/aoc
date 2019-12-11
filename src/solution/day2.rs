use std::fs;

use crate::solution::Solution;

pub struct Day2Solve {}

impl Solution for Day2Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let mut memory = read_ints(input_file_path)?;
        let result = compute(12, 2, &mut memory);
        Ok(result.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let mut memory = read_ints(input_file_path)?;

        /*
         * Search through (noun, verb) pairs to find the answer.
         * The larger the sum of noun + verb the larger the result will be (this is clear if you
         * examine the first two op instructions in the input file). Therefore, we can regard the
         * following two statements as true and leverage them to prune some pairs:
         * 
         *     1. Since the answer computed in part 1 is less then the answer in part 2 we can skip
         *        all pairs where noun < 12 and verb <= 2.
         *     2. If v1 <= v2 and compute(noun, v1) > 19690720 then compute(noun, v2) >=
         *        compute(noun, v1) > 19690720 thus we can prune pairs for (noun, v) where v > v1.
         */
        let mut result = 0;
        for noun in 12..100 {
            for verb in 3..100 {
                let mut mem_copy = memory.clone();
                let sub_res = compute(noun, verb, &mut mem_copy);
                if sub_res == 19690720 {
                    result = (100 * noun) + verb;
                } else if sub_res == 0 || sub_res > 19690720 {
                    break;
                }
            }

            // Exit if we found the answer
            if result != 0 {
                break;
            }
        }
        
        Ok(result.to_string())
    }
}

/**
 * Run the Intcode program as documented. Return 0 if a (noun, verb) input pair will cause a index
 * out of bounds error.
 */
fn compute(noun: u32, verb: u32, memory: &mut Vec<u32>) -> u32 {

    memory[1] = noun;
    memory[2] = verb;
    
    let mut instr_ptr: usize = 0;

    while memory[instr_ptr] != 99 {
        let op1_register = memory[instr_ptr + 1] as usize;
        let op2_register = memory[instr_ptr + 2] as usize;
        let register = memory[instr_ptr + 3] as usize;

        memory[register] = match memory[instr_ptr] {
            1 => memory[op1_register] + memory[op2_register],
            2 => memory[op1_register] * memory[op2_register],
            _ => panic!("Unknown op code found.")
        };

        instr_ptr += 4;

        if instr_ptr >= memory.len() {
            return 0;
        }
    }

    memory[0]
}


fn read_ints(input_file_path: &str) -> std::io::Result<Vec<u32>> {
    let ints: Vec<u32> = fs::read_to_string(input_file_path)?
        .trim()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    Ok(ints)
}

