use std::fs;
use std::cmp::{min, max};

use crate::solution::Solution;

pub struct Day3Solve {}

impl Solution for Day3Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {
        let (wire1_lines, wire2_lines) = read_lines(input_file_path)?;
        let mut min_dist = std::i32::MAX;
        let origin = Coord::new(0, 0);
        for line1 in &wire1_lines {
            for line2 in &wire2_lines {

                for int in line1.intersection(&line2) {
                    if !int.is_origin() {
                        min_dist = min(min_dist, int.l1_dist(&origin));
                    }
                }

            }
        }

        Ok(min_dist.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        let (wire1_lines, wire2_lines) = read_lines(input_file_path)?;

        let mut min_path_dist = std::i32::MAX;

        let mut wire1_path_dist = 0;
        let mut wire2_path_dist = 0;
        for line1 in &wire1_lines {
            for line2 in &wire2_lines {

                for int in line1.intersection(&line2) {
                    if !int.is_origin() {
                        let path_dist = 
                            int.l1_dist(&line1.start) + 
                            int.l1_dist(&line2.start) + 
                            wire1_path_dist + 
                            wire2_path_dist;
                        min_path_dist = min(min_path_dist, path_dist);
                    }
                }
                wire2_path_dist += line2.len();
            }
            wire1_path_dist += line1.len();
            wire2_path_dist = 0;
        }

        Ok(min_path_dist.to_string())
    }
}

#[derive(Clone, Debug)]
struct Coord { 
    x: i32,
    y: i32
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn is_origin(&self) -> bool {
        (self.x == 0 && self.y == 0)
    }

    pub fn l1_dist(&self, other: &Coord) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

#[derive(Clone, Debug)]
struct Line { 
    start: Coord,
    end: Coord
}

impl Line {
    pub fn len(&self) -> i32 {
        // Assume line is vertical or horizontal to avoid using floats
        if self.start.x == self.end.x {
            (self.end.y - self.start.y).abs()
        } else {
            (self.end.x - self.start.x).abs()
        }
    }

    pub fn x_range(&self) -> Range {
        Range::between(self.start.x, self.end.x)
    }

    pub fn y_range(&self) -> Range {
        Range::between(self.start.y, self.end.y)
    }

    /**
     * Return all points of intersection for this Line and the other given Line. Lines can 
     * intersect at multiple points if they are coincident.
     *
     * Since lines in the problem domain are either strictly vertical or strictly horizontal 
     * computing intersection can be simplified. Here, we project the lines onto the the x-axis and
     * y-axis and compute overlapping ranges. We then take all points in the two dimensional 
     * region expressed by those ranges. This is essentially the Seperating Axis Therom for lines.
     */
    pub fn intersection(&self, other: &Line) -> Vec<Coord> {
        let x_int_range = self.x_range().intersection(&other.x_range());
        let y_int_range = self.y_range().intersection(&other.y_range());

        let mut intersections = Vec::new();
        if let Some(xr) = x_int_range {
            if let Some(yr) = y_int_range {
                for x in xr.l .. (xr.r + 1) {
                    for y in yr.l .. (yr.r + 1) {
                        intersections.push( Coord::new(x, y) );
                    }
                }
            }
        }

        intersections
    }
}

#[derive(Clone, Debug)]
struct Range {
    l: i32, 
    r: i32
}

impl Range {
    pub fn between(p1: i32, p2: i32) -> Range {
        Range { l: min(p1, p2), r: max(p1, p2) }
    }

    /**
     * Return a Range which expresses the intersection of this Range and the other given Range. If 
     * the intersection is empty return None.
     *
     * Ranges are interpreted as closed intervals.
     */
    fn intersection(&self, other: &Range) -> Option<Range> {
        let (left, right) = if self.l <= other.l { (self, other) } else { (other, self) };
        if left.r >= right.l {
            Some( Range { l: right.l, r: min(left.r, right.r) } )
        } else {
            None
        }
    }
}


fn read_lines(input_file_path: &str) -> std::io::Result<(Vec<Line>, Vec<Line>)> {
    let contents = fs::read_to_string(input_file_path)?;
    let serial_lines: Vec<&str> = contents 
        .trim()
        .split("\n")
        .collect();

    Ok((deserialize_lines(serial_lines[0]), deserialize_lines(serial_lines[1])))
}

fn deserialize_lines(serial_lines: &str) -> Vec<Line> {
    let moves: Vec<&str>  = serial_lines.split(",").collect();
    let mut lines = Vec::with_capacity(moves.len());

    let mut start = Coord::new(0, 0);
    for mov in moves {
        let dir = mov.chars().next().unwrap();
        let dist = mov.split_at(1).1.parse::<i32>().unwrap();
        let end = match dir {
            'L' => Coord::new(start.x - dist, start.y),
            'R' => Coord::new(start.x + dist, start.y),
            'D' => Coord::new(start.x, start.y - dist),
            'U' => Coord::new(start.x, start.y + dist),
            _ => panic!("Unknown direction.")
        };
        lines.push( Line { start: start.clone(), end: end.clone() } );
        start = end;
    }

    lines
}

