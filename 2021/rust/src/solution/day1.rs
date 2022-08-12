use std::io::BufRead;

use super::Solution;

pub struct Day1Solution {}

impl Solution for Day1Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints = read_ints(input);

        let mut count = 0;

        // Assume len(ints) >= 2
        for i in 1..ints.len() {
            if ints.get(i) > ints.get(i - 1) {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints = read_ints(input);

        let mut prev_sum = std::i32::MAX;
        let mut count = 0;

        // Assume len(ints) >= 3
        for i in 2..ints.len() {
            let curr_sum = ints[i - 2] + ints[i - 1] + ints[i];
            if curr_sum > prev_sum {
                count += 1;
            }
            prev_sum = curr_sum;
        }

        Ok(count.to_string())
    }
}

fn read_ints(input: &mut dyn BufRead) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}
