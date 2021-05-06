use std::io::BufRead;
use regex::Regex;

use crate::solution::Solution;

pub struct Day2Solve {}

impl Solution for Day2Solve {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let count: u32 = input.lines()
            .filter(|line| {
                let line = line.as_ref().unwrap();
                let (policy, password) = parse_line(line);
                is_valid_attempt_1(&policy, &password)
            })
        .count() as u32;

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let count: u32 = input.lines()
            .filter(|line| {
                let line = line.as_ref().unwrap();
                let (policy, password) = parse_line(line);
                is_valid_attempt_2(&policy, &password)
            })
        .count() as u32;

        Ok(count.to_string())
    }
}

struct Policy {
    x: u32,
    y: u32,
    character: char,
}

fn parse_line(line: &String) -> (Policy, String) {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(line).unwrap();

    let x = caps.get(1).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap();
    let y = caps.get(2).map(|m| m.as_str().parse::<u32>().unwrap()).unwrap();
    let character = caps.get(3).map(|m| m.as_str().parse::<char>().unwrap()).unwrap();
    let password = caps.get(4).map(|m| m.as_str()).unwrap().to_owned();

    ( Policy {x, y, character}, password )
}

fn is_valid_attempt_1(policy: &Policy, password: &String) -> bool {
    let freq_count = password.chars()
        .filter(|c| *c == policy.character)
        .count() as u32;

    let min = policy.x;
    let max = policy.y;

    min <= freq_count && freq_count <= max
}

fn is_valid_attempt_2(policy: &Policy, password: &String) -> bool {
    // Safe because we assume the string only contains ASCII characters
    let bytes = password.as_bytes();

    let first = bytes[(policy.x as usize) - 1] as char;
    let second = bytes[(policy.y as usize) - 1] as char;

    ((first == policy.character) as u32) + ((second == policy.character) as u32) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let mut input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".as_bytes();
        let under_test = Day2Solve {};
        let result = under_test.part_1(&mut input);

        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn test_part_2() {
        let mut input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc".as_bytes();
        let under_test = Day2Solve {};
        let result = under_test.part_2(&mut input);

        assert_eq!(result.unwrap(), "1");
    }
}

