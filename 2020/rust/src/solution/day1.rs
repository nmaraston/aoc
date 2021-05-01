use std::collections::HashSet;
use std::io::BufRead;

use crate::solution::Solution;

pub struct Day1Solve {}

impl Solution for Day1Solve {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints: HashSet<i32> = read_ints(input)
            .into_iter()
            .collect();

        for i in &ints {
            let remainder = 2020 - i;
            if ints.contains(&remainder) {
                return Ok((i * remainder).to_string());
            }
        }

        Ok("No result found!".to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints = read_ints(input);
        for i in &ints {
            for j in &ints {
                for k in &ints {
                    if (i + j + k) == 2020 {
                        let prod = i * j * k;
                        return Ok(prod.to_string());
                    }

                }
            }
        }

        Ok("No result found!".to_string())
    }
}

fn read_ints(input: &mut dyn BufRead) -> Vec<i32> {
    input 
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut input = "1721\n979\n366\n299\n675\n1456".as_bytes();
        let under_test = Day1Solve {};
        let result = under_test.part_1(&mut input);

        assert_eq!(result.unwrap(), "514579");
    }

    #[test]
    fn test_part_2() {
        let mut input = "1721\n979\n366\n299\n675\n1456".as_bytes();
        let under_test = Day1Solve {};
        let result = under_test.part_1(&mut input);

        assert_eq!(result.unwrap(), "241861950");
    }
}

