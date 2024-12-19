use std::io::BufRead;

use super::{AocSolutionError, Solution};

pub struct Day19Solution {}

impl Solution for Day19Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut line_iter = input.lines();
        let line = line_iter.next().unwrap()?;
        let patterns: Vec<&str> = line.split(", ").collect();

        // Exhaust empty line
        line_iter.next();

        let mut count = 0;
        while let Some(line) = line_iter.next() {
            let line = line?;
            if search(&line, "", &patterns) {
                count += 1;
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        // TODO: This does not work! Brute force is far to slow!
        let mut line_iter = input.lines();
        let line = line_iter.next().unwrap()?;
        let patterns: Vec<&str> = line.split(", ").collect();

        // Exhaust empty line
        line_iter.next();

        let mut sum = 0;
        while let Some(line) = line_iter.next() {
            println!("{:?}", sum);
            let line = line?;
            sum += count(&line, "", &patterns);
        }

        Ok(sum.to_string())
    }
}

fn search(target: &str, curr: &str, patterns: &Vec<&str>) -> bool {
    if target == curr {
        true
    } else {
        let mut res = false;
        for pattern in patterns {
            let mut next = curr.to_owned();
            next.push_str(pattern);

            if target.starts_with(&next) && search(target, &next, patterns) {
                res = true;
                break;
            }
        }
        res
    }
}

fn count(target: &str, curr: &str, patterns: &Vec<&str>) -> u32 {
    if target == curr {
        1
    } else {
        let mut res = 0;
        for pattern in patterns {
            let mut next = curr.to_owned();
            next.push_str(pattern);

            if target.starts_with(&next) {
                res += count(target, &next, patterns);
            }
        }
        res
    }
}

/*
fn count2(target: &str, patterns: &Vec<&str>) -> u32 {
    if patterns.contains(&target) {
        1
    } else if target.len() == 1 {
        0
    } else {
        let mut res = 0;
        for i in 1..target.len() {
            let left = &target[..i];
            let right = &target[i..];

            res += count2(left, patterns) * count2(right, patterns);
        }

        res / ((target.len() as u32) - 1)
    }
}
*/
