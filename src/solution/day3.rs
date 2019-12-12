use std::fs;
use std::cmp::{min, max};

use crate::solution::Solution;

pub struct Day3Solve {}

impl Solution for Day3Solve {

    fn part_1(&self, input_file_path: &str) -> std::io::Result<String> {

        let mut min_distance = std::u32::MAX;
        let (wire1_lines, wire2_lines) = read_lines(input_file_path)?;

        for line1 in &wire1_lines {
            for line2 in &wire2_lines {

                let intersections = line_intersection(&line1, &line2);

                for int in intersections {
                    if int.x != 0 || int.y != 0 {
                        let l1_origin_dist = (int.x.abs() + int.y.abs()) as u32;
                        if l1_origin_dist < min_distance {
                            min_distance = l1_origin_dist;
                        }
                    }
                }
            }
        }

        Ok(min_distance.to_string())
    }

    fn part_2(&self, input_file_path: &str) -> std::io::Result<String> {
        Ok("Unimplemented".to_owned())
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
}

#[derive(Clone, Debug)]
struct Line { 
    start: Coord,
    end: Coord
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
}

/**
 * Return all points of intersection for the two given line segments. Lines can intersect at 
 * multiple points if they are coincident.
 *
 * Since lines in the problem domain are either strictly vertical or strictly horizontal computing
 * intersection can be simplified. Here, we project the lines onto the the x-axis and y-axis and
 * compute overlapping ranges. We then take all points in the two dimensional region expressed by 
 * those ranges. This is essentially the Seperating Axis Therom for lines.
 */
fn line_intersection(line1: &Line, line2: &Line) -> Vec<Coord> {
    let mut intersections = Vec::new();

    let x_int_range = range_intersection( 
        &Range::between(line1.start.x, line1.end.x),
        &Range::between(line2.start.x, line2.end.x));
    let y_int_range = range_intersection( 
        &Range::between(line1.start.y, line1.end.y),
        &Range::between(line2.start.y, line2.end.y));

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

/**
 * Return a range which expresses the intersection of the two given Ranges. If the intersection is
 * empty return None.
 *
 * Ranges are interpreted as closed intervals.
 */
fn range_intersection(r1: &Range, r2: &Range) -> Option<Range> {
    let (left, right) = if r1.l <= r2.l { (r1, r2) } else { (r2, r1) };
    if left.r >= right.l {
        Some( Range { l: right.l, r: min(left.r, right.r) } )
    } else {
        None
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

