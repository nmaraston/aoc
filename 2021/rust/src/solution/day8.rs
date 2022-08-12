use std::collections::HashSet;
use std::convert::TryInto;
use std::io::BufRead;

use super::Solution;

#[derive(Debug)]
struct SignalPattern {
    signals: [HashSet<char>; 10],
    outputs: [HashSet<char>; 4],
}

impl SignalPattern {
    fn from(line: String) -> SignalPattern {
        let mut split = line.split(" | ");

        let signals: Vec<HashSet<char>> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        let outputs: Vec<HashSet<char>> = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();

        SignalPattern {
            // This will fail if signals.len() != 10 or outputs.len() != 4
            signals: signals.try_into().unwrap(),
            outputs: outputs.try_into().unwrap(),
        }
    }

    fn decode(&self) -> Vec<(&HashSet<char>, u32)> {
        let mut signals: Vec<&HashSet<char>> = self.signals.iter().collect();

        // One, four, seven and eight are given based on unique lenghts
        let one = remove_first(&mut signals, |s| s.len() == 2);
        let four = remove_first(&mut signals, |s| s.len() == 4);
        let seven = remove_first(&mut signals, |s| s.len() == 3);
        let eight = remove_first(&mut signals, |s| s.len() == 7);

        // Six is the only string to contain 6 characters and not contain one
        let six = remove_first(&mut signals, |s| s.len() == 6 && !one.is_subset(&s));

        // Five is the only remaining string to contain 5 characters and is a subset of six
        let five = remove_first(&mut signals, |s| s.len() == 5 && s.is_subset(&six));

        // Nine is the only remaining string to contain 6 characters with the cardinality of the
        // set difference between one being 1
        let nine = remove_first(&mut signals, |s| {
            s.len() == 6 && s.difference(&five).count() == 1
        });

        //  Zero is the only remaining string of 6 characters
        let zero = remove_first(&mut signals, |s| s.len() == 6);

        // Three is the only remaining string with the cardinality of the set difference between
        // one being 3
        let three = remove_first(&mut signals, |s| s.difference(&one).count() == 3);

        // Only two remains
        let two = signals.remove(0);

        vec![
            (zero, 0),
            (one, 1),
            (two, 2),
            (three, 3),
            (four, 4),
            (five, 5),
            (six, 6),
            (seven, 7),
            (eight, 8),
            (nine, 9),
        ]
    }
}
fn remove_first<T, F>(vec: &mut Vec<T>, predicate: F) -> T
where
    F: FnMut(&T) -> bool,
{
    // Unwrap here for convenience
    let index = vec.iter().position(predicate).unwrap();
    vec.remove(index)
}

pub struct Day8Solution {}

impl Solution for Day8Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let signal_patterns: Vec<SignalPattern> = input
            .lines()
            .map(|line| SignalPattern::from(line.unwrap()))
            .collect();

        let mut count = 0;
        for pattern in &signal_patterns {
            for output in &pattern.outputs {
                let len = output.len();
                if len == 2 || len == 3 || len == 4 || len == 7 {
                    count += 1;
                }
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let signal_patterns: Vec<SignalPattern> = input
            .lines()
            .map(|line| SignalPattern::from(line.unwrap()))
            .collect();

        let mut decode_sum = 0;
        for pattern in &signal_patterns {
            let signal_mapping = pattern.decode();
            let mut pow = 3;
            for output in &pattern.outputs {
                for mapping in &signal_mapping {
                    if mapping.0 == output {
                        decode_sum += mapping.1 * (10 as u32).pow(pow);
                        if pow != 0 {
                            pow -= 1;
                        }
                    }
                }
            }
        }

        Ok(decode_sum.to_string())
    }
}
