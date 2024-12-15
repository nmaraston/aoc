use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{iCoord, CharMatrix};

pub struct Day8Solution {}

impl Solution for Day8Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let matrix = CharMatrix::from_input(input)?;
        let mut anitnode_coords = HashSet::new();

        for (coord1, val1) in &matrix {
            for (coord2, val2) in &matrix {
                if coord1 == coord2 || val1 == '.' || val1 != val2 {
                    continue;
                }

                let icoord1: iCoord = coord1.into();
                let icoord2: iCoord = coord2.into();

                let antinode1 = icoord1 + (icoord1 - icoord2);
                let antinode2 = icoord2 + (icoord2 - icoord1);

                if is_valid_coord(&matrix, antinode1) {
                    anitnode_coords.insert(antinode1);
                }

                if is_valid_coord(&matrix, antinode2) {
                    anitnode_coords.insert(antinode2);
                }
            }
        }

        Ok(anitnode_coords.len().to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let matrix = CharMatrix::from_input(input)?;
        let mut anitnode_coords = HashSet::new();

        for (coord1, val1) in &matrix {
            for (coord2, val2) in &matrix {
                if coord1 == coord2 || val1 == '.' || val1 != val2 {
                    continue;
                }

                let icoord1: iCoord = coord1.into();
                let icoord2: iCoord = coord2.into();

                let mut antinode1 = icoord1;
                let mut antinode2 = icoord2;

                loop {
                    let valid1 = is_valid_coord(&matrix, antinode1);
                    let valid2 = is_valid_coord(&matrix, antinode2);

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

fn is_valid_coord(matrix: &CharMatrix, coord: iCoord) -> bool {
    coord.row >= 0
        && coord.row < matrix.num_rows as i32
        && coord.col >= 0
        && coord.col < matrix.num_cols as i32
}
