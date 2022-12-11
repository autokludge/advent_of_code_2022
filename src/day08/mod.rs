pub mod input;
pub mod part1;
pub mod part2;

use std::{fmt::Display, vec};

use crate::{Output, Part};

#[derive(Debug)]
pub struct TreeGrid {
    grid: Vec<u8>,
    grid_transposed: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl std::fmt::Display for TreeGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cols:{}  rows:{}", self.cols, self.rows);
        writeln!(f, "grid:");
        for row in self.grid.chunks(self.cols) {
            writeln!(f, "{:?}", row);
        }
        Ok(())
    }
}

impl TreeGrid {
    pub fn new(grid: Vec<u8>, rows: usize, cols: usize) -> Self {
        let grid_transposed = vec![];
        let mut s = Self {
            grid,
            grid_transposed,
            rows,
            cols,
        };
        s.calculate_transposed();
        s
    }

    fn calculate_transposed(&mut self) {
        let cols = self.cols;
        let rows = self.rows;
        let mut transposed_grid = vec![];
        for i in (1..=cols) {
            for j in (1..=rows) {
                transposed_grid.push(self.cell(i, j).clone());
            }
        }
        self.grid_transposed = transposed_grid;
    }

    pub fn cell(&self, col: usize, row: usize) -> u8 {
        self.grid[self.coord_to_idx(col, row)]
    }

    fn coord_to_idx(&self, col: usize, row: usize) -> usize {
        assert!(col <= self.cols);
        assert!(row <= self.rows);
        let col_0_idx = col - 1;
        let row_0_idx = row - 1;
        (row_0_idx * self.cols) + col_0_idx
    }

    fn coord_to_idx_transposed(&self, col: usize, row: usize) -> usize {
        assert!(row <= self.cols);
        assert!(col <= self.rows);
        let col_0_idx = row - 1;
        let row_0_idx = col - 1;
        (col_0_idx * self.rows) + row_0_idx
    }

    fn idx_to_cell(&self, idx: usize) -> (usize, usize) {
        let col_0_idx = idx % self.cols;
        let row_0_idx = idx / self.cols;

        (col_0_idx + 1, row_0_idx + 1)
    }

    pub fn row(&self, row: usize) -> &[u8] {
        let r_idx = self.coord_to_idx(1, row);
        let row_len = r_idx + self.cols;
        &self.grid[r_idx..row_len]
    }
    pub fn col(&self, col: usize) -> &[u8] {
        let c_idx = self.coord_to_idx_transposed(1, col);
        let col_len = c_idx + self.rows;
        &self.grid_transposed[c_idx..col_len]
    }

    pub fn cell_transposed(&self, col: usize, row: usize) -> u8 {
        self.grid_transposed[self.coord_to_idx_transposed(col, row)]
    }

    pub fn cell_hidden_from_outside(&self, col: usize, row: usize) -> bool {
        if col == 1 || row == 1 || col == self.cols || row == self.rows {
            return false;
        }
        let tree_height = self.cell(col, row);

        let grid_col = self.col(col);
        let grid_row = self.row(row);

        //look north
        let north = &grid_col[0..row - 1];
        let north_max = *north.iter().max().unwrap();

        //look south
        let south = &grid_col[row..];
        let south_max = *south.iter().max().unwrap();

        //look east
        let east = &grid_row[col..];
        let east_max = *east.iter().max().unwrap();

        //look west
        let west = &grid_row[0..col - 1];
        let west_max = *west.iter().max().unwrap();

        north_max >= tree_height
            && east_max >= tree_height
            && south_max >= tree_height
            && west_max >= tree_height
    }

    pub fn cell_visible_from_outside(&self, col: usize, row: usize) -> bool {
        !self.cell_hidden_from_outside(col, row)
    }

    pub fn tree_scenic_score(&self, col: usize, row: usize) -> usize {
        let tree_height = self.cell(col, row);
        let grid_col = self.col(col);
        let grid_row = self.row(row);

        //look north
        let north: Vec<u8> = grid_col[0..row - 1].to_owned().into_iter().rev().collect();
        let north_seen = match north.iter().position(|&x| x >= tree_height) {
            Some(x) => x + 1,
            None => {
                if row == 0 {
                    0
                } else {
                    north.len()
                }
            }
        };

        //look south
        let south = &grid_col[row..];
        let south_seen = match south.iter().position(|&x| x >= tree_height) {
            Some(x) => x + 1,
            None => {
                if row == 0 {
                    0
                } else {
                    south.len()
                }
            }
        };

        //look east
        let east = &grid_row[col..];
        let east_seen = match east.iter().position(|&x| x >= tree_height) {
            Some(x) => x + 1,
            None => {
                if row == 0 {
                    0
                } else {
                    east.len()
                }
            }
        };

        //look west
        let west: Vec<u8> = grid_row[0..col - 1].to_owned().into_iter().rev().collect();
        let west_seen = match west.iter().position(|&x| x >= tree_height) {
            Some(x) => x + 1,
            None => {
                if row == 0 {
                    0
                } else {
                    west.len()
                }
            }
        };

        north_seen * south_seen * east_seen * west_seen
    }
}

pub type Input = TreeGrid;

pub fn run(part: Part) -> Output {
    match part {
        Part::OneEx => {
            let exampleinput = input::readex();
            part1::solve(&exampleinput)
        }
        Part::One => {
            let input = input::read();
            part1::solve(&input)
        }
        Part::TwoEx => {
            let exampleinput = input::readex();
            part2::solve(&exampleinput)
        }
        Part::Two => {
            let input = input::read();
            part2::solve(&input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one_example() {
        let result = run(Part::OneEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two_example() {
        let result = run(Part::TwoEx);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
