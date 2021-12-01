use std::io::BufRead;

use crate::solution::Solution;

pub struct Day3Solve {}

struct Map {
    col_count: usize,
    cells: Vec<bool>
}

impl Map {

    fn load(input: &mut dyn BufRead) -> Self {
        let mut col_count = 0;
        let mut cells: Vec<bool> = vec![];
        for line in input.lines() {
            let line = line.unwrap();

            // Only set col_count once
            if col_count == 0 {
                col_count = line.len();
            }

            for c in line.chars() {
                cells.push(c == '#')
            }
        }

        Map { col_count, cells }
    }

    fn get(&self, row: usize, col: usize) -> Option<bool> {
        let i = (row * self.col_count) + (col % self.col_count);

        // Is there a better way to do this without dereferencing?
        self.cells.get(i).map(|b| *b)
    }

    fn traverse(&self, right: usize, down: usize) -> u32 {
        let (mut row, mut col, mut count) = (0, 0, 0);
        while let Some(cell) = self.get(row, col) {
            count += if cell { 1 } else { 0 };
            row += down;
            col += right;
        }

        count
    }


}

impl Solution for Day3Solve {

    fn part_1(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let map = Map::load(input);
        let count = map.traverse(3, 1);
        Ok(count.to_string())
    }

    fn part_2(&self, input: &mut dyn BufRead) -> std::io::Result<String> {
        let map = Map::load(input);
        let directions = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut product = 1;
        for (right, down) in directions {
            product *= map.traverse(right, down);
        }

        Ok(product.to_string())
    }
}

