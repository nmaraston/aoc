use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{parse_char_grid, CharGrid, ICoord};

pub struct Day8Solution {}

impl Solution for Day8Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let mut anitnode_coords = HashSet::new();

        for (coord1, val1) in &grid {
            for (coord2, val2) in &grid {
                if coord1 == coord2 || val1 == '.' || val1 != val2 {
                    continue;
                }

                let icoord1: ICoord = coord1.into();
                let icoord2: ICoord = coord2.into();

                let antinode1 = icoord1 + (icoord1 - icoord2);
                let antinode2 = icoord2 + (icoord2 - icoord1);

                if is_valid_coord(&grid, antinode1) {
                    anitnode_coords.insert(antinode1);
                }

                if is_valid_coord(&grid, antinode2) {
                    anitnode_coords.insert(antinode2);
                }
            }
        }

        Ok(anitnode_coords.len().to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let mut anitnode_coords = HashSet::new();

        for (coord1, val1) in &grid {
            for (coord2, val2) in &grid {
                if coord1 == coord2 || val1 == '.' || val1 != val2 {
                    continue;
                }

                let icoord1: ICoord = coord1.into();
                let icoord2: ICoord = coord2.into();

                let mut antinode1 = icoord1;
                let mut antinode2 = icoord2;

                loop {
                    let valid1 = is_valid_coord(&grid, antinode1);
                    let valid2 = is_valid_coord(&grid, antinode2);

                    if !valid1 && !valid2 {
                        break;
                    }

                    if valid1 {
                        anitnode_coords.insert(antinode1);
                    }

                    if valid2 {
                        anitnode_coords.insert(antinode2);
                    }

                    antinode1 += (icoord2 - icoord1);
                    antinode2 += (icoord1 - icoord2);
                }
            }
        }

        Ok(anitnode_coords.len().to_string())
    }
}

fn is_valid_coord(grid: &CharGrid, coord: ICoord) -> bool {
    coord.row >= 0
        && coord.row < grid.num_rows as i32
        && coord.col >= 0
        && coord.col < grid.num_cols as i32
}
