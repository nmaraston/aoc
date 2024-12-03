use std::collections::HashMap;
use std::io::BufRead;

use super::{AocSolutionError, Solution};

pub struct Day1Solution {}

impl Solution for Day1Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut col1_nums = Vec::new();
        let mut col2_nums = Vec::new();

        for line in input.lines() {
            let line = line?;
            let mut line_split = line.split("   ");

            let n1 = line_split.next().unwrap().parse::<i32>()?;
            let n2 = line_split.next().unwrap().parse::<i32>()?;

            col1_nums.push(n1);
            col2_nums.push(n2);
        }

        col1_nums.sort();
        col2_nums.sort();

        let mut sum = 0;
        for i in 0..col1_nums.len() {
            sum += (col2_nums[i] - col1_nums[i]).abs()
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut col1_nums = Vec::new();
        let mut col2_freqs = HashMap::new();

        for line in input.lines() {
            let line = line?;
            let mut line_split = line.split("   ");

            let n1 = line_split.next().unwrap().parse::<i32>()?;
            let n2 = line_split.next().unwrap().parse::<i32>()?;

            col1_nums.push(n1);
            *col2_freqs.entry(n2).or_insert(0) += 1
        }

        let mut sum = 0;
        for n in col1_nums {
            sum += n * col2_freqs.get(&n).unwrap_or(&0)
        }

        Ok(sum.to_string())
    }
}
