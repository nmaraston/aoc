mod day1;
mod day2;
mod day3;

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
        _ => panic!("No solution implemented for given day {}", day),
    }
}
