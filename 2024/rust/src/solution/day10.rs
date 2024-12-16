use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{parse_int_grid, IntGrid, UCoord};

pub struct Day10Solution {}

impl Solution for Day10Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_int_grid(input)?;

        let mut sum = 0;
        let mut peaks: HashSet<UCoord> = HashSet::new();
        for (coord, _) in &grid {
            if grid.at_coord(coord) == 0 {
                search_peaks(&grid, coord, &mut peaks);
                sum += peaks.len();
            }
            peaks.clear();
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_int_grid(input)?;

        let mut sum = 0;
        for (coord, _) in &grid {
            if grid.at_coord(coord) == 0 {
                sum += count_trails(&grid, coord);
            }
        }

        Ok(sum.to_string())
    }
}

fn search_peaks(grid: &IntGrid, coord: UCoord, peaks: &mut HashSet<UCoord>) {
    if grid.at_coord(coord) == 9 {
        peaks.insert(coord);
    } else {
        for neighbour in grid.manhattan_neighbours(coord) {
            if grid.at_coord(neighbour) == grid.at_coord(coord) + 1 {
                search_peaks(grid, neighbour, peaks);
            }
        }
    }
}

fn count_trails(grid: &IntGrid, coord: UCoord) -> u32 {
    if grid.at_coord(coord) == 9 {
        1
    } else {
        let mut count = 0;
        for neighbour in grid.manhattan_neighbours(coord) {
            if grid.at_coord(neighbour) == grid.at_coord(coord) + 1 {
                count += count_trails(grid, neighbour);
            }
        }
        count
    }
}
