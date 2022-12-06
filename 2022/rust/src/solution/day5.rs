use std::collections::HashMap;
use std::io::{BufRead, Lines};

use lazy_static::lazy_static;
use regex::Regex;

use super::Solution;

pub struct Day5Solution {}

impl Solution for Day5Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut lines = input.lines();
        let mut warehouse = Warehouse::parse(&mut lines)?;

        lines.next();

        for line in lines {
            let line = line?;
            let crate_move = CrateMove::parse(&line);
            warehouse.move_crates_pop(crate_move);
        }

        Ok(warehouse.top_crates())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut lines = input.lines();
        let mut warehouse = Warehouse::parse(&mut lines)?;

        lines.next();

        for line in lines {
            let line = line?;
            let crate_move = CrateMove::parse(&line);
            warehouse.move_crates_seq(crate_move);
        }

        Ok(warehouse.top_crates())
    }
}

struct CrateMove {
    num_crates: i32,
    from_stack: i32,
    to_stack: i32,
}

impl CrateMove {
    fn parse(line: &String) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let caps = RE.captures(&line).unwrap();

        Self {
            num_crates: caps[1].parse::<i32>().unwrap(),
            from_stack: caps[2].parse::<i32>().unwrap(),
            to_stack: caps[3].parse::<i32>().unwrap(),
        }
    }
}

struct Warehouse {
    stacks: HashMap<i32, Vec<char>>,
    max_stack_num: i32,
}

impl Warehouse {
    fn parse(lines: &mut Lines<&mut dyn BufRead>) -> std::io::Result<Self> {
        let mut stacks = HashMap::new();
        let mut max_stack_num = 0;

        'outer: while let Some(line) = lines.next() {
            let line = line?;
            for (i, c) in line.char_indices() {
                match c {
                    'A'..='Z' => {
                        let stack_num = ((i + 3) / 4) as i32;
                        stacks
                            .entry(stack_num)
                            .or_insert_with(|| Vec::new())
                            .push(c);
                        max_stack_num = max_stack_num.max(stack_num);
                    }
                    '1' => break 'outer,
                    _ => continue,
                }
            }
        }

        for stack_num in 1..=max_stack_num {
            stacks.get_mut(&stack_num).unwrap().reverse();
        }

        Ok(Self {
            stacks,
            max_stack_num,
        })
    }

    fn get_stack_mut(&mut self, stack_num: i32) -> &mut Vec<char> {
        self.stacks
            .get_mut(&stack_num)
            .expect("no stack found for stack number")
    }

    fn move_crates_pop(&mut self, crate_move: CrateMove) {
        for _ in 0..crate_move.num_crates {
            let cr8 = self
                .get_stack_mut(crate_move.from_stack)
                .pop()
                .expect("stack is unexpectedly empty");
            self.get_stack_mut(crate_move.to_stack).push(cr8);
        }
    }

    fn move_crates_seq(&mut self, crate_move: CrateMove) {
        let from_stack = self.get_stack_mut(crate_move.from_stack);
        let num_crates = crate_move.num_crates as usize;
        let drain = from_stack.drain((from_stack.len() - num_crates)..);
        let mut v = Vec::new();
        for c in drain {
            v.push(c);
        }
        self.get_stack_mut(crate_move.to_stack).extend(v);
    }

    fn top_crates(&self) -> String {
        let mut msg = String::new();
        for stack_num in 1..=self.max_stack_num {
            let c = self.stacks.get(&stack_num).unwrap().last().unwrap();
            msg.push(*c);
        }
        msg
    }
}
