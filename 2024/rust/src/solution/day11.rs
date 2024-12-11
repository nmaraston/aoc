use std::collections::HashMap;
use std::io::BufRead;
use std::num::ParseIntError;

use super::{AocSolutionError, Solution};

pub struct Day11Solution {}

impl Solution for Day11Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        solve(input, 25)
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        solve(input, 75)
    }
}

fn solve(input: &mut dyn BufRead, blink_count: u8) -> Result<String, AocSolutionError> {
    let mut line = String::new();
    let _ = input.read_line(&mut line);

    // pop trailing newline character
    line.pop();

    let numbers: Vec<u64> = line
        .split(' ')
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<u64>, ParseIntError>>()?;

    let mut table: HashMap<(u8, u64), u64> = HashMap::new();

    let count: u64 = numbers
        .iter()
        .map(|n| blink(*n, blink_count, &mut table))
        .sum();

    Ok(count.to_string())
}

fn blink(stone: u64, blink_num: u8, table: &mut HashMap<(u8, u64), u64>) -> u64 {
    if blink_num == 0 {
        1
    } else if table.contains_key(&(blink_num, stone)) {
        *table.get(&(blink_num, stone)).unwrap()
    } else {
        let num_digits = count_digits(stone);
        let b = if stone == 0 {
            blink(1, blink_num - 1, table)
        } else if num_digits % 2 == 0 {
            let n1 = stone % (10_u64.pow(num_digits / 2));
            let n2 = stone / (10_u64.pow(num_digits / 2));
            blink(n1, blink_num - 1, table) + blink(n2, blink_num - 1, table)
        } else {
            blink(stone * 2024, blink_num - 1, table)
        };
        table.insert((blink_num, stone), b);
        b
    }
}

fn count_digits(mut n: u64) -> u32 {
    let mut count = 0;
    loop {
        count += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    count
}
