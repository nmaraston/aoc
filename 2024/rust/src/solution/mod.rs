mod day1;
mod day10;
mod day11;
mod day12;
mod day16;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::io::BufRead;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocSolutionError {
    // Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    // Represents cases where parsing the input file failed.
    #[error("failed to parse input file: {message:?}")]
    ParseError { message: String },
}

impl From<ParseIntError> for AocSolutionError {
    fn from(err: ParseIntError) -> AocSolutionError {
        AocSolutionError::ParseError {
            message: format!("failed to parse int: {:?}", err),
        }
    }
}

pub trait Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError>;
    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError>;
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
        10 => Box::new(day10::Day10Solution {}),
        11 => Box::new(day11::Day11Solution {}),
        12 => Box::new(day12::Day12Solution {}),
        16 => Box::new(day16::Day16Solution {}),
        18 => Box::new(day18::Day18Solution {}),
        19 => Box::new(day19::Day19Solution {}),
        _ => panic!("No solution implemented for given day {}", day),
    }
}
