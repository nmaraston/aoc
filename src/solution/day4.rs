use std::fs;

use crate::solution::Solution;

pub struct Day4Solve {}

impl Solution for Day4Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let range = read_range(input_file_path)?;

        let mut valid_pass_count = 0;
        for num in range.0 .. range.1 {

            let mut has_double = false;
            let mut mono_increasing = true;

            let digits = digits(num);

            for i in 1 .. digits.len() {
                has_double = has_double || (digits[i] == digits[i - 1]);
                mono_increasing = mono_increasing && (digits[i] <= digits[i - 1]);
                if !mono_increasing {
                    break;
                }
            }

            if has_double && mono_increasing {
                valid_pass_count += 1;
            }
        }

        Ok(valid_pass_count.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
    }
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


fn read_range(input_file_path: &str) -> std::io::Result<(u32, u32)> {
    let contents: Vec<u32>  = fs::read_to_string(input_file_path)?
        .trim()
        .split("-")
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    Ok((contents[0], contents[1]))
}

