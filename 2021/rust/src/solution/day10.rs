use std::collections::HashMap;
use std::io::BufRead;

use super::Solution;

pub struct Day10Solution {}

impl Solution for Day10Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let char_scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
        let mut result = 0;

        for line in input.lines() {
            let line = line?;

            if let ParseResult::UnexpectedChar(c) = parse_line(line) {
                result += char_scores[&c];
            }
        }

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let char_scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

        let mut scores = Vec::new();
        for line in input.lines() {
            let line = line?;

            if let ParseResult::Incomplete(stack) = parse_line(line) {
                let score = stack
                    .iter()
                    .rev()
                    .fold(0, |score: u64, c: &char| (5 * score) + char_scores[&c]);
                scores.push(score);
            }
        }

        // Take median score. Assume odd number of elements
        scores.sort();
        let result = scores[scores.len() / 2];

        Ok(result.to_string())
    }
}

enum ParseResult {
    Correct,
    UnexpectedChar(char),
    Incomplete(Vec<char>),
}

fn parse_line(line: String) -> ParseResult {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            ')' => {
                if stack.pop() != Some('(') {
                    return ParseResult::UnexpectedChar(')');
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return ParseResult::UnexpectedChar(']');
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return ParseResult::UnexpectedChar('}');
                }
            }
            '>' => {
                if stack.pop() != Some('<') {
                    return ParseResult::UnexpectedChar('>');
                }
            }
            _ => stack.push(c),
        }
    }

    if !stack.is_empty() {
        return ParseResult::Incomplete(stack);
    }

    return ParseResult::Correct;
}
