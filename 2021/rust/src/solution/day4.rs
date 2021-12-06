use std::io::{BufRead, Lines};

use super::Solution;

#[derive(Debug)]
struct Board {
    cells: Vec<(u32, bool)>
}

impl Board {
    // Read 5 lines from the given line iterator to decode the board.
    fn from_read(lines: &mut Lines<&mut dyn BufRead>) -> std::io::Result<Board> {
        let mut cells = Vec::with_capacity(25);
        lines
            .take(5)
            .for_each(|line| {
                line.unwrap()
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .for_each(|i| cells.push((i, false)))
            });

        Ok(Board { cells })
    }

    // mark a cell
    fn mark(&mut self, number: u32) {
        for cell in &mut self.cells {
            if cell.0 == number {
                cell.1 = true;
            }
        }
    }

    // check if a cell is marked
    fn is_cell_marked(&self, row: usize, col: usize) -> bool {
        self.cells[row*5 + col].1
    }


    // check if board is in a winning state
    fn is_winner(&self) -> bool {
        for i in 0..5 {
            if self.is_row_winner(i) || self.is_col_winner(i) {
                return true
            }
        }
        false
    }

    // check if row number is in a winning state
    fn is_row_winner(&self, row: usize) -> bool {
        for col in 0..5 {
            if !self.is_cell_marked(row, col) {
                return false;
            }
        }
        true
    }

    // check if column number is in a winning state
    fn is_col_winner(&self, col: usize) -> bool {
        for row in 0..5 {
            if !self.is_cell_marked(row, col) {
                return false;
            }
        }
        true
    }

    // Sum umarked cell values
    fn score(&self) -> u32 {
        self.cells
            .iter()
            .map(|cell| if cell.1 { 0 } else { cell.0 })
            .sum()
    }
}

pub struct Day4Solution { }

impl Solution for Day4Solution {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let (numbers, mut boards) = load_numbers_and_boards(input)?;

        // Play bingo. Find first winning board
        for number in numbers {
            for board in &mut boards {
                board.mark(number);
                if board.is_winner() {
                    let score = board.score() * number;
                    return Ok(score.to_string())
                }

            }
        }

        // We should not get here
        Ok("No winning board found".to_owned())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let (numbers, boards) = load_numbers_and_boards(input)?;

        // Use tuples of (Board, bool) where the bool flags if the board is in a winning state
        let mut boards: Vec<(Board, bool)> = boards
            .into_iter()
            .map(|b| (b, false))
            .collect();

        // Play bingo. Find last winning board
        let mut last_winning_score = 0;
        for number in numbers {
            for board in &mut boards {
                if !board.1 {
                    board.0.mark(number);
                    if board.0.is_winner() {
                        board.1 = true;
                        last_winning_score = board.0.score() * number;
                    }
                }
            }
        }

        Ok(last_winning_score.to_string())
    }
}

fn load_numbers_and_boards(input: &mut dyn BufRead)  -> std::io::Result<(Vec<u32>, Vec<Board>)> {
    let mut lines = input.lines().into_iter();

    let numbers: Vec<u32> = lines.next()
        .unwrap()?
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    while let Some(line) = lines.next() {
        // If this line is empty, assume the next 5 lines are a board encoding.
        if line?.is_empty() {
            boards.push(Board::from_read(&mut lines)?)
        }
    }

    Ok((numbers, boards))
}
