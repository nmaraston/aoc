use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use super::Solution;

pub struct Day6Solution {}

impl Solution for Day6Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        Ok(solve(input, 4)?.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        Ok(solve(input, 14)?.to_string())
    }
}

fn solve(input: &mut dyn BufRead, window_size: usize) -> std::io::Result<usize> {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    let mut result = 0;
    let mut deque = VecDeque::new();
    for (i, c) in buffer.char_indices() {
        deque.push_back(c);

        let mut set: HashSet<char> = HashSet::with_capacity(window_size);
        set.extend(deque.iter());

        if set.len() == window_size {
            result = i + 1;
            break;
        } else if deque.len() < window_size {
            continue;
        } else {
            deque.pop_front();
        }
    }

    Ok(result)
}
