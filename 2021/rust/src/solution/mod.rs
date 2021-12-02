mod day1;
mod day2;

use std::io::BufRead;

pub trait Solution {
    fn part_1(&self, input: &mut dyn BufRead) ->std::io::Result<String>;
    fn part_2(&self, input: &mut dyn BufRead) ->std::io::Result<String>;
}

pub fn get_solution(day: u32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1Solution { }),
        2 => Box::new(day2::Day2Solution { }),
        _ => panic!("No solution implemented for given day {}", day),
    }
}

