pub trait Solution {
    fn part_1(&self, input_file_path: &str) -> std::io::Result<String>;
    fn part_2(&self, input_file_path: &str) -> std::io::Result<String>; 
}

pub mod day1;

pub const DAY1: day1::Day1Solve = day1::Day1Solve {};
