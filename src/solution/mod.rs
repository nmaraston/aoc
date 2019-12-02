pub trait Solution {
    fn part_1(&self) -> String;
    fn part_2(&self) -> String;
}

pub mod day1;

pub const DAY1: day1::Day1 = day1::Day1 {};
