use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use crate::util::{parse_char_grid, CharGrid, Direction, IntGrid, UCoord};

use super::{AocSolutionError, Solution};

pub struct Day16Solution {}

impl Solution for Day16Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let (start, end) = find_terminal_coords(&grid);
        let dist_grid = shortest_dist_grid(&grid, start, Direction::Right);
        Ok(dist_grid.at_coord(end).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let (start, end) = find_terminal_coords(&grid);
        let dist_grid = shortest_dist_grid(&grid, start, Direction::Right);
        let shortest_dist = dist_grid.at_coord(end);

        // End direction is determined to be Left from looking at the input.
        // Set `start_dir = Direction::Down` when testing with test inputs.
        let reverse_dist_grid = shortest_dist_grid(&grid, end, Direction::Left);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(end);

        while let Some(coord) = queue.pop_front() {
            visited.insert(coord);

            for next in dist_grid.manhattan_neighbours(coord) {
                let cost_from_start = dist_grid.at_coord(next);
                let cost_to_end = reverse_dist_grid.at_coord(next);

                if !visited.contains(&next)
                    && grid.at_coord(next) != '#'
                    && cost_from_start + cost_to_end <= shortest_dist
                {
                    queue.push_back(next);
                }
            }
        }

        Ok(visited.len().to_string())
    }
}

fn find_terminal_coords(grid: &CharGrid) -> (UCoord, UCoord) {
    let mut start = None;
    let mut end = None;

    for (coord, val) in grid {
        if val == 'S' {
            start = Some(coord)
        } else if val == 'E' {
            end = Some(coord)
        }
    }

    match (start, end) {
        (Some(s), Some(e)) => (s, e),
        _ => panic!("Start or end coords marked with 'S' or 'E' are missing from input!"),
    }
}

fn shortest_dist_grid(grid: &CharGrid, start: UCoord, start_dir: Direction) -> IntGrid {
    let mut dist_grid = IntGrid::new(grid.num_rows, grid.num_cols, u32::MAX);

    let mut queue = VecDeque::new();
    queue.push_back((start, start_dir, 0));
    dist_grid.set(start.row, start.col, 0);

    while let Some((coord, curr_dir, curr_cost)) = queue.pop_front() {
        let next_dirs = match curr_dir {
            Direction::Up | Direction::Down => [curr_dir, Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [curr_dir, Direction::Up, Direction::Down],
        };

        for next_dir in next_dirs {
            if let Some(next) = grid.at_direction(coord, next_dir) {
                let cost = if next_dir == curr_dir { 1 } else { 1001 };

                if grid.at_coord(next.coord) == '#'
                    || dist_grid.at_coord(next.coord) <= (curr_cost + cost)
                {
                    continue;
                }

                queue.push_back((next.coord, next_dir, curr_cost + cost));
                dist_grid.set(next.coord.row, next.coord.col, curr_cost + cost);
            }
        }
    }

    dist_grid
}
