use std::fmt;
use std::io::BufRead;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T> {
    pub row: T,
    pub col: T,
}

impl<T> Add for Coord<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl<T> AddAssign for Coord<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.row += other.row;
        self.col += other.col;
    }
}

impl<T> Sub for Coord<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            row: self.row - other.row,
            col: self.col - other.col,
        }
    }
}

impl Into<Coord<i32>> for Coord<usize> {
    fn into(self) -> Coord<i32> {
        Coord {
            row: self.row as i32,
            col: self.col as i32,
        }
    }
}

pub type uCoord = Coord<usize>;
pub type iCoord = Coord<i32>;

pub struct CharMatrix {
    pub num_rows: usize,
    pub num_cols: usize,
    cells: Vec<char>,
}

impl CharMatrix {
    pub fn new(rows: usize, cols: usize, default_char: char) -> CharMatrix {
        CharMatrix {
            num_rows: rows,
            num_cols: cols,
            cells: vec![default_char; rows * cols],
        }
    }

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

pub struct CharMatrixIterator<'a> {
    matrix: &'a CharMatrix,
    index: usize,
}

impl<'a> IntoIterator for &'a CharMatrix {
    type Item = (Coord<usize>, char);
    type IntoIter = CharMatrixIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CharMatrixIterator {
            matrix: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for CharMatrixIterator<'a> {
    type Item = (Coord<usize>, char);

    fn next(&mut self) -> Option<(Coord<usize>, char)> {
        if self.index >= self.matrix.cells.len() {
            None
        } else {
            let val = self.matrix.cells[self.index];
            let row = self.index / self.matrix.num_rows;
            let col = self.index % self.matrix.num_cols;

            self.index += 1;
            Some((Coord { row, col }, val))
        }
    }
}
