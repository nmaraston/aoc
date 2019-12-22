use std::fs;

use crate::solution::Solution;
use super::intcode::Computer;

pub struct Day2Solve {}

impl Solution for Day2Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let program = load_program(input_file_path)?; 

        // Dummy input/output device handlers to satisfy generalized Intcode spec in day 5
        let input_device = || 0;
        let mut output_device = |val| println!("{}", val);

        let mut computer = Computer::new(&input_device, &mut output_device, program);
        computer.set_noun(12);
        computer.set_verb(2);
        computer.run().unwrap();
        let result = computer.mem_read(0).unwrap();
        Ok(result.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let program = load_program(input_file_path)?; 

        // Dummy input/output device handlers to satisfy generalized Intcode spec in day 5
        let input_device = || 0;
        let mut output_device = |val| println!("{}", val);

        let mut computer = Computer::new(&input_device, &mut output_device, program);
        let answer = 19690720;

        /*
         * Search through (noun, verb) pairs to find the answer.
         * The larger the sum of noun + verb the larger the result will be (this is clear if you
         * examine the first two op instructions in the input file). Therefore, we can regard the
         * following two statements as true and leverage them to prune some pairs:
         * 
         *   1. Since the answer computed in part 1 is less then the answer in part 2 we can skip
         *      all pairs where noun < 12 and verb <= 2.
         *   2. If v1 <= v2 and compute(noun, v1) > 19690720 then compute(noun, v2) >=
         *      compute(noun, v1) > 19690720 thus we can prune pairs for (noun, v) where v > v1.
         */
        let mut result = 0;
        for noun in 12..100 {
            for verb in 3..100 {

                computer.set_noun(noun);
                computer.set_verb(verb);
                let run_result = computer.run();
                let sub_res = computer.mem_read(0).unwrap();
                computer.reset();

                // If the result is an Err simply continue in the case where some input causes a
                // invalid IntCode program
                if let Ok(_) = run_result {
                    if sub_res == answer {
                        result = (100 * noun) + verb;
                        break;
                    } else if sub_res == 0 || sub_res > answer {
                        break;
                    }
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

fn load_program(file_path: &str) -> std::io::Result<Vec<i32>> {
    let program = fs::read_to_string(file_path)?
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    Ok(program)
}

