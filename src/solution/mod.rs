pub trait Solution {
    fn part_1(&self, input_file_path: &str) -> std::io::Result<String>;
    fn part_2(&self, input_file_path: &str) -> std::io::Result<String>; 
}

mod day1;
mod day2;
mod day3;
mod day4;

pub fn get_solution(day: u32) -> Box<dyn Solution> {
    match day {
        1 => Box::new(day1::Day1Solve {}),
        2 => Box::new(day2::Day2Solve {}),
        3 => Box::new(day3::Day3Solve {}),
        4 => Box::new(day4::Day4Solve {}),
        _ => panic!("No solution implemented for given day {}", day),
    }
}

