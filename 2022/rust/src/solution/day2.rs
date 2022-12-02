use std::io::BufRead;

use super::Solution;

pub struct Day2Solution {}

impl Solution for Day2Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let moves = parse_line(&line.unwrap());
                compute_score(moves.0, moves.1)
            })
            .sum();

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let result: u32 = input
            .lines()
            .map(|line| {
                let moves = parse_line(&line.unwrap());
                compute_score(moves.0, needed_move(moves.0, moves.1))
            })
            .sum();

        Ok(result.to_string())
    }
}

fn parse_line(line: &String) -> (char, char) {
    let mut chars = line.chars();
    let p1move = chars.next().unwrap();
    chars.next();
    let p2move = chars.next().unwrap();

    (p1move, p2move)
}

fn needed_move(player1_move: char, outcome: char) -> char {
    match (player1_move, outcome) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("Unrecognized character in file"),
    }
}

fn compute_score(player_move1: char, player_move2: char) -> u32 {
    match (player_move1, player_move2) {
        ('A', 'X') => 1 + 3,
        ('A', 'Y') => 2 + 6,
        ('A', 'Z') => 3 + 0,
        ('B', 'X') => 1 + 0,
        ('B', 'Y') => 2 + 3,
        ('B', 'Z') => 3 + 6,
        ('C', 'X') => 1 + 6,
        ('C', 'Y') => 2 + 0,
        ('C', 'Z') => 3 + 3,
        _ => panic!("Unrecognized character in file"),
    }
}
