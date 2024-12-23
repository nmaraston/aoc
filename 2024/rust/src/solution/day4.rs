use std::collections::HashSet;
use std::io::BufRead;

use crate::util::{parse_char_grid, CharGrid};

use super::{AocSolutionError, Solution};

pub struct Day4Solution {}

impl Solution for Day4Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;

        let mut count = 0;
        let word = ['X', 'M', 'A', 'S'];
        for row in 0..grid.num_rows {
            for col in 0..grid.num_cols {
                count += word_search(&grid, row, col, &word);
            }
        }

        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;

        let m_s_set = HashSet::from(['M', 'S']);
        let mut count = 0;
        for row in 1..(grid.num_rows - 1) {
            for col in 1..(grid.num_cols - 1) {
                if grid.get(row, col) == 'A' {
                    let diag1 =
                        HashSet::from([grid.get(row - 1, col - 1), grid.get(row + 1, col + 1)]);
                    let diag2 =
                        HashSet::from([grid.get(row - 1, col + 1), grid.get(row + 1, col - 1)]);

                    if diag1 == m_s_set && diag2 == m_s_set {
                        count += 1;
                    }
                }
            }
        }

        Ok(count.to_string())
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

fn word_search(grid: &CharGrid, row: usize, col: usize, word: &[char]) -> u32 {
    let mut count = 0;
    count += word_search_helper(grid, row, col, word, 0, Direction::Left);
    count += word_search_helper(grid, row, col, word, 0, Direction::Right);
    count += word_search_helper(grid, row, col, word, 0, Direction::Up);
    count += word_search_helper(grid, row, col, word, 0, Direction::Down);
    count += word_search_helper(grid, row, col, word, 0, Direction::UpperLeft);
    count += word_search_helper(grid, row, col, word, 0, Direction::UpperRight);
    count += word_search_helper(grid, row, col, word, 0, Direction::LowerLeft);
    count += word_search_helper(grid, row, col, word, 0, Direction::LowerRight);
    count
}

fn word_search_helper(
    grid: &CharGrid,
    mut row: usize,
    mut col: usize,
    word: &[char],
    depth: usize,
    direction: Direction,
) -> u32 {
    if grid.get(row, col) != word[depth] {
        0
    } else if depth + 1 == word.len() {
        1
    } else {
        match direction {
            Direction::Left => {
                if col == grid.num_cols - 1 {
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
                if row == grid.num_rows - 1 {
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
                if row == 0 || col == grid.num_cols - 1 {
                    return 0;
                }
                row -= 1;
                col += 1;
            }
            Direction::LowerLeft => {
                if row == grid.num_rows - 1 || col == 0 {
                    return 0;
                }
                row += 1;
                col -= 1;
            }
            Direction::LowerRight => {
                if row == grid.num_rows - 1 || col == grid.num_cols - 1 {
                    return 0;
                }
                row += 1;
                col += 1;
            }
        };

        return word_search_helper(grid, row, col, word, depth + 1, direction);
    }
}
