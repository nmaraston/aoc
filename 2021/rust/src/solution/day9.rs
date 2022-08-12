use std::io::BufRead;

use super::Solution;

struct Cell<T> {
    pub value: T,
    pub row: usize,
    pub column: usize,
}

impl<T> Cell<T> {
    fn new(value: T, row: usize, column: usize) -> Cell<T> {
        Cell { value, row, column }
    }
}

struct Grid<T> {
    pub nrows: usize,
    pub ncols: usize,
    cells: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    fn new(nrows: usize, ncols: usize, cells: Vec<T>) -> Grid<T> {
        Grid {
            nrows,
            ncols,
            cells,
        }
    }

    fn new_with_default(nrows: usize, ncols: usize, default: T) -> Grid<T> {
        let mut cells = Vec::with_capacity(nrows * ncols);
        for _ in 0..(nrows * ncols) {
            cells.push(default);
        }
        Grid {
            nrows,
            ncols,
            cells,
        }
    }

    fn get_unchecked(&self, row: usize, column: usize) -> T {
        self.get(row, column).unwrap()
    }

    fn get(&self, row: usize, column: usize) -> Option<T> {
        if row >= self.nrows {
            None
        } else if column >= self.ncols {
            None
        } else {
            Some(self.cells[(row * self.ncols) + column])
        }
    }

    fn put(&mut self, row: usize, column: usize, value: T) {
        self.cells[(row * self.ncols) + column] = value;
    }

    fn neighbours(&self, row: usize, column: usize) -> Vec<Cell<T>> {
        let mut result = Vec::new();

        if row > 0 {
            let value = self.get_unchecked(row - 1, column);
            result.push(Cell::new(value, row - 1, column));
        }
        if row < self.nrows - 1 {
            let value = self.get_unchecked(row + 1, column);
            result.push(Cell::new(value, row + 1, column));
        }
        if column > 0 {
            let value = self.get_unchecked(row, column - 1);
            result.push(Cell::new(value, row, column - 1));
        }
        if column < self.ncols - 1 {
            let value = self.get_unchecked(row, column + 1);
            result.push(Cell::new(value, row, column + 1));
        }

        result
    }
}

pub struct Day9Solution {}

impl Solution for Day9Solution {
    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let grid = read_to_grid(input)?;
        let result: u32 = find_local_mins(&grid)
            .into_iter()
            .map(|cord| grid.get(cord.0, cord.1).unwrap() + 1)
            .sum();

        Ok(result.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let grid = read_to_grid(input)?;

        let mut basin_sizes: Vec<u32> = find_local_mins(&grid)
            .into_iter()
            .map(|cord| compute_basin_size(&grid, cord.0, cord.1))
            .collect();
        basin_sizes.sort_by(|x, y| y.cmp(x));

        let result = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

        Ok(result.to_string())
    }
}

fn read_to_grid(input: &mut dyn BufRead) -> std::io::Result<Grid<u32>> {
    let mut nrows: usize = 0;
    let mut ncols: usize = 0;
    let mut cells = Vec::new();
    for line in input.lines() {
        let line = line?;
        nrows += 1;
        ncols = line.len();
        for c in line.chars() {
            cells.push(c.to_digit(10).unwrap());
        }
    }

    Ok(Grid::new(nrows, ncols, cells))
}

fn find_local_mins(input_grid: &Grid<u32>) -> Vec<(usize, usize)> {
    let mut local_mins = Vec::new();

    for row in 0..input_grid.nrows {
        for col in 0..input_grid.ncols {
            let cell = input_grid.get_unchecked(row, col);

            let mut local_min = true;
            for neighbour in input_grid.neighbours(row, col) {
                local_min = local_min && cell < neighbour.value;
            }

            if local_min {
                local_mins.push((row, col));
            }
        }
    }

    local_mins
}

fn compute_basin_size(input_grid: &Grid<u32>, row: usize, column: usize) -> u32 {
    let mut visited_grid: Grid<bool> =
        Grid::new_with_default(input_grid.nrows, input_grid.ncols, false);
    compute_basin_size_helper(input_grid, &mut visited_grid, row, column)
}

fn compute_basin_size_helper(
    input_grid: &Grid<u32>,
    visited_grid: &mut Grid<bool>,
    row: usize,
    column: usize,
) -> u32 {
    visited_grid.put(row, column, true);

    let value = input_grid.get_unchecked(row, column);
    let mut size = 1;

    for neighbour in input_grid.neighbours(row, column) {
        let visited = visited_grid.get_unchecked(neighbour.row, neighbour.column);
        if !visited && neighbour.value != 9 && value < neighbour.value {
            size += compute_basin_size_helper(
                input_grid,
                visited_grid,
                neighbour.row,
                neighbour.column,
            );
        }
    }

    size
}
