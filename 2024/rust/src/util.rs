use std::fmt;
use std::io::BufRead;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

pub struct CharMatrix {
    pub num_cols: usize,
    pub num_rows: usize,
    cells: Vec<char>,
}

impl CharMatrix {
    pub fn from_input(input: &mut dyn BufRead) -> Result<CharMatrix, std::io::Error> {
        let mut cells = Vec::new();
        let mut num_cols = 0;
        let mut num_rows = 0;
        for line in input.lines() {
            let line = line?;
            num_rows += 1;
            num_cols = 0;

            for c in line.chars() {
                cells.push(c);
                num_cols += 1;
            }
        }

        Ok(CharMatrix {
            num_cols,
            num_rows,
            cells,
        })
    }

    fn assert_indicies(&self, row: usize, col: usize) {
        assert!(
            row < self.num_rows,
            "illegal row index {} >= num_rows={}",
            row,
            self.num_rows
        );
        assert!(
            col < self.num_cols,
            "illegal col index {} >= num_cols={}",
            col,
            self.num_cols
        );
    }

    pub fn get(&self, row: usize, col: usize) -> char {
        self.assert_indicies(row, col);
        self.cells[self.num_rows * row + col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: char) {
        self.assert_indicies(row, col);
        self.cells[self.num_rows * row + col] = val;
    }
}

impl fmt::Display for CharMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                if col != self.num_cols - 1 {
                    write!(f, "{} ", self.get(row, col))?;
                } else {
                    writeln!(f, "{}", self.get(row, col))?;
                }
            }
        }

        Ok(())
    }
}
