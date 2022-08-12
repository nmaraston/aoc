use std::io::BufRead;

use super::Solution;

pub struct Day2Solution {}

impl Solution for Day2Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut horizontal = 0;
        let mut depth = 0;

        for line in input.lines() {
            let line = line?;
            let split: Vec<&str> = line.split_whitespace().collect();

            if split.len() != 2 {
                panic!("Expected line of form '(forward|up|down) [1-9]'");
            }

            let command = split[0];
            let arg = split[1].parse::<i32>().unwrap();

            match command {
                "forward" => horizontal += arg,
                "up" => depth -= arg,
                "down" => depth += arg,
                _ => panic!("Unknown command '{}'", command),
            }
        }

        Ok((horizontal * depth).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut aim = 0;
        let mut horizontal = 0;
        let mut depth = 0;

        for line in input.lines() {
            let line = line?;
            let split: Vec<&str> = line.split_whitespace().collect();

            if split.len() != 2 {
                panic!("Expected line of form '(forward|up|down) [1-9]'");
            }

            let command = split[0];
            let arg = split[1].parse::<i32>().unwrap();

            match command {
                "forward" => {
                    horizontal += arg;
                    depth += aim * arg;
                }
                "up" => aim -= arg,
                "down" => aim += arg,
                _ => panic!("Unknown command '{}'", command),
            }
        }

        Ok((horizontal * depth).to_string())
    }
}
