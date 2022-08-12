use std::cmp::max;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use super::Solution;

struct Cord {
    x: i16,
    y: i16,
}

struct Line {
    c1: Cord,
    c2: Cord,
}

// Heavily unchecked helper for parsing a u16 out of a regex capture
fn get_u16_at(caps: &Captures, i: usize) -> i16 {
    let c = caps.get(i);
    c.map(|m| m.as_str().parse::<i16>()).unwrap().unwrap()
}

impl Line {
    fn from(line: &String) -> Line {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        Line {
            c1: Cord {
                x: get_u16_at(&caps, 1),
                y: get_u16_at(&caps, 2),
            },
            c2: Cord {
                x: get_u16_at(&caps, 3),
                y: get_u16_at(&caps, 4),
            },
        }
    }

    fn is_diagonal(&self) -> bool {
        !(self.is_horizontal() || self.is_vertical())
    }

    fn is_horizontal(&self) -> bool {
        self.c1.y == self.c2.y
    }

    fn is_vertical(&self) -> bool {
        self.c1.x == self.c2.x
    }

    fn max_x(&self) -> i16 {
        max(self.c1.x, self.c2.x)
    }

    fn max_y(&self) -> i16 {
        max(self.c1.y, self.c2.y)
    }

    fn render(&self, grid: &mut Vec<Vec<i32>>) {
        let mut x = self.c1.x;
        let mut y = self.c1.y;

        let x_inc = self.inc(self.c1.x, self.c2.x);
        let y_inc = self.inc(self.c1.y, self.c2.y);

        let x_bound = self.c2.x + x_inc;
        let y_bound = self.c2.y + y_inc;

        loop {
            if x == x_bound && y == y_bound {
                break;
            }
            grid[x as usize][y as usize] += 1;
            x = x + x_inc;
            y = y + y_inc;
        }
    }

    fn inc(&self, a: i16, b: i16) -> i16 {
        if a > b {
            -1
        } else if b > a {
            1
        } else {
            0
        }
    }
}

pub struct Day5Solution {}

impl Solution for Day5Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut lines: Vec<Line> = Vec::new();

        // Find max (x,y) to form a bounding box
        let mut max_x = 0;
        let mut max_y = 0;

        for file_line in input.lines() {
            let line = Line::from(&file_line?);
            if !line.is_diagonal() {
                max_x = max(max_x, line.max_x());
                max_y = max(max_y, line.max_y());
                lines.push(line);
            }
        }

        Ok(render_and_count(&lines, max_x, max_y).to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let mut lines: Vec<Line> = Vec::new();

        // Find max (x,y) to form a bounding box
        let mut max_x = 0;
        let mut max_y = 0;

        for file_line in input.lines() {
            let line = Line::from(&file_line?);
            max_x = max(max_x, line.max_x());
            max_y = max(max_y, line.max_y());
            lines.push(line);
        }

        Ok(render_and_count(&lines, max_x, max_y).to_string())
    }
}

fn render_and_count(lines: &Vec<Line>, grid_cols: i16, grid_rows: i16) -> i32 {
    let x_bound = (grid_cols + 1) as usize;
    let y_bound = (grid_rows + 1) as usize;

    let mut grid = vec![vec![0; y_bound]; x_bound];

    for line in lines {
        line.render(&mut grid);
    }

    let mut count = 0;
    for x in 0..x_bound {
        for y in 0..y_bound {
            if grid[x as usize][y as usize] >= 2 {
                count += 1;
            }
        }
    }

    count
}
