use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};

pub struct Day4Solution {}

impl Solution for Day4Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let matrix = Matrix::new(input)?;

        let mut count = 0;
        let word = ['X', 'M', 'A', 'S'];
        for row in 0..matrix.num_rows {
            for col in 0..matrix.num_cols {
                count += word_search(&matrix, row, col, &word);
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let matrix = Matrix::new(input)?;

        let m_s_set = HashSet::from(['M', 'S']);
        let mut count = 0;
        for row in 1..(matrix.num_rows - 1) {
            for col in 1..(matrix.num_cols - 1) {
                if matrix.get(row, col) == 'A' {
                    let diag1 =
                        HashSet::from([matrix.get(row - 1, col - 1), matrix.get(row + 1, col + 1)]);
                    let diag2 =
                        HashSet::from([matrix.get(row - 1, col + 1), matrix.get(row + 1, col - 1)]);

                    if diag1 == m_s_set && diag2 == m_s_set {
                        count += 1;
                    }
                }
            }
        }

        Ok(count.to_string())
    }
}

struct Matrix {
    pub num_cols: usize,
    pub num_rows: usize,
    cells: Vec<char>,
}

impl Matrix {
    pub fn new(input: &mut dyn BufRead) -> Result<Matrix, AocSolutionError> {
        let mut cells = Vec::new();
        let mut num_cols = 0;
        let mut num_rows = 0;
        for line in input.lines() {
            let line = line?;
            num_rows += 1;
            num_cols = 0;

            for c in line.chars() {
                cells.push(c);
                num_cols += 1;
            }
        }

        Ok(Matrix {
            num_cols,
            num_rows,
            cells,
        })
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        if row >= self.num_rows {
            panic!(
                "illegal row index '{}' >= num_rows='{}'",
                row, self.num_rows
            )
        }
        if col >= self.num_cols {
            panic!(
                "illegal col index '{}' >= num_cols='{}'",
                col, self.num_cols
            )
        }

        self.cells[self.num_rows * row + col]
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

fn word_search(matrix: &Matrix, row: usize, col: usize, word: &[char]) -> u32 {
    let mut count = 0;
    count += word_search_helper(matrix, row, col, word, 0, Direction::Left);
    count += word_search_helper(matrix, row, col, word, 0, Direction::Right);
    count += word_search_helper(matrix, row, col, word, 0, Direction::Up);
    count += word_search_helper(matrix, row, col, word, 0, Direction::Down);
    count += word_search_helper(matrix, row, col, word, 0, Direction::UpperLeft);
    count += word_search_helper(matrix, row, col, word, 0, Direction::UpperRight);
    count += word_search_helper(matrix, row, col, word, 0, Direction::LowerLeft);
    count += word_search_helper(matrix, row, col, word, 0, Direction::LowerRight);
    count
}

fn word_search_helper(
    matrix: &Matrix,
    mut row: usize,
    mut col: usize,
    word: &[char],
    depth: usize,
    direction: Direction,
) -> u32 {
    if matrix.get(row, col) != word[depth] {
        0
    } else if depth + 1 == word.len() {
        1
    } else {
        match direction {
            Direction::Left => {
                if col == matrix.num_cols - 1 {
                    return 0;
                }
                col += 1;
            }
            Direction::Right => {
                if col == 0 {
                    return 0;
                }
                col -= 1;
            }
            Direction::Up => {
                if row == 0 {
                    return 0;
                }
                row -= 1;
            }
            Direction::Down => {
                if row == matrix.num_rows - 1 {
                    return 0;
                }
                row += 1;
            }
            Direction::UpperLeft => {
                if row == 0 || col == 0 {
                    return 0;
                }
                row -= 1;
                col -= 1;
            }
            Direction::UpperRight => {
                if row == 0 || col == matrix.num_cols - 1 {
                    return 0;
                }
                row -= 1;
                col += 1;
            }
            Direction::LowerLeft => {
                if row == matrix.num_rows - 1 || col == 0 {
                    return 0;
                }
                row += 1;
                col -= 1;
            }
            Direction::LowerRight => {
                if row == matrix.num_rows - 1 || col == matrix.num_cols - 1 {
                    return 0;
                }
                row += 1;
                col += 1;
            }
        };

        return word_search_helper(matrix, row, col, word, depth + 1, direction);
    }
}