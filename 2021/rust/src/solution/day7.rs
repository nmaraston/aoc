use std::io::BufRead;

use super::Solution;

pub struct Day7Solution {}

impl Solution for Day7Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints = read_ints(input)?;
        let f = |pos: i16, int: i16| (pos - int).abs() as u32;
        Ok(find_position(ints, f).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let ints = read_ints(input)?;
        let f = |pos: i16, int: i16| {
            let n = (pos - int).abs() as u32;
            (n * (n + 1)) / 2
        };
        Ok(find_position(ints, f).to_string())
    }
}

fn find_position(ints: Vec<i16>, f: impl Fn(i16, i16) -> u32) -> u32 {
    let max_int = ints.iter().max().unwrap();

    let mut min_total = std::u32::MAX;
    for pos in 0..*max_int {
        let mut curr_total: u32 = 0;
        for int in &ints {
            curr_total += f(pos, *int)
        }

        min_total = std::cmp::min(min_total, curr_total);
    }

    min_total
}

fn read_ints(input: &mut dyn BufRead) -> std::io::Result<Vec<i16>> {
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    Ok(buf
        .split(",")
        .map(|s| s.trim_end().parse::<i16>().unwrap())
        .collect())
}
