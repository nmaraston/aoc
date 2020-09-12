use std::fs;

use crate::solution::Solution;

pub struct Day4Solve {}

impl Solution for Day4Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let range = read_range(input_file_path)?;

        let count = count_valid_passwords(range.0, range.1, |val| {
            let digits = digits(val);
            mono_increasing(&digits) && has_double(&digits)
        });

        Ok(count.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let range = read_range(input_file_path)?;

        let count = count_valid_passwords(range.0, range.1, |val| {
            let digits = digits(val);
            mono_increasing(&digits) && has_strict_double(&digits)
        });

        Ok(count.to_string())
    }
}

fn count_valid_passwords(lower: u32, upper: u32, valid: fn(u32) -> bool) -> u32 {
    (lower .. upper)
        .fold(0, |sum, val| {
            match valid(val) {
                true => sum + 1,
                false => sum
            }})
}

fn digits(num: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut n = num;
    while  n != 0 {
        let sub_n = n / 10;
        result.push(n - (sub_n * 10));
        n = sub_n;
    }
    result
}

fn has_double(digits: &Vec<u32>) -> bool {
    let mut result = false;
    for i in 1 .. digits.len() {
        result = result || (digits[i] == digits[i - 1]);
    }
    result
}

fn has_strict_double(digits: &Vec<u32>) -> bool {
    let mut cand = digits[0];
    let mut adj_count = 1;
    for i in 1 .. digits.len() {
        if digits[i] == cand {
            adj_count += 1;
        } else if adj_count == 2 {
            return true;
        } else {
            cand = digits[i];
            adj_count = 1;
        }
    }

    adj_count == 2
}

fn mono_increasing(digits: &Vec<u32>) -> bool {
    for i in 1 .. digits.len() {
        if !(digits[i - 1] >= digits[i]) {
            return false;
        }
    }
    true
}

fn read_range(input_file_path: &str) -> std::io::Result<(u32, u32)> {
    let contents: Vec<u32> = fs::read_to_string(input_file_path)?
        .trim()
        .split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    Ok((contents[0], contents[1]))
}

