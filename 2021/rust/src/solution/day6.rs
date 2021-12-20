use std::{io::BufRead, collections::HashMap};

use super::Solution;

pub struct Day6Solution { }

impl Solution for Day6Solution {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        Ok(simulate_lanternfish(input, 80)?.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        Ok(simulate_lanternfish(input, 256)?.to_string())
    }
}

fn simulate_lanternfish(input: &mut dyn BufRead, days: u16) -> std::io::Result<u64> {
    let ints = read_ints(input)?;

    let mut cache: HashMap<(u16, u16), u64> = HashMap::new();

    let mut total: u64 = 0;
    for age in ints {
        total += simulate_single_lanternfish(age, days, &mut cache);
    }

    Ok(total)
}

fn simulate_single_lanternfish(age: u16, day: u16, cache: &mut HashMap<(u16, u16), u64>) -> u64 {
    let key = (age, day);
    if let Some(res) = cache.get(&key) {
        *res
    } else {
        let res = if day == 0 || age >= day {
            1
        } else {
            simulate_single_lanternfish(6, day - age - 1, cache) + simulate_single_lanternfish(8, day - age - 1, cache)
        };
        cache.insert(key, res);
        res
    }
}

fn read_ints(input: &mut dyn BufRead) -> std::io::Result<Vec<u16>> {
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    Ok(
        buf
          .split(",")
          .map(|s| s.trim_end().parse::<u16>().unwrap())
          .collect())
}

