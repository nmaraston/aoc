use std::io::BufRead;

// TODO: consider returning closures as solutions rather then trying to
//  leverage polymorphism via a trait object.
pub trait Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String>;
    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String>; 
}

mod day1;

pub fn get_solution(day: u32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1Solve {}),
        _ => panic!("No solution implemented for given day {}", day),
    }
}

