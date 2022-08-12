mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::io::BufRead;

pub trait Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String>;
    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String>;
}

pub fn get_solution(day: u32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1Solution {}),
        2 => Box::new(day2::Day2Solution {}),
        3 => Box::new(day3::Day3Solution {}),
        4 => Box::new(day4::Day4Solution {}),
        5 => Box::new(day5::Day5Solution {}),
        6 => Box::new(day6::Day6Solution {}),
        7 => Box::new(day7::Day7Solution {}),
        8 => Box::new(day8::Day8Solution {}),
        9 => Box::new(day9::Day9Solution {}),
        _ => panic!("No solution implemented for given day {}", day),
    }
}
