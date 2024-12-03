use regex::Regex;
use std::io::BufRead;
use std::ops::Range;

use super::{AocSolutionError, Solution};

pub struct Day3Solution {}

impl Solution for Day3Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;

        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0;
        for (_, [operand1, operand2]) in re.captures_iter(&input_buffer).map(|c| c.extract()) {
            sum += operand1.parse::<i32>()? * operand2.parse::<i32>()?;
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut input_buffer = String::new();
        input.read_to_string(&mut input_buffer)?;

        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let do_dont_re = Regex::new(r"(do\(\)|don't\(\))").unwrap();

        let mut enabled_start: i32 = 0;
        let mut enabled_ranges = Vec::new();
        for cap in do_dont_re.captures_iter(&input_buffer) {
            let m = cap.get(0).unwrap();
            if enabled_start >= 0 && m.as_str() == "don't()" {
                enabled_ranges.push((enabled_start as usize)..m.start());
                enabled_start = -1;
            } else if enabled_start == -1 && m.as_str() == "do()" {
                enabled_start = m.end() as i32;
            }
        }

        if enabled_start >= 0 {
            enabled_ranges.push((enabled_start as usize)..input_buffer.len());
        }

        let mut sum = 0;

        for cap in mul_re.captures_iter(&input_buffer) {
            let m = cap.get(0).unwrap();
            let operand1 = cap.get(1).unwrap().as_str();
            let operand2 = cap.get(2).unwrap().as_str();
            if is_in_enabled_range(&enabled_ranges, m.start()) {
                sum += operand1.parse::<i32>()? * operand2.parse::<i32>()?;
            }
        }

        Ok(sum.to_string())
    }
}

fn is_in_enabled_range(ranges: &Vec<Range<usize>>, index: usize) -> bool {
    for range in ranges {
        if range.contains(&index) {
            return true;
        }
    }
    false
}
