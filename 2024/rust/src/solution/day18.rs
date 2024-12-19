use std::collections::{HashSet, VecDeque};
use std::{io::BufRead, num::ParseIntError};

use super::{AocSolutionError, Solution};
use crate::util::{CharGrid, IntGrid, UCoord};

pub struct Day18Solution {}

// Set MAX_COORD=6 and FALLEN_BYTES=12 when working with test input.
const MAX_COORD: usize = 70;
const FALLEN_BYTES: usize = 1024;

const START: UCoord = UCoord { row: 0, col: 0 };
const EXIT: UCoord = UCoord {
    row: MAX_COORD,
    col: MAX_COORD,
};

impl Solution for Day18Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut grid = CharGrid::new(MAX_COORD + 1, MAX_COORD + 1, '.');

        let mut line_iter = input.lines();
        for _ in 0..FALLEN_BYTES {
            let line = line_iter.next().unwrap()?;
            let coord = read_coord(line)?;
            grid.set(coord.row, coord.col, '#');
        }

        Ok(shortest_path(&grid, START, EXIT).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let mut grid = CharGrid::new(MAX_COORD + 1, MAX_COORD + 1, '.');

        let mut res = START;
        for (byte_num, line) in input.lines().enumerate() {
            let line = line?;
            let coord = read_coord(line)?;
            grid.set(coord.row, coord.col, '#');

            if byte_num >= FALLEN_BYTES && shortest_path(&grid, START, EXIT) == u32::MAX {
                res = coord;
                break;
            }
        }

        Ok(format!("{},{}", res.row, res.col))
    }
}

fn read_coord(line: String) -> Result<UCoord, AocSolutionError> {
    let nums = line
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    Ok(UCoord {
        row: nums[0],
        col: nums[1],
    })
}

fn shortest_path(grid: &CharGrid, start: UCoord, end: UCoord) -> u32 {
    let mut visited = HashSet::new();
    let mut dist_grid = IntGrid::new(grid.num_rows, grid.num_cols, u32::MAX);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((coord, dist)) = queue.pop_front() {
        dist_grid.set(coord.row, coord.col, dist);

        for neighbour in grid.manhattan_neighbours(coord) {
            if !visited.contains(&neighbour) && grid.at_coord(neighbour) == '.' {
                queue.push_back((neighbour, dist + 1));
                visited.insert(neighbour);
            }
        }
    }

    dist_grid.at_coord(end)
}
