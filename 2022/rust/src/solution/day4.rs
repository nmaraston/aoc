use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

use super::Solution;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn intersects(&self, other: &Range) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

pub struct Day4Solution {}

impl Solution for Day4Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let (range1, range2) = parse_line(&line.unwrap());
                (range1.contains(&range2) || range2.contains(&range1)) as u32
            })
            .sum();

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let (range1, range2) = parse_line(&line.unwrap());
                (range1.intersects(&range2)) as u32
            })
            .sum();

        Ok(result.to_string())
    }
}

fn parse_line(line: &String) -> (Range, Range) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    let caps = RE.captures(&line).unwrap();

    let range1 = Range {
        start: caps[1].parse::<u32>().unwrap(),
        end: caps[2].parse::<u32>().unwrap(),
    };
    let range2 = Range {
        start: caps[3].parse::<u32>().unwrap(),
        end: caps[4].parse::<u32>().unwrap(),
    };

    (range1, range2)
}
