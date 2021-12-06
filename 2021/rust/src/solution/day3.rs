use std::io::BufRead;

use super::Solution;

pub struct Day3Solution { }

impl Solution for Day3Solution {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        // Assume each line contains a string of 12 characters where each
        // character is a '0' or 1'.
        let mut counts = [0; 12];

        for line in input.lines() {
            for (i, c) in line?.chars().enumerate() {
                let inc = if c == '0' { -1 } else { 1 };
                counts[i] += inc;
            }
        }

        let mut gamma: u32 = 0;

        for n in &counts {
            gamma = gamma << 1;
            if *n >= 0 {
                gamma = gamma | 1;
            }
        }

        let epsilon =  gamma ^ 0x00000FFF;

        Ok((gamma * epsilon).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let report: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect();

        let oxy = part_2_search(&report, |count| {
            if count >= 0 { '1' } else { '0' }
        });
        let c02 = part_2_search(&report, |count| {
            if count >= 0 { '0' } else { '1' }
        });

        Ok((oxy * c02).to_string())
    }
}

fn part_2_search(
    report: &Vec<Vec<char>>,
    keep: impl Fn(i32) -> char,
) -> u32
{
    let mut candidates: Vec<usize> = (0..report.len()).collect();
    let mut i = 0;

    while candidates.len() != 1 {
        let mut count = 0;
        for cand in &candidates {
            let inc = if report[*cand][i] == '0' { -1 } else { 1 };
            count += inc;
        }

        let keep_char = keep(count);
        candidates.retain(|cand| {
            report[*cand][i] == keep_char
        });

        i += 1;
    }

    // Convert Vec of '1's and '0's to unsigned int
    let mut result: u32 = 0;
    for c in &report[candidates[0]] {
        result = result << 1;
        if *c == '1' {
            result = result | 1;
        }
    }

    result
}
