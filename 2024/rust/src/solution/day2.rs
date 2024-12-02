use std::io::BufRead;

use super::Solution;

pub struct Day2Solution {}

impl Solution for Day2Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut count = 0;
        for line in input.lines() {
            let line = line?;
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            if is_safe(&nums) {
                count += 1
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut count = 0;
        for line in input.lines() {
            let line = line?;
            let mut nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();

            if is_safe(&nums) {
                count += 1
            } else {
                for i in 0..nums.len() {
                    let mut num_copy = nums.to_vec();
                    num_copy.remove(i);
                    if is_safe(&num_copy) {
                        count += 1;
                        break;
                    }
                }
            }
        }

        Ok(count.to_string())
    }
}

fn is_safe(nums: &Vec<i32>) -> bool {
    let mut safe_increasing = true;
    let mut safe_decreasing = true;

    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];

        if diff >= 0 || diff < -3 {
            safe_decreasing = false;
        }
        if diff <= 0 || diff > 3 {
            safe_increasing = false;
        }

        if !safe_increasing && !safe_decreasing {
            break;
        }
    }

    safe_increasing || safe_decreasing
}
