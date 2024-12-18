use std::collections::HashSet;
use std::io::BufRead;

use super::{AocSolutionError, Solution};
use crate::util::{parse_char_grid, CharGrid, Direction, UCoord};

pub struct Day12Solution {}

impl Solution for Day12Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;
        let mut sum = 0;
        let mut visited = HashSet::new();
        for (coord, _) in &grid {
            if !visited.contains(&coord) {
                let area = find_area(&grid, coord, &mut visited);
                let edges = find_edges(&grid, coord);
                sum += area * (edges.len() as u32);
            }
        }

        Ok(sum.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> Result<String, AocSolutionError> {
        let grid = parse_char_grid(input)?;

        let mut sum = 0;
        let mut visited = HashSet::new();
        for (coord, _) in &grid {
            if !visited.contains(&coord) {
                let area = find_area(&grid, coord, &mut visited);
                let edges = find_edges(&grid, coord);
                sum += area * count_sides(&edges);
            }
        }

        Ok(sum.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    direction: Direction,
    coord: UCoord,
}

impl Edge {
    fn new(direction: Direction, coord: UCoord) -> Edge {
        Edge { direction, coord }
    }
}

fn find_area(grid: &CharGrid, coord: UCoord, visited: &mut HashSet<UCoord>) -> u32 {
    if visited.contains(&coord) {
        0
    } else {
        visited.insert(coord);
        let mut area = 1;
        for neighbour in grid.manhattan_neighbours(coord) {
            if grid.at_coord(coord) == grid.at_coord(neighbour) {
                area += find_area(grid, neighbour, visited)
            }
        }
        area
    }
}

fn find_edges(grid: &CharGrid, coord: UCoord) -> Vec<Edge> {
    let mut edges = Vec::new();
    let mut visited = HashSet::new();
    find_edges_helper(grid, coord, &mut edges, &mut visited);
    edges
}

fn find_edges_helper(
    grid: &CharGrid,
    coord: UCoord,
    edges: &mut Vec<Edge>,
    visited: &mut HashSet<UCoord>,
) {
    if !visited.contains(&coord) {
        visited.insert(coord);

        let mut discover_edge = |direction: Direction| {
            if let Some(next) = grid.at_direction(coord, direction) {
                if grid.at_coord(coord) != next.val {
                    edges.push(Edge::new(direction, coord));
                } else {
                    find_edges_helper(grid, next.coord, edges, visited);
                }
            } else {
                edges.push(Edge::new(direction, coord));
            }
        };

        discover_edge(Direction::UP);
        discover_edge(Direction::DOWN);
        discover_edge(Direction::LEFT);
        discover_edge(Direction::RIGHT);
    }
}

fn count_sides(edges: &Vec<Edge>) -> u32 {
    let mut sides: Vec<Vec<Edge>> = edges.into_iter().map(|e| vec![*e]).collect();

    // Merge edge collections that contain adjacent edges until no more merges can be found.
    // Only merge one edge collection at a time otherwise manging indicies is difficult.
    loop {
        let mut merge = false;
        let mut merge_indicies: Vec<(usize, usize)> = Vec::new();
        'outer: for (s1i, s1) in sides.iter().enumerate() {
            for (s2i, s2) in sides.iter().enumerate() {
                if s1i == s2i {
                    break;
                }

                for e1 in s1 {
                    for e2 in s2 {
                        if same_side(e1, e2) {
                            merge = true;
                            merge_indicies.push((s1i, s2i));
                            break 'outer;
                        }
                    }
                }
            }
        }

        for (s1i, s2i) in merge_indicies {
            // There is probably a way to mutate edge collections in place but to avoid
            // fighting with the borrow checker for now just create new edge Vecs when merging.
            let mut new_side = Vec::new();
            new_side.extend(sides.remove(s1i.max(s2i)));
            new_side.extend(sides.remove(s1i.min(s2i)));
            sides.push(new_side);
        }

        if !merge {
            break;
        }
    }

    sides.len() as u32
}

fn same_side(e1: &Edge, e2: &Edge) -> bool {
    let same_vertical_side = (e1.direction == Direction::LEFT || e1.direction == Direction::RIGHT)
        && e1.direction == e2.direction
        && e1.coord.col == e2.coord.col
        && e1.coord.row.abs_diff(e2.coord.row) == 1;
    let same_horizontal_side = (e1.direction == Direction::UP || e1.direction == Direction::DOWN)
        && e1.direction == e2.direction
        && e1.coord.row == e2.coord.row
        && e1.coord.col.abs_diff(e2.coord.col) == 1;

    same_vertical_side || same_horizontal_side
}
