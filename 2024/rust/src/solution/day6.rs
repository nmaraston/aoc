use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{CharMatrix, Coord};

pub struct Day6Solution {}

impl Solution for Day6Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let matrix = CharMatrix::from_input(input)?;
        let guard = find_guard(&matrix);

        Ok(compute_visited_set(&matrix, guard).len().to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut matrix = CharMatrix::from_input(input)?;
        let start_guard = find_guard(&matrix);
        let mut count = 0;

        // We only need to test for cycles by adding obstuctions to the expected
        // path of the guard.
        let visited = compute_visited_set(&matrix, start_guard);
        for Coord { row, col } in visited {
            // Can't add an obstruction to the guards starting position
            if row == start_guard.coord.row && col == start_guard.coord.col {
                continue;
            }

            // Add obstruction
            matrix.set(row, col, '#');

            let mut visited_obstructions: HashSet<Guard> = HashSet::new();
            let mut cur_guard = start_guard;

            while let Some(next) = cur_guard.next_move(&matrix) {
                if matrix.get(next.row, next.col) == '#' {
                    // A cycle occurs when the guard visits the same
                    // obstruction twice with the same orientation.
                    if visited_obstructions.contains(&cur_guard) {
                        count += 1;
                        break;
                    }

                    visited_obstructions.insert(cur_guard);
                    cur_guard.turn();
                } else {
                    cur_guard.coord = next;
                }
            }

            // Remove obstruction
            matrix.set(row, col, '.');
        }

        Ok(count.to_string())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
    pub coord: Coord,
    pub direction: Direction,
}

impl Guard {
    pub fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        };
    }

    pub fn next_move(&self, matrix: &CharMatrix) -> Option<Coord> {
        match self.direction {
            Direction::Left => {
                if self.coord.col == 0 {
                    None
                } else {
                    Some(self.coord - Coord { row: 0, col: 1 })
                }
            }
            Direction::Right => {
                if self.coord.col == matrix.num_cols - 1 {
                    None
                } else {
                    Some(self.coord + Coord { row: 0, col: 1 })
                }
            }
            Direction::Up => {
                if self.coord.row == 0 {
                    None
                } else {
                    Some(self.coord - Coord { row: 1, col: 0 })
                }
            }
            Direction::Down => {
                if self.coord.row == matrix.num_rows - 1 {
                    None
                } else {
                    Some(self.coord + Coord { row: 1, col: 0 })
                }
            }
        }
    }
}

fn find_guard(matrix: &CharMatrix) -> Guard {
    for row in 0..matrix.num_rows {
        for col in 0..matrix.num_cols {
            match matrix.get(row, col) {
                '>' => {
                    return Guard {
                        coord: Coord { row, col },
                        direction: Direction::Left,
                    }
                }
                '<' => {
                    return Guard {
                        coord: Coord { row, col },
                        direction: Direction::Right,
                    }
                }
                '^' => {
                    return Guard {
                        coord: Coord { row, col },
                        direction: Direction::Up,
                    }
                }
                'V' => {
                    return Guard {
                        coord: Coord { row, col },
                        direction: Direction::Down,
                    }
                }
                _ => continue,
            };
        }
    }

    panic!("No guard found in input!");
}

fn compute_visited_set(matrix: &CharMatrix, mut guard: Guard) -> HashSet<Coord> {
    let mut visited = HashSet::new();
    visited.insert(guard.coord);

    while let Some(next) = guard.next_move(matrix) {
        if matrix.get(next.row, next.col) == '#' {
            guard.turn();
        } else {
            visited.insert(next);
            guard.coord = next
        }
    }

    visited
}
