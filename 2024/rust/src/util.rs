use std::fmt;
use std::io::BufRead;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord<T> {
    pub row: T,
    pub col: T,
}

pub type UCoord = Coord<usize>;
pub type ICoord = Coord<i32>;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cell<T> {
    pub val: T,
    pub coord: UCoord,
}

pub struct Grid<T> {
    pub num_rows: usize,
    pub num_cols: usize,
    cells: Vec<T>,
}

pub type CharGrid = Grid<char>;
pub type IntGrid = Grid<u32>;

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    pub fn new(rows: usize, cols: usize, default: T) -> Grid<T> {
        Grid {
            num_rows: rows,
            num_cols: cols,
            cells: vec![default; rows * cols],
        }
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

    pub fn get(&self, row: usize, col: usize) -> T {
        self.assert_indicies(row, col);
        self.cells[self.num_rows * row + col]
    }

    pub fn at_coord(&self, coord: UCoord) -> T {
        self.get(coord.row, coord.col)
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.assert_indicies(row, col);
        self.cells[self.num_rows * row + col] = val;
    }

    pub fn manhattan_neighbours(&self, coord: UCoord) -> Vec<UCoord> {
        let mut neighbours = Vec::new();

        if coord.row > 0 {
            neighbours.push(UCoord {
                row: coord.row - 1,
                col: coord.col,
            });
        }
        if coord.row < self.num_rows - 1 {
            neighbours.push(UCoord {
                row: coord.row + 1,
                col: coord.col,
            });
        }
        if coord.col > 0 {
            neighbours.push(UCoord {
                row: coord.row,
                col: coord.col - 1,
            });
        }
        if coord.col < self.num_cols - 1 {
            neighbours.push(UCoord {
                row: coord.row,
                col: coord.col + 1,
            });
        }

        neighbours
    }

    pub fn at_direction(&self, coord: UCoord, direction: Direction) -> Option<Cell<T>> {
        match (direction) {
            Direction::UP => {
                if coord.row == 0 {
                    None
                } else {
                    let res_coord = UCoord {
                        row: coord.row - 1,
                        col: coord.col,
                    };
                    Some(Cell {
                        coord: res_coord,
                        val: self.at_coord(res_coord),
                    })
                }
            }
            Direction::DOWN => {
                if coord.row == self.num_rows - 1 {
                    None
                } else {
                    let res_coord = UCoord {
                        row: coord.row + 1,
                        col: coord.col,
                    };
                    Some(Cell {
                        coord: res_coord,
                        val: self.at_coord(res_coord),
                    })
                }
            }
            Direction::LEFT => {
                if coord.col == 0 {
                    None
                } else {
                    let res_coord = UCoord {
                        row: coord.row,
                        col: coord.col - 1,
                    };
                    Some(Cell {
                        coord: res_coord,
                        val: self.at_coord(res_coord),
                    })
                }
            }
            Direction::RIGHT => {
                if coord.col == self.num_cols - 1 {
                    None
                } else {
                    let res_coord = UCoord {
                        row: coord.row,
                        col: coord.col + 1,
                    };
                    Some(Cell {
                        coord: res_coord,
                        val: self.at_coord(res_coord),
                    })
                }
            }
        }
    }
}

pub fn parse_char_grid(input: &mut dyn BufRead) -> Result<CharGrid, std::io::Error> {
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

    Ok(Grid {
        num_cols,
        num_rows,
        cells,
    })
}

pub fn parse_int_grid(input: &mut dyn BufRead) -> Result<IntGrid, std::io::Error> {
    let mut cells = Vec::new();
    let mut num_cols = 0;
    let mut num_rows = 0;
    for line in input.lines() {
        let line = line?;
        num_rows += 1;
        num_cols = 0;

        for c in line.chars() {
            cells.push(c.to_digit(10).unwrap());
            num_cols += 1;
        }
    }

    Ok(Grid {
        num_cols,
        num_rows,
        cells,
    })
}

impl<T> fmt::Display for Grid<T>
where
    T: Clone + Copy + fmt::Display,
{
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

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    index: usize,
}

impl<'a, T> IntoIterator for &'a Grid<T>
where
    T: Clone + Copy,
{
    type Item = (UCoord, T);
    type IntoIter = GridIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIterator {
            grid: self,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T>
where
    T: Clone + Copy,
{
    type Item = (UCoord, T);

    fn next(&mut self) -> Option<(UCoord, T)> {
        if self.index >= self.grid.cells.len() {
            None
        } else {
            let val = self.grid.cells[self.index];
            let row = self.index / self.grid.num_rows;
            let col = self.index % self.grid.num_cols;

            self.index += 1;
            Some((Coord { row, col }, val))
        }
    }
}
