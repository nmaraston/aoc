use std::collections::HashSet;
use std::io::BufRead;

use phf::phf_map;

use super::Solution;

static PRIORITIES: phf::Map<char, u32> = phf_map! {
  'a' => 1,
  'b' => 2,
  'c' => 3,
  'd' => 4,
  'e' => 5,
  'f' => 6,
  'g' => 7,
  'h' => 8,
  'i' => 9,
  'j' => 10,
  'k' => 11,
  'l' => 12,
  'm' => 13,
  'n' => 14,
  'o' => 15,
  'p' => 16,
  'q' => 17,
  'r' => 18,
  's' => 19,
  't' => 20,
  'u' => 21,
  'v' => 22,
  'w' => 23,
  'x' => 24,
  'y' => 25,
  'z' => 26,
};

pub struct Day3Solution {}

impl Solution for Day3Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut result = 0;
        for line in input.lines() {
            let line = line?;
            let compartment2 = to_hash_set(&line[line.len() / 2..]);

            let common_item_type = &line[..line.len() / 2]
                .chars()
                .find(|c| compartment2.contains(c))
                .expect("no common item type found between rucksack compartments");

            result += priority(*common_item_type);
        }

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut result = 0;
        let mut line_iter = input.lines();

        while let Some(line) = line_iter.next() {
            let rucksack2 = to_hash_set(&line_iter.next().unwrap()?);
            let rucksack3 = to_hash_set(&line_iter.next().unwrap()?);

            let common_item = line?
                .chars()
                .find(|c| rucksack2.contains(c) && rucksack3.contains(c))
                .expect("no common item type found between rucksack triplet");

            result += priority(common_item);
        }

        Ok(result.to_string())
    }
}

fn to_hash_set(part: &str) -> HashSet<char> {
    let mut set = HashSet::with_capacity(part.len());
    for c in part.chars() {
        set.insert(c);
    }
    set
}

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        PRIORITIES.get(&c.to_lowercase().next().unwrap()).unwrap() + 26
    } else {
        *PRIORITIES.get(&c).unwrap()
    }
}
