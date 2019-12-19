use crate::solution::Solution;
use super::intcode;

pub struct Day2Solve {}

impl Solution for Day2Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let mut intcode_computer = intcode::Computer::from_file(input_file_path)?;
        let result = intcode_computer.run(12, 2).unwrap();
        intcode_computer.reset();
        Ok(result.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let mut intcode_computer = intcode::Computer::from_file(input_file_path)?;
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
                let sub_res = intcode_computer.run(noun, verb);
                intcode_computer.reset();

                // If the result is an Err simply continue in the case where invalid input causes
                // a invalid IntCode program
                if let Ok(r) = sub_res {
                    if r == answer {
                        result = (100 * noun) + verb;
                        break;
                    } else if r == 0 || r > answer {
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

