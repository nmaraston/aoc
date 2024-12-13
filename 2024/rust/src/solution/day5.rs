use std::io::BufRead;
use std::{collections::HashMap, collections::HashSet, num::ParseIntError};

use super::{AocSolutionError, Solution};

pub struct Day5Solution {}

impl Solution for Day5Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut sum = 0;
        let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
        for line in input.lines() {
            let line = line?;

            if line.is_empty() {
                break;
            }

            let nums = line
                .split('|')
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()?;

            map.entry(nums[1])
                .or_insert_with(|| HashSet::with_capacity(1))
                .insert(nums[0]);
        }

        for line in input.lines() {
            let line = line?;

            let nums = line
                .split(',')
                .map(|n| n.parse::<u32>())
                .collect::<Result<Vec<u32>, ParseIntError>>()?;

            for i in 0..nums.len() {
                if i == nums.len() - 1 {
                    sum += nums[nums.len() / 2];
                } else if let Some(deps) = map.get(&nums[i]) {
                    let mut remaining_set: HashSet<u32> = HashSet::new();
                    remaining_set.extend(&nums[i..]);
                    if !deps.is_disjoint(&remaining_set) {
                        break;
                    }
                }
            }
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        unimplemented!()
    }
}
