use std::collections::HashSet;
use std::{io::BufRead, num::ParseIntError};

use super::{AocSolutionError, Solution};

pub struct Day7Solution {}

impl Solution for Day7Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let eval = |o1: u64, o2: u64| {
            let mut res: HashSet<u64> = HashSet::new();
            res.insert(o1 * o2);
            res.insert(o1 + o2);
            res
        };

        solve(input, &eval)
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let eval = |o1: u64, o2: u64| {
            let mut res: HashSet<u64> = HashSet::new();
            res.insert(o1 * o2);
            res.insert(o1 + o2);
            res.insert((o1 * 10_u64.pow(o2.ilog10() + 1)) + o2);
            res
        };

        solve(input, &eval)
    }
}

fn solve(
    input: &mut dyn BufRead,
    eval: &impl Fn(u64, u64) -> HashSet<u64>,
) -> Result<String, AocSolutionError> {
    let mut sum = 0;
    for line in input.lines() {
        let line = line?;
        let split: Vec<&str> = line.split(": ").collect();

        let test_val = split[0].parse::<u64>()?;
        let nums = split[1]
            .split_whitespace()
            .map(|n| n.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;

        if possible_evals(&nums, &eval).contains(&test_val) {
            sum += test_val;
        }
    }

    Ok(sum.to_string())
}

fn possible_evals(nums: &[u64], eval: &impl Fn(u64, u64) -> HashSet<u64>) -> HashSet<u64> {
    let mut result = HashSet::new();

    if nums.len() == 1 {
        result.insert(nums[0]);
    } else {
        let phead = possible_evals(&nums[0..nums.len() - 1], eval);
        let ptail = possible_evals(&nums[nums.len() - 1..], eval);

        for o1 in &phead {
            for o2 in &ptail {
                result.extend(eval(*o1, *o2));
            }
        }
    }

    result
}
