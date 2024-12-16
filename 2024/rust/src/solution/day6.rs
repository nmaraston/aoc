use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{parse_char_grid, CharGrid, UCoord};

pub struct Day6Solution {}

impl Solution for Day6Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let guard = find_guard(&grid);

        Ok(compute_visited_set(&grid, guard).len().to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut grid = parse_char_grid(input)?;
        let start_guard = find_guard(&grid);
        let mut count = 0;

        // We only need to test for cycles by adding obstuctions to the expected
        // path of the guard.
        let visited = compute_visited_set(&grid, start_guard);
        for UCoord { row, col } in visited {
            // Can't add an obstruction to the guards starting position
            if row == start_guard.coord.row && col == start_guard.coord.col {
                continue;
            }

            // Add obstruction
            grid.set(row, col, '#');

            let mut visited_obstructions: HashSet<Guard> = HashSet::new();
            let mut cur_guard = start_guard;

            while let Some(next) = cur_guard.next_move(&grid) {
                if grid.get(next.row, next.col) == '#' {
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
            grid.set(row, col, '.');
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
    pub coord: UCoord,
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

    pub fn next_move(&self, grid: &CharGrid) -> Option<UCoord> {
        match self.direction {
            Direction::Left => {
                if self.coord.col == 0 {
                    None
                } else {
                    Some(self.coord - UCoord { row: 0, col: 1 })
                }
            }
            Direction::Right => {
                if self.coord.col == grid.num_cols - 1 {
                    None
                } else {
                    Some(self.coord + UCoord { row: 0, col: 1 })
                }
            }
            Direction::Up => {
                if self.coord.row == 0 {
                    None
                } else {
                    Some(self.coord - UCoord { row: 1, col: 0 })
                }
            }
            Direction::Down => {
                if self.coord.row == grid.num_rows - 1 {
                    None
                } else {
                    Some(self.coord + UCoord { row: 1, col: 0 })
                }
            }
        }
    }
}

fn find_guard(grid: &CharGrid) -> Guard {
    for row in 0..grid.num_rows {
        for col in 0..grid.num_cols {
            match grid.get(row, col) {
                '>' => {
                    return Guard {
                        coord: UCoord { row, col },
                        direction: Direction::Left,
                    }
                }
                '<' => {
                    return Guard {
                        coord: UCoord { row, col },
                        direction: Direction::Right,
                    }
                }
                '^' => {
                    return Guard {
                        coord: UCoord { row, col },
                        direction: Direction::Up,
                    }
                }
                'V' => {
                    return Guard {
                        coord: UCoord { row, col },
                        direction: Direction::Down,
                    }
                }
                _ => continue,
            };
        }
    }

    panic!("No guard found in input!");
}

fn compute_visited_set(grid: &CharGrid, mut guard: Guard) -> HashSet<UCoord> {
    let mut visited = HashSet::new();
    visited.insert(guard.coord);

    while let Some(next) = guard.next_move(grid) {
        if grid.get(next.row, next.col) == '#' {
            guard.turn();
        } else {
            visited.insert(next);
            guard.coord = next
        }
    }

    visited
}
