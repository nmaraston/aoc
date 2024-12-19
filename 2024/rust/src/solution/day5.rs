use std::cmp::Ordering;
use std::io::BufRead;
use std::{collections::HashMap, collections::HashSet, num::ParseIntError};

use super::{AocSolutionError, Solution};

pub struct Day5Solution {}

impl Solution for Day5Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut sum = 0;
        let deps_map = read_deps_map(input)?;

        for line in input.lines() {
            let line = line?;
            let pages = parse_page_nums(line)?;

            if is_ordered(&pages, &deps_map) {
                sum += pages[pages.len() / 2];
            }
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut sum = 0;
        let deps_map = read_deps_map(input)?;

        for line in input.lines() {
            let line = line?;
            let mut pages = parse_page_nums(line)?;

            if !is_ordered(&pages, &deps_map) {
                pages.sort_by(|a, b| {
                    if deps_map.get(a).map_or(false, |s| s.contains(b)) {
                        Ordering::Greater
                    } else if deps_map.get(b).map_or(false, |s| s.contains(a)) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });

                sum += pages[pages.len() / 2];
            }
        }

        Ok(sum.to_string())
    }
}

fn read_deps_map(input: &mut dyn BufRead) -> Result<HashMap<u32, HashSet<u32>>, AocSolutionError> {
    let mut deps_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in input.lines() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let page_dep = line
            .split('|')
            .map(|n| n.parse::<u32>())
            .collect::<Result<Vec<u32>, ParseIntError>>()?;

        deps_map
            .entry(page_dep[1])
            .or_insert_with(|| HashSet::with_capacity(1))
            .insert(page_dep[0]);
    }

    Ok(deps_map)
}

fn parse_page_nums(line: String) -> Result<Vec<u32>, ParseIntError> {
    line.split(',')
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

fn is_ordered(nums: &[u32], deps_map: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..nums.len() - 1 {
        if let Some(deps) = deps_map.get(&nums[i]) {
            for n in nums.iter().skip(i + 1) {
                if deps.contains(n) {
                    return false;
                }
            }
        }
    }

    true
}
